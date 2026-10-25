#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxAgentData_ {
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
    avatar_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    default_language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_chat_engine_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multi_language_training: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_spell_correction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_playbook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_language_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    time_zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_AdvancedSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    answer_feedback_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_AnswerFeedbackSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_ClientCertificateSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gen_app_builder_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_GenAppBuilderSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_integration_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_GitIntegrationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    personalization_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_PersonalizationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speech_to_text_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_SpeechToTextSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_to_speech_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_TextToSpeechSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxAgent_Timeouts>,
    dynamic: DialogflowCxAgentDynamic,
}

struct DialogflowCxAgent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxAgentData_>,
}

#[derive(Clone)]
pub struct DialogflowCxAgent(std::rc::Rc<DialogflowCxAgent_>);

impl DialogflowCxAgent {
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
        "Set the field `avatar_uri`.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn set_avatar_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().avatar_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_chat_engine_on_destroy`.\nIf set to 'true', Terraform will delete the chat engine associated with the agent when the agent is destroyed.\nOtherwise, the chat engine will persist.\n\nThis virtual field addresses a critical dependency chain: 'agent' -> 'engine' -> 'data store'. The chat engine is automatically\nprovisioned when a data store is linked to the agent, meaning Terraform doesn't have direct control over its lifecycle as a managed\nresource. This creates a problem when both the agent and data store are managed by Terraform and need to be destroyed. Without\ndelete_chat_engine_on_destroy set to true, the data store's deletion would fail because the unmanaged chat engine would still be\nusing it. This setting ensures that the entire dependency chain can be properly torn down.\nSee 'mmv1/templates/terraform/examples/dialogflowcx_tool_data_store.tf.tmpl' as an example.\n\nData store can be linked to an agent through the 'knowledgeConnectorSettings' field of a [flow](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows#resource:-flow)\nor a [page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#resource:-page)\nor the 'dataStoreSpec' field of a [tool](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#resource:-tool).\nThe ID of the implicitly created engine is stored in the 'genAppBuilderSettings' field of the [agent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#resource:-agent)."]
    pub fn set_delete_chat_engine_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_chat_engine_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_multi_language_training`.\nEnable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent."]
    pub fn set_enable_multi_language_training(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_multi_language_training = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_spell_correction`.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn set_enable_spell_correction(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_spell_correction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_stackdriver_logging`.\nDetermines whether this agent should log conversation queries."]
    pub fn set_enable_stackdriver_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `locked`.\nIndicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for [agents.restore][]."]
    pub fn set_locked(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().locked = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_settings`.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn set_security_settings(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_playbook`.\nName of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: **projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/playbooks/<PlaybookID>**. Currently only the default playbook with id \"00000000-0000-0000-0000-000000000000\" is allowed."]
    pub fn set_start_playbook(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_playbook = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `supported_language_codes`.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn set_supported_language_codes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().supported_language_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `answer_feedback_settings`.\n"]
    pub fn set_answer_feedback_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_AnswerFeedbackSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().answer_feedback_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.answer_feedback_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_certificate_settings`.\n"]
    pub fn set_client_certificate_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_ClientCertificateSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gen_app_builder_settings`.\n"]
    pub fn set_gen_app_builder_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_GenAppBuilderSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gen_app_builder_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gen_app_builder_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git_integration_settings`.\n"]
    pub fn set_git_integration_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_GitIntegrationSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().git_integration_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.git_integration_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `personalization_settings`.\n"]
    pub fn set_personalization_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_PersonalizationSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().personalization_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.personalization_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `speech_to_text_settings`.\n"]
    pub fn set_speech_to_text_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_SpeechToTextSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().speech_to_text_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.speech_to_text_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_to_speech_settings`.\n"]
    pub fn set_text_to_speech_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_TextToSpeechSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().text_to_speech_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.text_to_speech_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxAgent_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn avatar_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.avatar_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_chat_engine_on_destroy` after provisioning.\nIf set to 'true', Terraform will delete the chat engine associated with the agent when the agent is destroyed.\nOtherwise, the chat engine will persist.\n\nThis virtual field addresses a critical dependency chain: 'agent' -> 'engine' -> 'data store'. The chat engine is automatically\nprovisioned when a data store is linked to the agent, meaning Terraform doesn't have direct control over its lifecycle as a managed\nresource. This creates a problem when both the agent and data store are managed by Terraform and need to be destroyed. Without\ndelete_chat_engine_on_destroy set to true, the data store's deletion would fail because the unmanaged chat engine would still be\nusing it. This setting ensures that the entire dependency chain can be properly torn down.\nSee 'mmv1/templates/terraform/examples/dialogflowcx_tool_data_store.tf.tmpl' as an example.\n\nData store can be linked to an agent through the 'knowledgeConnectorSettings' field of a [flow](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows#resource:-flow)\nor a [page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#resource:-page)\nor the 'dataStoreSpec' field of a [tool](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#resource:-tool).\nThe ID of the implicitly created engine is stored in the 'genAppBuilderSettings' field of the [agent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#resource:-agent)."]
    pub fn delete_chat_engine_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_chat_engine_on_destroy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the agent, unique within the location."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_multi_language_training` after provisioning.\nEnable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent."]
    pub fn enable_multi_language_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_language_training", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_spell_correction` after provisioning.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_spell_correction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locked` after provisioning.\nIndicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for [agents.restore][]."]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the agent."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzi` after provisioning.\nA read only boolean field reflecting Zone Isolation status of the agent."]
    pub fn satisfies_pzi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzi", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzs` after provisioning.\nA read only boolean field reflecting Zone Separation status of the agent."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_playbook` after provisioning.\nName of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: **projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/playbooks/<PlaybookID>**. Currently only the default playbook with id \"00000000-0000-0000-0000-000000000000\" is allowed."]
    pub fn start_playbook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_playbook", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn supported_language_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_language_codes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AdvancedSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `answer_feedback_settings` after provisioning.\n"]
    pub fn answer_feedback_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AnswerFeedbackSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.answer_feedback_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(&self) -> terrars::ListRef<DialogflowCxAgent_ClientCertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gen_app_builder_settings` after provisioning.\n"]
    pub fn gen_app_builder_settings(&self) -> terrars::ListRef<DialogflowCxAgent_GenAppBuilderSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gen_app_builder_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `git_integration_settings` after provisioning.\n"]
    pub fn git_integration_settings(&self) -> terrars::ListRef<DialogflowCxAgent_GitIntegrationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_integration_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `personalization_settings` after provisioning.\n"]
    pub fn personalization_settings(&self) -> terrars::ListRef<DialogflowCxAgent_PersonalizationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.personalization_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `speech_to_text_settings` after provisioning.\n"]
    pub fn speech_to_text_settings(&self) -> terrars::ListRef<DialogflowCxAgent_SpeechToTextSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.speech_to_text_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> terrars::ListRef<DialogflowCxAgent_TextToSpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_to_speech_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxAgent_TimeoutsRef {
        DialogflowCxAgent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxAgent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxAgent { }

impl terrars::ToListMappable for DialogflowCxAgent {
    type O = terrars::ListRef<DialogflowCxAgentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxAgent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_agent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxAgent {
    pub tf_id: std::string::String,
    #[doc =
        "The default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub default_language_code: terrars::PrimField<std::string::String>,
    #[doc = "The human-readable name of the agent, unique within the location."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxAgent {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxAgent {
        let out = DialogflowCxAgent(std::rc::Rc::new(DialogflowCxAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxAgentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                avatar_uri: core::default::Default::default(),
                default_language_code: self.default_language_code,
                delete_chat_engine_on_destroy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                enable_multi_language_training: core::default::Default::default(),
                enable_spell_correction: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                locked: core::default::Default::default(),
                project: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                start_playbook: core::default::Default::default(),
                supported_language_codes: core::default::Default::default(),
                time_zone: self.time_zone,
                advanced_settings: core::default::Default::default(),
                answer_feedback_settings: core::default::Default::default(),
                client_certificate_settings: core::default::Default::default(),
                gen_app_builder_settings: core::default::Default::default(),
                git_integration_settings: core::default::Default::default(),
                personalization_settings: core::default::Default::default(),
                speech_to_text_settings: core::default::Default::default(),
                text_to_speech_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxAgentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `avatar_uri` after provisioning.\nThe URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted Web Demo integration."]
    pub fn avatar_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.avatar_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language of the agent as a language tag. [See Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language)\nfor a list of the currently supported language codes. This field cannot be updated after creation."]
    pub fn default_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_chat_engine_on_destroy` after provisioning.\nIf set to 'true', Terraform will delete the chat engine associated with the agent when the agent is destroyed.\nOtherwise, the chat engine will persist.\n\nThis virtual field addresses a critical dependency chain: 'agent' -> 'engine' -> 'data store'. The chat engine is automatically\nprovisioned when a data store is linked to the agent, meaning Terraform doesn't have direct control over its lifecycle as a managed\nresource. This creates a problem when both the agent and data store are managed by Terraform and need to be destroyed. Without\ndelete_chat_engine_on_destroy set to true, the data store's deletion would fail because the unmanaged chat engine would still be\nusing it. This setting ensures that the entire dependency chain can be properly torn down.\nSee 'mmv1/templates/terraform/examples/dialogflowcx_tool_data_store.tf.tmpl' as an example.\n\nData store can be linked to an agent through the 'knowledgeConnectorSettings' field of a [flow](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows#resource:-flow)\nor a [page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#resource:-page)\nor the 'dataStoreSpec' field of a [tool](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#resource:-tool).\nThe ID of the implicitly created engine is stored in the 'genAppBuilderSettings' field of the [agent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#resource:-agent)."]
    pub fn delete_chat_engine_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_chat_engine_on_destroy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of this agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the agent, unique within the location."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_multi_language_training` after provisioning.\nEnable training multi-lingual models for this agent. These models will be trained on all the languages supported by the agent."]
    pub fn enable_multi_language_training(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_language_training", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_spell_correction` after provisioning.\nIndicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_spell_correction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDetermines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this agent is located in.\n\n~> **Note:** The first time you are deploying an Agent in your project you must configure location settings.\n This is a one time step but at the moment you can only [configure location settings](https://cloud.google.com/dialogflow/cx/docs/concept/region#location-settings) via the Dialogflow CX console.\n Another options is to use global location so you don't need to manually configure location settings."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locked` after provisioning.\nIndicates whether the agent is locked for changes. If the agent is locked, modifications to the agent will be rejected except for [agents.restore][]."]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the agent."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzi` after provisioning.\nA read only boolean field reflecting Zone Isolation status of the agent."]
    pub fn satisfies_pzi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzi", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `satisfies_pzs` after provisioning.\nA read only boolean field reflecting Zone Separation status of the agent."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_playbook` after provisioning.\nName of the start playbook in this agent. A start playbook will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: **projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/playbooks/<PlaybookID>**. Currently only the default playbook with id \"00000000-0000-0000-0000-000000000000\" is allowed."]
    pub fn start_playbook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_playbook", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_language_codes` after provisioning.\nThe list of all languages supported by this agent (except for the default_language_code)."]
    pub fn supported_language_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_language_codes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York,\nEurope/Paris."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AdvancedSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `answer_feedback_settings` after provisioning.\n"]
    pub fn answer_feedback_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AnswerFeedbackSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.answer_feedback_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(&self) -> terrars::ListRef<DialogflowCxAgent_ClientCertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gen_app_builder_settings` after provisioning.\n"]
    pub fn gen_app_builder_settings(&self) -> terrars::ListRef<DialogflowCxAgent_GenAppBuilderSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gen_app_builder_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `git_integration_settings` after provisioning.\n"]
    pub fn git_integration_settings(&self) -> terrars::ListRef<DialogflowCxAgent_GitIntegrationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_integration_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `personalization_settings` after provisioning.\n"]
    pub fn personalization_settings(&self) -> terrars::ListRef<DialogflowCxAgent_PersonalizationSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.personalization_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `speech_to_text_settings` after provisioning.\n"]
    pub fn speech_to_text_settings(&self) -> terrars::ListRef<DialogflowCxAgent_SpeechToTextSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.speech_to_text_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> terrars::ListRef<DialogflowCxAgent_TextToSpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_to_speech_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxAgent_TimeoutsRef {
        DialogflowCxAgent_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxAgentDynamic {
    advanced_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_AdvancedSettings>>,
    answer_feedback_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_AnswerFeedbackSettings>>,
    client_certificate_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxAgent_ClientCertificateSettings>>,
    gen_app_builder_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_GenAppBuilderSettings>>,
    git_integration_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_GitIntegrationSettings>>,
    personalization_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_PersonalizationSettings>>,
    speech_to_text_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_SpeechToTextSettings>>,
    text_to_speech_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_TextToSpeechSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination {
    #[doc =
        "Set the field `uri`.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination {
    type O = terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination {}

impl BuildDialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination {
    pub fn build(self) -> DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination {
        DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestinationRef {
        DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_AdvancedSettings_DtmfSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_digits: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxAgent_AdvancedSettings_DtmfSettings {
    #[doc =
        "Set the field `enabled`.\nIf true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finish_digit`.\nThe digit that terminates a DTMF digit sequence."]
    pub fn set_finish_digit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finish_digit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_digits`.\nMax length of DTMF digits."]
    pub fn set_max_digits(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_digits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_AdvancedSettings_DtmfSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_DtmfSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_AdvancedSettings_DtmfSettings {}

impl BuildDialogflowCxAgent_AdvancedSettings_DtmfSettings {
    pub fn build(self) -> DialogflowCxAgent_AdvancedSettings_DtmfSettings {
        DialogflowCxAgent_AdvancedSettings_DtmfSettings {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_AdvancedSettings_DtmfSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_AdvancedSettings_DtmfSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxAgent_AdvancedSettings_DtmfSettingsRef {
        DialogflowCxAgent_AdvancedSettings_DtmfSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_AdvancedSettings_DtmfSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf true, incoming audio is processed for DTMF (dual tone multi frequency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will detect the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `finish_digit` after provisioning.\nThe digit that terminates a DTMF digit sequence."]
    pub fn finish_digit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }

    #[doc = "Get a reference to the value of field `max_digits` after provisioning.\nMax length of DTMF digits."]
    pub fn max_digits(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_digits", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_AdvancedSettings_LoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_consent_based_redaction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_interaction_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_logging: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxAgent_AdvancedSettings_LoggingSettings {
    #[doc =
        "Set the field `enable_consent_based_redaction`.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."]
    pub fn set_enable_consent_based_redaction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_consent_based_redaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_interaction_logging`.\nEnables DF Interaction logging."]
    pub fn set_enable_interaction_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_interaction_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_stackdriver_logging`.\nEnables Google Cloud Logging."]
    pub fn set_enable_stackdriver_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_stackdriver_logging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_AdvancedSettings_LoggingSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_LoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_AdvancedSettings_LoggingSettings {}

impl BuildDialogflowCxAgent_AdvancedSettings_LoggingSettings {
    pub fn build(self) -> DialogflowCxAgent_AdvancedSettings_LoggingSettings {
        DialogflowCxAgent_AdvancedSettings_LoggingSettings {
            enable_consent_based_redaction: core::default::Default::default(),
            enable_interaction_logging: core::default::Default::default(),
            enable_stackdriver_logging: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_AdvancedSettings_LoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_AdvancedSettings_LoggingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxAgent_AdvancedSettings_LoggingSettingsRef {
        DialogflowCxAgent_AdvancedSettings_LoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_AdvancedSettings_LoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_consent_based_redaction` after provisioning.\nEnables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted."]
    pub fn enable_consent_based_redaction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_consent_based_redaction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_interaction_logging` after provisioning.\nEnables DF Interaction logging."]
    pub fn enable_interaction_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_interaction_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nEnables Google Cloud Logging."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_AdvancedSettings_SpeechSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpointer_sensitivity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    models: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_speech_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_timeout_based_endpointing: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxAgent_AdvancedSettings_SpeechSettings {
    #[doc =
        "Set the field `endpointer_sensitivity`.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."]
    pub fn set_endpointer_sensitivity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.endpointer_sensitivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `models`.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."]
    pub fn set_models(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.models = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_speech_timeout`.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_no_speech_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.no_speech_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_timeout_based_endpointing`.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."]
    pub fn set_use_timeout_based_endpointing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_timeout_based_endpointing = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_AdvancedSettings_SpeechSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_SpeechSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_AdvancedSettings_SpeechSettings {}

impl BuildDialogflowCxAgent_AdvancedSettings_SpeechSettings {
    pub fn build(self) -> DialogflowCxAgent_AdvancedSettings_SpeechSettings {
        DialogflowCxAgent_AdvancedSettings_SpeechSettings {
            endpointer_sensitivity: core::default::Default::default(),
            models: core::default::Default::default(),
            no_speech_timeout: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_AdvancedSettings_SpeechSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_AdvancedSettings_SpeechSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxAgent_AdvancedSettings_SpeechSettingsRef {
        DialogflowCxAgent_AdvancedSettings_SpeechSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_AdvancedSettings_SpeechSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpointer_sensitivity` after provisioning.\nSensitivity of the speech model that detects the end of speech. Scale from 0 to 100."]
    pub fn endpointer_sensitivity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpointer_sensitivity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `models` after provisioning.\nMapping from language to Speech-to-Text model. The mapped Speech-to-Text model will be selected for requests from its corresponding language. For more information, see [Speech models](https://cloud.google.com/dialogflow/cx/docs/concept/speech-models).\nAn object containing a list of **\"key\": value** pairs. Example: **{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }**."]
    pub fn models(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.models", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_speech_timeout` after provisioning.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn no_speech_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_speech_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_timeout_based_endpointing` after provisioning.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."]
    pub fn use_timeout_based_endpointing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_timeout_based_endpointing", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxAgent_AdvancedSettingsDynamic {
    audio_export_gcs_destination: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination>>,
    dtmf_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_AdvancedSettings_DtmfSettings>>,
    logging_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_AdvancedSettings_LoggingSettings>>,
    speech_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxAgent_AdvancedSettings_SpeechSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_AdvancedSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_export_gcs_destination: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_AdvancedSettings_DtmfSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_AdvancedSettings_LoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speech_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_AdvancedSettings_SpeechSettings>>,
    dynamic: DialogflowCxAgent_AdvancedSettingsDynamic,
}

impl DialogflowCxAgent_AdvancedSettings {
    #[doc = "Set the field `audio_export_gcs_destination`.\n"]
    pub fn set_audio_export_gcs_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_export_gcs_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_export_gcs_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_DtmfSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dtmf_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dtmf_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_LoggingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `speech_settings`.\n"]
    pub fn set_speech_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings_SpeechSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.speech_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.speech_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_AdvancedSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_AdvancedSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_AdvancedSettings {}

impl BuildDialogflowCxAgent_AdvancedSettings {
    pub fn build(self) -> DialogflowCxAgent_AdvancedSettings {
        DialogflowCxAgent_AdvancedSettings {
            audio_export_gcs_destination: core::default::Default::default(),
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_AdvancedSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_AdvancedSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_AdvancedSettingsRef {
        DialogflowCxAgent_AdvancedSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_AdvancedSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_export_gcs_destination` after provisioning.\n"]
    pub fn audio_export_gcs_destination(
        &self,
    ) -> terrars::ListRef<DialogflowCxAgent_AdvancedSettings_AudioExportGcsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_export_gcs_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AdvancedSettings_DtmfSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AdvancedSettings_LoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]
    pub fn speech_settings(&self) -> terrars::ListRef<DialogflowCxAgent_AdvancedSettings_SpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.speech_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_AnswerFeedbackSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_answer_feedback: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxAgent_AnswerFeedbackSettings {
    #[doc =
        "Set the field `enable_answer_feedback`.\nIf enabled, end users will be able to provide [answer feedback](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/submitAnswerFeedback#body.AnswerFeedback)\nto Dialogflow responses. Feature works only if interaction logging is enabled in the Dialogflow agent."]
    pub fn set_enable_answer_feedback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_answer_feedback = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_AnswerFeedbackSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_AnswerFeedbackSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_AnswerFeedbackSettings {}

impl BuildDialogflowCxAgent_AnswerFeedbackSettings {
    pub fn build(self) -> DialogflowCxAgent_AnswerFeedbackSettings {
        DialogflowCxAgent_AnswerFeedbackSettings { enable_answer_feedback: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_AnswerFeedbackSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_AnswerFeedbackSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_AnswerFeedbackSettingsRef {
        DialogflowCxAgent_AnswerFeedbackSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_AnswerFeedbackSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_answer_feedback` after provisioning.\nIf enabled, end users will be able to provide [answer feedback](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/submitAnswerFeedback#body.AnswerFeedback)\nto Dialogflow responses. Feature works only if interaction logging is enabled in the Dialogflow agent."]
    pub fn enable_answer_feedback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_answer_feedback", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_ClientCertificateSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    private_key: terrars::PrimField<std::string::String>,
    ssl_certificate: terrars::PrimField<std::string::String>,
}

impl DialogflowCxAgent_ClientCertificateSettings {
    #[doc =
        "Set the field `passphrase`.\nThe name of the SecretManager secret version resource storing the passphrase. 'passphrase' should be left unset if the private key is not encrypted. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub fn set_passphrase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.passphrase = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_ClientCertificateSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_ClientCertificateSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_ClientCertificateSettings {
    #[doc =
        "The name of the SecretManager secret version resource storing the private key encoded in PEM format. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub private_key: terrars::PrimField<std::string::String>,
    #[doc =
        "The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub ssl_certificate: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxAgent_ClientCertificateSettings {
    pub fn build(self) -> DialogflowCxAgent_ClientCertificateSettings {
        DialogflowCxAgent_ClientCertificateSettings {
            passphrase: core::default::Default::default(),
            private_key: self.private_key,
            ssl_certificate: self.ssl_certificate,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_ClientCertificateSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_ClientCertificateSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxAgent_ClientCertificateSettingsRef {
        DialogflowCxAgent_ClientCertificateSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_ClientCertificateSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `passphrase` after provisioning.\nThe name of the SecretManager secret version resource storing the passphrase. 'passphrase' should be left unset if the private key is not encrypted. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key` after provisioning.\nThe name of the SecretManager secret version resource storing the private key encoded in PEM format. Format: **projects/{project}/secrets/{secret}/versions/{version}**"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_certificate` after provisioning.\nThe ssl certificate encoded in PEM format. This string must include the begin header and end footer lines."]
    pub fn ssl_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_GenAppBuilderSettings {
    engine: terrars::PrimField<std::string::String>,
}

impl DialogflowCxAgent_GenAppBuilderSettings { }

impl terrars::ToListMappable for DialogflowCxAgent_GenAppBuilderSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_GenAppBuilderSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_GenAppBuilderSettings {
    #[doc =
        "The full name of the Gen App Builder engine related to this agent if there is one.\nFormat: projects/{Project ID}/locations/{Location ID}/collections/{Collection ID}/engines/{Engine ID}"]
    pub engine: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxAgent_GenAppBuilderSettings {
    pub fn build(self) -> DialogflowCxAgent_GenAppBuilderSettings {
        DialogflowCxAgent_GenAppBuilderSettings { engine: self.engine }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_GenAppBuilderSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_GenAppBuilderSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_GenAppBuilderSettingsRef {
        DialogflowCxAgent_GenAppBuilderSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_GenAppBuilderSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `engine` after provisioning.\nThe full name of the Gen App Builder engine related to this agent if there is one.\nFormat: projects/{Project ID}/locations/{Location ID}/collections/{Collection ID}/engines/{Engine ID}"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_GitIntegrationSettings_GithubSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branches: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracking_branch: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxAgent_GitIntegrationSettings_GithubSettings {
    #[doc =
        "Set the field `access_token`.\nThe access token used to authenticate the access to the GitHub repository."]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `branches`.\nA list of branches configured to be used from Dialogflow."]
    pub fn set_branches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.branches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe unique repository display name for the GitHub repository."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository_uri`.\nThe GitHub repository URI related to the agent."]
    pub fn set_repository_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tracking_branch`.\nThe branch of the GitHub repository tracked for this agent."]
    pub fn set_tracking_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tracking_branch = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_GitIntegrationSettings_GithubSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_GitIntegrationSettings_GithubSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_GitIntegrationSettings_GithubSettings {}

impl BuildDialogflowCxAgent_GitIntegrationSettings_GithubSettings {
    pub fn build(self) -> DialogflowCxAgent_GitIntegrationSettings_GithubSettings {
        DialogflowCxAgent_GitIntegrationSettings_GithubSettings {
            access_token: core::default::Default::default(),
            branches: core::default::Default::default(),
            display_name: core::default::Default::default(),
            repository_uri: core::default::Default::default(),
            tracking_branch: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_GitIntegrationSettings_GithubSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_GitIntegrationSettings_GithubSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxAgent_GitIntegrationSettings_GithubSettingsRef {
        DialogflowCxAgent_GitIntegrationSettings_GithubSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_GitIntegrationSettings_GithubSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_token` after provisioning.\nThe access token used to authenticate the access to the GitHub repository."]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `branches` after provisioning.\nA list of branches configured to be used from Dialogflow."]
    pub fn branches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.branches", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe unique repository display name for the GitHub repository."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository_uri` after provisioning.\nThe GitHub repository URI related to the agent."]
    pub fn repository_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tracking_branch` after provisioning.\nThe branch of the GitHub repository tracked for this agent."]
    pub fn tracking_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tracking_branch", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxAgent_GitIntegrationSettingsDynamic {
    github_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxAgent_GitIntegrationSettings_GithubSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_GitIntegrationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github_settings: std::option::Option<std::vec::Vec<DialogflowCxAgent_GitIntegrationSettings_GithubSettings>>,
    dynamic: DialogflowCxAgent_GitIntegrationSettingsDynamic,
}

impl DialogflowCxAgent_GitIntegrationSettings {
    #[doc = "Set the field `github_settings`.\n"]
    pub fn set_github_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxAgent_GitIntegrationSettings_GithubSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_GitIntegrationSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_GitIntegrationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_GitIntegrationSettings {}

impl BuildDialogflowCxAgent_GitIntegrationSettings {
    pub fn build(self) -> DialogflowCxAgent_GitIntegrationSettings {
        DialogflowCxAgent_GitIntegrationSettings {
            github_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_GitIntegrationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_GitIntegrationSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_GitIntegrationSettingsRef {
        DialogflowCxAgent_GitIntegrationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_GitIntegrationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `github_settings` after provisioning.\n"]
    pub fn github_settings(&self) -> terrars::ListRef<DialogflowCxAgent_GitIntegrationSettings_GithubSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_PersonalizationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_end_user_metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxAgent_PersonalizationSettings {
    #[doc =
        "Set the field `default_end_user_metadata`.\nDefault end user metadata, used when processing DetectIntent requests. Recommended to be filled as a template instead of hard-coded value, for example { \"age\": \"$session.params.age\" }.\nThe data will be merged with the [QueryParameters.end_user_metadata](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/QueryParameters#FIELDS.end_user_metadata)\nin [DetectIntentRequest.query_params](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/detectIntent#body.request_body.FIELDS.query_params) during query processing.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct)."]
    pub fn set_default_end_user_metadata(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_end_user_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_PersonalizationSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_PersonalizationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_PersonalizationSettings {}

impl BuildDialogflowCxAgent_PersonalizationSettings {
    pub fn build(self) -> DialogflowCxAgent_PersonalizationSettings {
        DialogflowCxAgent_PersonalizationSettings { default_end_user_metadata: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_PersonalizationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_PersonalizationSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_PersonalizationSettingsRef {
        DialogflowCxAgent_PersonalizationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_PersonalizationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_end_user_metadata` after provisioning.\nDefault end user metadata, used when processing DetectIntent requests. Recommended to be filled as a template instead of hard-coded value, for example { \"age\": \"$session.params.age\" }.\nThe data will be merged with the [QueryParameters.end_user_metadata](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/QueryParameters#FIELDS.end_user_metadata)\nin [DetectIntentRequest.query_params](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.sessions/detectIntent#body.request_body.FIELDS.query_params) during query processing.\n\nThis field uses JSON data as a string. The value provided must be a valid JSON representation documented in [Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct)."]
    pub fn default_end_user_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_end_user_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_SpeechToTextSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_speech_adaptation: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxAgent_SpeechToTextSettings {
    #[doc = "Set the field `enable_speech_adaptation`.\nWhether to use speech adaptation for speech recognition."]
    pub fn set_enable_speech_adaptation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_speech_adaptation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_SpeechToTextSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_SpeechToTextSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_SpeechToTextSettings {}

impl BuildDialogflowCxAgent_SpeechToTextSettings {
    pub fn build(self) -> DialogflowCxAgent_SpeechToTextSettings {
        DialogflowCxAgent_SpeechToTextSettings { enable_speech_adaptation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_SpeechToTextSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_SpeechToTextSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_SpeechToTextSettingsRef {
        DialogflowCxAgent_SpeechToTextSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_SpeechToTextSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_speech_adaptation` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn enable_speech_adaptation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_speech_adaptation", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_TextToSpeechSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synthesize_speech_configs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxAgent_TextToSpeechSettings {
    #[doc =
        "Set the field `synthesize_speech_configs`.\nConfiguration of how speech should be synthesized, mapping from [language](https://cloud.google.com/dialogflow/cx/docs/reference/language) to [SynthesizeSpeechConfig](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#synthesizespeechconfig).\nThese settings affect:\n* The phone gateway synthesize configuration set via Agent.text_to_speech_settings.\n* How speech is synthesized when invoking session APIs. 'Agent.text_to_speech_settings' only applies if 'OutputAudioConfig.synthesize_speech_config' is not specified."]
    pub fn set_synthesize_speech_configs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.synthesize_speech_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxAgent_TextToSpeechSettings {
    type O = terrars::BlockAssignable<DialogflowCxAgent_TextToSpeechSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_TextToSpeechSettings {}

impl BuildDialogflowCxAgent_TextToSpeechSettings {
    pub fn build(self) -> DialogflowCxAgent_TextToSpeechSettings {
        DialogflowCxAgent_TextToSpeechSettings { synthesize_speech_configs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_TextToSpeechSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_TextToSpeechSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_TextToSpeechSettingsRef {
        DialogflowCxAgent_TextToSpeechSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_TextToSpeechSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `synthesize_speech_configs` after provisioning.\nConfiguration of how speech should be synthesized, mapping from [language](https://cloud.google.com/dialogflow/cx/docs/reference/language) to [SynthesizeSpeechConfig](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents#synthesizespeechconfig).\nThese settings affect:\n* The phone gateway synthesize configuration set via Agent.text_to_speech_settings.\n* How speech is synthesized when invoking session APIs. 'Agent.text_to_speech_settings' only applies if 'OutputAudioConfig.synthesize_speech_config' is not specified."]
    pub fn synthesize_speech_configs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.synthesize_speech_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxAgent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxAgent_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxAgent_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxAgent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxAgent_Timeouts {}

impl BuildDialogflowCxAgent_Timeouts {
    pub fn build(self) -> DialogflowCxAgent_Timeouts {
        DialogflowCxAgent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxAgent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxAgent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxAgent_TimeoutsRef {
        DialogflowCxAgent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxAgent_TimeoutsRef {
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
