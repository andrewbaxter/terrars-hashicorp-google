#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowConversationProfileData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automated_agent_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_AutomatedAgentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_agent_assistant_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_HumanAgentAssistantConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_agent_handoff_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_HumanAgentHandoffConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_message_event_notification_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_NewMessageEventNotificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_recognition_result_notification_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_NewRecognitionResultNotificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_NotificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stt_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_SttConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowConversationProfile_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tts_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_TtsConfig>>,
    dynamic: DialogflowConversationProfileDynamic,
}

struct DialogflowConversationProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowConversationProfileData_>,
}

#[derive(Clone)]
pub struct DialogflowConversationProfile(std::rc::Rc<DialogflowConversationProfile_>);

impl DialogflowConversationProfile {
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
        "Set the field `language_code`.\nLanguage code for the conversation profile. This should be a BCP-47 language tag."]
    pub fn set_language_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_settings`.\nName of the CX SecuritySettings reference for the agent."]
    pub fn set_security_settings(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\nThe time zone of this conversational profile."]
    pub fn set_time_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automated_agent_config`.\n"]
    pub fn set_automated_agent_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_AutomatedAgentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automated_agent_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automated_agent_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human_agent_assistant_config`.\n"]
    pub fn set_human_agent_assistant_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentAssistantConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_agent_assistant_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_agent_assistant_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human_agent_handoff_config`.\n"]
    pub fn set_human_agent_handoff_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentHandoffConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_agent_handoff_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_agent_handoff_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_message_event_notification_config`.\n"]
    pub fn set_new_message_event_notification_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowConversationProfile_NewMessageEventNotificationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().new_message_event_notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.new_message_event_notification_config =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_recognition_result_notification_config`.\n"]
    pub fn set_new_recognition_result_notification_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowConversationProfile_NewRecognitionResultNotificationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().new_recognition_result_notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.new_recognition_result_notification_config =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_NotificationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stt_config`.\n"]
    pub fn set_stt_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_SttConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stt_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stt_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowConversationProfile_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tts_config`.\n"]
    pub fn set_tts_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_TtsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tts_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tts_config = std::option::Option::Some(d);
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
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. Human readable name for this profile. Max length 1024 bytes."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for the conversation profile. This should be a BCP-47 language tag."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the conversation profile."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of this conversation profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nName of the CX SecuritySettings reference for the agent."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this conversational profile."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_agent_config` after provisioning.\n"]
    pub fn automated_agent_config(&self) -> terrars::ListRef<DialogflowConversationProfile_AutomatedAgentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_agent_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_agent_assistant_config` after provisioning.\n"]
    pub fn human_agent_assistant_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_agent_assistant_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_agent_handoff_config` after provisioning.\n"]
    pub fn human_agent_handoff_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentHandoffConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_agent_handoff_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DialogflowConversationProfile_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `new_message_event_notification_config` after provisioning.\n"]
    pub fn new_message_event_notification_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_NewMessageEventNotificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.new_message_event_notification_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `new_recognition_result_notification_config` after provisioning.\n"]
    pub fn new_recognition_result_notification_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.new_recognition_result_notification_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<DialogflowConversationProfile_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stt_config` after provisioning.\n"]
    pub fn stt_config(&self) -> terrars::ListRef<DialogflowConversationProfile_SttConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stt_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowConversationProfile_TimeoutsRef {
        DialogflowConversationProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tts_config` after provisioning.\n"]
    pub fn tts_config(&self) -> terrars::ListRef<DialogflowConversationProfile_TtsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tts_config", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowConversationProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowConversationProfile { }

impl terrars::ToListMappable for DialogflowConversationProfile {
    type O = terrars::ListRef<DialogflowConversationProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowConversationProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_conversation_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowConversationProfile {
    pub tf_id: std::string::String,
    #[doc = "Required. Human readable name for this profile. Max length 1024 bytes."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location of the conversation profile."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowConversationProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowConversationProfile {
        let out = DialogflowConversationProfile(std::rc::Rc::new(DialogflowConversationProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowConversationProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                language_code: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                time_zone: core::default::Default::default(),
                automated_agent_config: core::default::Default::default(),
                human_agent_assistant_config: core::default::Default::default(),
                human_agent_handoff_config: core::default::Default::default(),
                logging_config: core::default::Default::default(),
                new_message_event_notification_config: core::default::Default::default(),
                new_recognition_result_notification_config: core::default::Default::default(),
                notification_config: core::default::Default::default(),
                stt_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tts_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowConversationProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowConversationProfileRef {
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
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. Human readable name for this profile. Max length 1024 bytes."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for the conversation profile. This should be a BCP-47 language tag."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the conversation profile."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of this conversation profile."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nName of the CX SecuritySettings reference for the agent."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of this conversational profile."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_agent_config` after provisioning.\n"]
    pub fn automated_agent_config(&self) -> terrars::ListRef<DialogflowConversationProfile_AutomatedAgentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automated_agent_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_agent_assistant_config` after provisioning.\n"]
    pub fn human_agent_assistant_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_agent_assistant_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_agent_handoff_config` after provisioning.\n"]
    pub fn human_agent_handoff_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentHandoffConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_agent_handoff_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DialogflowConversationProfile_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `new_message_event_notification_config` after provisioning.\n"]
    pub fn new_message_event_notification_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_NewMessageEventNotificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.new_message_event_notification_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `new_recognition_result_notification_config` after provisioning.\n"]
    pub fn new_recognition_result_notification_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.new_recognition_result_notification_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<DialogflowConversationProfile_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stt_config` after provisioning.\n"]
    pub fn stt_config(&self) -> terrars::ListRef<DialogflowConversationProfile_SttConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stt_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowConversationProfile_TimeoutsRef {
        DialogflowConversationProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tts_config` after provisioning.\n"]
    pub fn tts_config(&self) -> terrars::ListRef<DialogflowConversationProfile_TtsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tts_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfileDynamic {
    automated_agent_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_AutomatedAgentConfig>>,
    human_agent_assistant_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_HumanAgentAssistantConfig>>,
    human_agent_handoff_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_HumanAgentHandoffConfig>>,
    logging_config: std::option::Option<terrars::DynamicBlock<DialogflowConversationProfile_LoggingConfig>>,
    new_message_event_notification_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_NewMessageEventNotificationConfig>>,
    new_recognition_result_notification_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_NewRecognitionResultNotificationConfig>>,
    notification_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_NotificationConfig>>,
    stt_config: std::option::Option<terrars::DynamicBlock<DialogflowConversationProfile_SttConfig>>,
    tts_config: std::option::Option<terrars::DynamicBlock<DialogflowConversationProfile_TtsConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_AutomatedAgentConfig {
    agent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_AutomatedAgentConfig {
    #[doc = "Set the field `session_ttl`.\nConfigure lifetime of the Dialogflow session."]
    pub fn set_session_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_AutomatedAgentConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_AutomatedAgentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_AutomatedAgentConfig {
    #[doc =
        "ID of the Dialogflow agent environment to use.\nExpects the format \"projects/<Project ID>/locations/<Location ID>/agent/environments/<EnvironmentID>\""]
    pub agent: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowConversationProfile_AutomatedAgentConfig {
    pub fn build(self) -> DialogflowConversationProfile_AutomatedAgentConfig {
        DialogflowConversationProfile_AutomatedAgentConfig {
            agent: self.agent,
            session_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_AutomatedAgentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_AutomatedAgentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_AutomatedAgentConfigRef {
        DialogflowConversationProfile_AutomatedAgentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_AutomatedAgentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nID of the Dialogflow agent environment to use.\nExpects the format \"projects/<Project ID>/locations/<Location ID>/agent/environments/<EnvironmentID>\""]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_ttl` after provisioning.\nConfigure lifetime of the Dialogflow session."]
    pub fn session_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_model_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {
    #[doc =
        "Set the field `baseline_model_version`.\nVersion of current baseline model. It will be ignored if model is set. Valid versions are: Article Suggestion baseline model: - 0.9 - 1.0 (default) Summarization baseline model: - 1.0"]
    pub fn set_baseline_model_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.baseline_model_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model`.\nConversation model resource name. Format: projects/<Project ID>/conversationModels/<Model ID>."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig {
            baseline_model_version: core::default::Default::default(),
            model: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `baseline_model_version` after provisioning.\nVersion of current baseline model. It will be ignored if model is set. Valid versions are: Article Suggestion baseline model: - 0.9 - 1.0 (default) Summarization baseline model: - 1.0"]
    pub fn baseline_model_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline_model_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nConversation model resource name. Format: projects/<Project ID>/conversationModels/<Model ID>."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recent_sentences_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {
    #[doc =
        "Set the field `recent_sentences_count`.\nNumber of recent non-small-talk sentences to use as context for article and FAQ suggestion"]
    pub fn set_recent_sentences_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.recent_sentences_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig {
            recent_sentences_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `recent_sentences_count` after provisioning.\nNumber of recent non-small-talk sentences to use as context for article and FAQ suggestion"]
    pub fn recent_sentences_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recent_sentences_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_handoff_messages: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_ivr_messages: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_virtual_agent_messages: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {
    #[doc =
        "Set the field `drop_handoff_messages`.\nIf set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped."]
    pub fn set_drop_handoff_messages(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_handoff_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drop_ivr_messages`.\nIf set to true, all messages from ivr stage are dropped."]
    pub fn set_drop_ivr_messages(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_ivr_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `drop_virtual_agent_messages`.\nIf set to true, all messages from virtual agent are dropped."]
    pub fn set_drop_virtual_agent_messages(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_virtual_agent_messages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings {
            drop_handoff_messages: core::default::Default::default(),
            drop_ivr_messages: core::default::Default::default(),
            drop_virtual_agent_messages: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drop_handoff_messages` after provisioning.\nIf set to true, the last message from virtual agent (hand off message) and the message before it (trigger message of hand off) are dropped."]
    pub fn drop_handoff_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_handoff_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `drop_ivr_messages` after provisioning.\nIf set to true, all messages from ivr stage are dropped."]
    pub fn drop_ivr_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_ivr_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `drop_virtual_agent_messages` after provisioning.\nIf set to true, all messages from virtual agent are dropped."]
    pub fn drop_virtual_agent_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_virtual_agent_messages", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {
    #[doc =
        "Set the field `agent`.\nThe name of a dialogflow virtual agent used for intent detection and suggestion triggered by human agent. Format: projects/<Project ID>/locations/<Location ID>/agent."]
    pub fn set_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig {
            agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nThe name of a dialogflow virtual agent used for intent detection and suggestion triggered by human agent. Format: projects/<Project ID>/locations/<Location ID>/agent."]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceDynamic {
    human_agent_side_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
    agent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_agent_side_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig,
        >,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
    #[doc = "Set the field `human_agent_side_config`.\n"]
    pub fn set_human_agent_side_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.human_agent_side_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.human_agent_side_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
    #[doc =
        "The name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: projects/<Project ID>/locations/<Location ID>/agent."]
    pub agent: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource {
            agent: self.agent,
            human_agent_side_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nThe name of a Dialogflow virtual agent used for end user side intent detection and suggestion. Format: projects/<Project ID>/locations/<Location ID>/agent."]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc = "Get a reference to the value of field `human_agent_side_config` after provisioning.\n"]
    pub fn human_agent_side_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource_HumanAgentSideConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_agent_side_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {
    documents: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {

}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {
    #[doc =
        "Knowledge documents to query from. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<KnowledgeBase ID>/documents/<Document ID>."]
    pub documents: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource {
            documents: self.documents,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySourceRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `documents` after provisioning.\nKnowledge documents to query from. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<KnowledgeBase ID>/documents/<Document ID>."]
    pub fn documents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documents", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {
    knowledge_bases: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {

}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {
    #[doc =
        "Knowledge bases to query. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<Knowledge Base ID>."]
    pub knowledge_bases: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource {
            knowledge_bases: self.knowledge_bases,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySourceRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `knowledge_bases` after provisioning.\nKnowledge bases to query. Format: projects/<Project ID>/locations/<Location ID>/knowledgeBases/<Knowledge Base ID>."]
    pub fn knowledge_bases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_bases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    section_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {
    #[doc =
        "Set the field `section_types`.\nThe selected sections chosen to return when requesting a summary of a conversation\nIf not provided the default selection will be \"{SITUATION, ACTION, RESULT}\". Possible values: [\"SECTION_TYPE_UNSPECIFIED\", \"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\"]"]
    pub fn set_section_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.section_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections {
            section_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `section_types` after provisioning.\nThe selected sections chosen to return when requesting a summary of a conversation\nIf not provided the default selection will be \"{SITUATION, ACTION, RESULT}\". Possible values: [\"SECTION_TYPE_UNSPECIFIED\", \"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\"]"]
    pub fn section_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.section_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigDynamic {
    context_filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
        >,
    >,
    dialogflow_query_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
        >,
    >,
    document_query_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource,
        >,
    >,
    knowledge_base_query_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource,
        >,
    >,
    sections: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_results: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_filter_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialogflow_query_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_query_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base_query_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sections: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
        >,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {
    #[doc =
        "Set the field `confidence_threshold`.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."]
    pub fn set_confidence_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.confidence_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_results`.\nMaximum number of results to return."]
    pub fn set_max_results(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `context_filter_settings`.\n"]
    pub fn set_context_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.context_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.context_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dialogflow_query_source`.\n"]
    pub fn set_dialogflow_query_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialogflow_query_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialogflow_query_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_query_source`.\n"]
    pub fn set_document_query_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.document_query_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.document_query_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `knowledge_base_query_source`.\n"]
    pub fn set_knowledge_base_query_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knowledge_base_query_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_base_query_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sections`.\n"]
    pub fn set_sections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig {
            confidence_threshold: core::default::Default::default(),
            max_results: core::default::Default::default(),
            context_filter_settings: core::default::Default::default(),
            dialogflow_query_source: core::default::Default::default(),
            document_query_source: core::default::Default::default(),
            knowledge_base_query_source: core::default::Default::default(),
            sections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_threshold` after provisioning.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_results` after provisioning.\nMaximum number of results to return."]
    pub fn max_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_results", self.base))
    }

    #[doc = "Get a reference to the value of field `context_filter_settings` after provisioning.\n"]
    pub fn context_filter_settings(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.context_filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dialogflow_query_source` after provisioning.\n"]
    pub fn dialogflow_query_source(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialogflow_query_source", self.base))
    }

    #[doc = "Get a reference to the value of field `document_query_source` after provisioning.\n"]
    pub fn document_query_source(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DocumentQuerySourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_query_source", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_query_source` after provisioning.\n"]
    pub fn knowledge_base_query_source(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_KnowledgeBaseQuerySourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_base_query_source", self.base))
    }

    #[doc = "Get a reference to the value of field `sections` after provisioning.\n"]
    pub fn sections(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sections", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {
    #[doc = "Set the field `type_`.\nType of Human Agent Assistant API feature to request."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature {
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Human Agent Assistant API feature to request."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_small_talk: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    only_end_user: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {
    #[doc = "Set the field `no_small_talk`.\nDo not trigger if last utterance is small talk."]
    pub fn set_no_small_talk(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_small_talk = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `only_end_user`.\nOnly trigger suggestion if participant role of last utterance is END_USER."]
    pub fn set_only_end_user(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.only_end_user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings {
            no_small_talk: core::default::Default::default(),
            only_end_user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `no_small_talk` after provisioning.\nDo not trigger if last utterance is small talk."]
    pub fn no_small_talk(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_small_talk", self.base))
    }

    #[doc =
        "Get a reference to the value of field `only_end_user` after provisioning.\nOnly trigger suggestion if participant role of last utterance is END_USER."]
    pub fn only_end_user(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_end_user", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsDynamic {
    conversation_model_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
        >,
    >,
    conversation_process_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
        >,
    >,
    query_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig,
        >,
    >,
    suggestion_feature: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
        >,
    >,
    suggestion_trigger_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_agent_query_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_conversation_augmented_query: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_event_based_suggestion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_query_suggestion_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_query_suggestion_when_no_answer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_model_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_process_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggestion_feature: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggestion_trigger_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
        >,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {
    #[doc =
        "Set the field `disable_agent_query_logging`.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn set_disable_agent_query_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_agent_query_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_conversation_augmented_query`.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn set_enable_conversation_augmented_query(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_conversation_augmented_query = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_event_based_suggestion`.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."]
    pub fn set_enable_event_based_suggestion(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_event_based_suggestion = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_query_suggestion_only`.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"]
    pub fn set_enable_query_suggestion_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_query_suggestion_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_query_suggestion_when_no_answer`.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."]
    pub fn set_enable_query_suggestion_when_no_answer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_query_suggestion_when_no_answer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_model_config`.\n"]
    pub fn set_conversation_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conversation_process_config`.\n"]
    pub fn set_conversation_process_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_process_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_process_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_config`.\n"]
    pub fn set_query_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `suggestion_feature`.\n"]
    pub fn set_suggestion_feature(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.suggestion_feature = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.suggestion_feature = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `suggestion_trigger_settings`.\n"]
    pub fn set_suggestion_trigger_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.suggestion_trigger_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.suggestion_trigger_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs {
            disable_agent_query_logging: core::default::Default::default(),
            enable_conversation_augmented_query: core::default::Default::default(),
            enable_event_based_suggestion: core::default::Default::default(),
            enable_query_suggestion_only: core::default::Default::default(),
            enable_query_suggestion_when_no_answer: core::default::Default::default(),
            conversation_model_config: core::default::Default::default(),
            conversation_process_config: core::default::Default::default(),
            query_config: core::default::Default::default(),
            suggestion_feature: core::default::Default::default(),
            suggestion_trigger_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_agent_query_logging` after provisioning.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn disable_agent_query_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_agent_query_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_conversation_augmented_query` after provisioning.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn enable_conversation_augmented_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_conversation_augmented_query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_event_based_suggestion` after provisioning.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."]
    pub fn enable_event_based_suggestion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_event_based_suggestion", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_query_suggestion_only` after provisioning.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"]
    pub fn enable_query_suggestion_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_query_suggestion_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_query_suggestion_when_no_answer` after provisioning.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."]
    pub fn enable_query_suggestion_when_no_answer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_query_suggestion_when_no_answer", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `conversation_model_config` after provisioning.\n"]
    pub fn conversation_model_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_process_config` after provisioning.\n"]
    pub fn conversation_process_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_process_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_config` after provisioning.\n"]
    pub fn query_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_config", self.base))
    }

    #[doc = "Get a reference to the value of field `suggestion_feature` after provisioning.\n"]
    pub fn suggestion_feature(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suggestion_feature", self.base))
    }

    #[doc = "Get a reference to the value of field `suggestion_trigger_settings` after provisioning.\n"]
    pub fn suggestion_trigger_settings(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suggestion_trigger_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigDynamic {
    feature_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_high_latency_features_sync_delivery: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generators: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_suggestion_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_configs: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs>,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {
    #[doc =
        "Set the field `disable_high_latency_features_sync_delivery`.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn set_disable_high_latency_features_sync_delivery(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_high_latency_features_sync_delivery = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generators`.\nList of various generator resource names used in the conversation profile."]
    pub fn set_generators(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.generators = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `group_suggestion_responses`.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn set_group_suggestion_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.group_suggestion_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_configs`.\n"]
    pub fn set_feature_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.feature_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {
    type O =
        terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {
    pub fn build(self) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig {
            disable_high_latency_features_sync_delivery: core::default::Default::default(),
            generators: core::default::Default::default(),
            group_suggestion_responses: core::default::Default::default(),
            feature_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_high_latency_features_sync_delivery` after provisioning.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn disable_high_latency_features_sync_delivery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_high_latency_features_sync_delivery", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `generators` after provisioning.\nList of various generator resource names used in the conversation profile."]
    pub fn generators(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generators", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_suggestion_responses` after provisioning.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn group_suggestion_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_suggestion_responses", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_configs` after provisioning.\n"]
    pub fn feature_configs(
        &self,
    ) -> terrars
    ::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigDynamic {
    context_filter_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
        >,
    >,
    dialogflow_query_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
        >,
    >,
    sections: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_results: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context_filter_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dialogflow_query_source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sections: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
        >,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {
    #[doc =
        "Set the field `confidence_threshold`.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."]
    pub fn set_confidence_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.confidence_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_results`.\nMaximum number of results to return."]
    pub fn set_max_results(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `context_filter_settings`.\n"]
    pub fn set_context_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.context_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.context_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dialogflow_query_source`.\n"]
    pub fn set_dialogflow_query_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dialogflow_query_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dialogflow_query_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sections`.\n"]
    pub fn set_sections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_Sections,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig {
            confidence_threshold: core::default::Default::default(),
            max_results: core::default::Default::default(),
            context_filter_settings: core::default::Default::default(),
            dialogflow_query_source: core::default::Default::default(),
            sections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_threshold` after provisioning.\nConfidence threshold of query result.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, SMART_REPLY, SMART_COMPOSE, KNOWLEDGE_SEARCH, KNOWLEDGE_ASSIST, ENTITY_EXTRACTION."]
    pub fn confidence_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_results` after provisioning.\nMaximum number of results to return."]
    pub fn max_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_results", self.base))
    }

    #[doc = "Get a reference to the value of field `context_filter_settings` after provisioning.\n"]
    pub fn context_filter_settings(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_ContextFilterSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.context_filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dialogflow_query_source` after provisioning.\n"]
    pub fn dialogflow_query_source(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_DialogflowQuerySourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dialogflow_query_source", self.base))
    }

    #[doc = "Get a reference to the value of field `sections` after provisioning.\n"]
    pub fn sections(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_QueryConfig_SectionsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sections", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsDynamic {
    conversation_model_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
        >,
    >,
    conversation_process_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
        >,
    >,
    query_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig,
        >,
    >,
    suggestion_feature: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
        >,
    >,
    suggestion_trigger_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_agent_query_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_conversation_augmented_query: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_event_based_suggestion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_query_suggestion_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_query_suggestion_when_no_answer: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_model_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_process_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggestion_feature: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggestion_trigger_settings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
        >,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {
    #[doc =
        "Set the field `disable_agent_query_logging`.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn set_disable_agent_query_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_agent_query_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_conversation_augmented_query`.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn set_enable_conversation_augmented_query(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_conversation_augmented_query = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_event_based_suggestion`.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."]
    pub fn set_enable_event_based_suggestion(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_event_based_suggestion = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_query_suggestion_only`.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"]
    pub fn set_enable_query_suggestion_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_query_suggestion_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_query_suggestion_when_no_answer`.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."]
    pub fn set_enable_query_suggestion_when_no_answer(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_query_suggestion_when_no_answer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_model_config`.\n"]
    pub fn set_conversation_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conversation_process_config`.\n"]
    pub fn set_conversation_process_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_process_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_process_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_config`.\n"]
    pub fn set_query_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `suggestion_feature`.\n"]
    pub fn set_suggestion_feature(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeature,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.suggestion_feature = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.suggestion_feature = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `suggestion_trigger_settings`.\n"]
    pub fn set_suggestion_trigger_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.suggestion_trigger_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.suggestion_trigger_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {
    pub fn build(
        self,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs {
            disable_agent_query_logging: core::default::Default::default(),
            enable_conversation_augmented_query: core::default::Default::default(),
            enable_event_based_suggestion: core::default::Default::default(),
            enable_query_suggestion_only: core::default::Default::default(),
            enable_query_suggestion_when_no_answer: core::default::Default::default(),
            conversation_model_config: core::default::Default::default(),
            conversation_process_config: core::default::Default::default(),
            query_config: core::default::Default::default(),
            suggestion_feature: core::default::Default::default(),
            suggestion_trigger_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_agent_query_logging` after provisioning.\nDisable the logging of search queries sent by human agents. It can prevent those queries from being stored at answer records.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn disable_agent_query_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_agent_query_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_conversation_augmented_query` after provisioning.\nEnable including conversation context during query answer generation.\nThis feature is only supported for types: KNOWLEDGE_SEARCH."]
    pub fn enable_conversation_augmented_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_conversation_augmented_query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_event_based_suggestion` after provisioning.\nAutomatically iterates all participants and tries to compile suggestions.\nThis feature is only supported for types: ARTICLE_SUGGESTION, FAQ, DIALOGFLOW_ASSIST, KNOWLEDGE_ASSIST."]
    pub fn enable_event_based_suggestion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_event_based_suggestion", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_query_suggestion_only` after provisioning.\nEnable query suggestion only.\nThis feature is only supported for types: KNOWLEDGE_ASSIST"]
    pub fn enable_query_suggestion_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_query_suggestion_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_query_suggestion_when_no_answer` after provisioning.\nEnable query suggestion even if we can't find its answer. By default, queries are suggested only if we find its answer.\nThis feature is only supported for types: KNOWLEDGE_ASSIST."]
    pub fn enable_query_suggestion_when_no_answer(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_query_suggestion_when_no_answer", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `conversation_model_config` after provisioning.\n"]
    pub fn conversation_model_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationModelConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_process_config` after provisioning.\n"]
    pub fn conversation_process_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_ConversationProcessConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_process_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_config` after provisioning.\n"]
    pub fn query_config(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs_QueryConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_config", self.base))
    }

    #[doc = "Get a reference to the value of field `suggestion_feature` after provisioning.\n"]
    pub fn suggestion_feature(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionFeatureRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suggestion_feature", self.base))
    }

    #[doc = "Get a reference to the value of field `suggestion_trigger_settings` after provisioning.\n"]
    pub fn suggestion_trigger_settings(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig_FeatureConfigs_SuggestionTriggerSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.suggestion_trigger_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigDynamic {
    feature_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_high_latency_features_sync_delivery: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generators: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_suggestion_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs>,
    >,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {
    #[doc =
        "Set the field `disable_high_latency_features_sync_delivery`.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn set_disable_high_latency_features_sync_delivery(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_high_latency_features_sync_delivery = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generators`.\nList of various generator resource names used in the conversation profile."]
    pub fn set_generators(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.generators = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `group_suggestion_responses`.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn set_group_suggestion_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.group_suggestion_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_configs`.\n"]
    pub fn set_feature_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.feature_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {
    type O =
        terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {
    pub fn build(self) -> DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig {
            disable_high_latency_features_sync_delivery: core::default::Default::default(),
            generators: core::default::Default::default(),
            group_suggestion_responses: core::default::Default::default(),
            feature_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_high_latency_features_sync_delivery` after provisioning.\nWhen disableHighLatencyFeaturesSyncDelivery is true and using the AnalyzeContent API, we will not deliver the responses from high latency features in the API response. The humanAgentAssistantConfig.notification_config must be configured and enableEventBasedSuggestion must be set to true to receive the responses from high latency features in Pub/Sub. High latency feature(s): KNOWLEDGE_ASSIST"]
    pub fn disable_high_latency_features_sync_delivery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_high_latency_features_sync_delivery", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `generators` after provisioning.\nList of various generator resource names used in the conversation profile."]
    pub fn generators(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generators", self.base))
    }

    #[doc =
        "Get a reference to the value of field `group_suggestion_responses` after provisioning.\nIf groupSuggestionResponses is false, and there are multiple featureConfigs in event based suggestion or StreamingAnalyzeContent, we will try to deliver suggestions to customers as soon as we get new suggestion. Different type of suggestions based on the same context will be in separate Pub/Sub event or StreamingAnalyzeContentResponse.\n\nIf groupSuggestionResponses set to true. All the suggestions to the same participant based on the same context will be grouped into a single Pub/Sub event or StreamingAnalyzeContentResponse."]
    pub fn group_suggestion_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_suggestion_responses", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_configs` after provisioning.\n"]
    pub fn feature_configs(
        &self,
    ) -> terrars
    ::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig_FeatureConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_entity_extraction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sentiment_analysis: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {
    #[doc =
        "Set the field `enable_entity_extraction`.\nEnable entity extraction in conversation messages on agent assist stage."]
    pub fn set_enable_entity_extraction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_entity_extraction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_sentiment_analysis`.\nEnable sentiment analysis in conversation messages on agent assist stage. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral."]
    pub fn set_enable_sentiment_analysis(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_sentiment_analysis = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {
    pub fn build(self) -> DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig {
            enable_entity_extraction: core::default::Default::default(),
            enable_sentiment_analysis: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_entity_extraction` after provisioning.\nEnable entity extraction in conversation messages on agent assist stage."]
    pub fn enable_entity_extraction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_entity_extraction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_sentiment_analysis` after provisioning.\nEnable sentiment analysis in conversation messages on agent assist stage. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral."]
    pub fn enable_sentiment_analysis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sentiment_analysis", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_NotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_NotificationConfig {
    #[doc =
        "Set the field `message_format`.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_NotificationConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_NotificationConfig {}

impl BuildDialogflowConversationProfile_NotificationConfig {
    pub fn build(self) -> DialogflowConversationProfile_NotificationConfig {
        DialogflowConversationProfile_NotificationConfig {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_NotificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_NotificationConfigRef {
        DialogflowConversationProfile_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `message_format` after provisioning.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentAssistantConfigDynamic {
    end_user_suggestion_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig>>,
    human_agent_suggestion_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig>,
    >,
    message_analysis_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig>>,
    notification_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_NotificationConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_user_suggestion_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_agent_suggestion_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_analysis_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_NotificationConfig>>,
    dynamic: DialogflowConversationProfile_HumanAgentAssistantConfigDynamic,
}

impl DialogflowConversationProfile_HumanAgentAssistantConfig {
    #[doc = "Set the field `end_user_suggestion_config`.\n"]
    pub fn set_end_user_suggestion_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_user_suggestion_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_user_suggestion_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human_agent_suggestion_config`.\n"]
    pub fn set_human_agent_suggestion_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.human_agent_suggestion_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.human_agent_suggestion_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `message_analysis_config`.\n"]
    pub fn set_message_analysis_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_analysis_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_analysis_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_NotificationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_HumanAgentAssistantConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentAssistantConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentAssistantConfig {}

impl BuildDialogflowConversationProfile_HumanAgentAssistantConfig {
    pub fn build(self) -> DialogflowConversationProfile_HumanAgentAssistantConfig {
        DialogflowConversationProfile_HumanAgentAssistantConfig {
            end_user_suggestion_config: core::default::Default::default(),
            human_agent_suggestion_config: core::default::Default::default(),
            message_analysis_config: core::default::Default::default(),
            notification_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentAssistantConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_HumanAgentAssistantConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentAssistantConfigRef {
        DialogflowConversationProfile_HumanAgentAssistantConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentAssistantConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_user_suggestion_config` after provisioning.\n"]
    pub fn end_user_suggestion_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfig_EndUserSuggestionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_user_suggestion_config", self.base))
    }

    #[doc = "Get a reference to the value of field `human_agent_suggestion_config` after provisioning.\n"]
    pub fn human_agent_suggestion_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfig_HumanAgentSuggestionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_agent_suggestion_config", self.base))
    }

    #[doc = "Get a reference to the value of field `message_analysis_config` after provisioning.\n"]
    pub fn message_analysis_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentAssistantConfig_MessageAnalysisConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_analysis_config", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<DialogflowConversationProfile_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig {
    account_number: terrars::PrimField<std::string::String>,
}

impl DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig { }

impl terrars::ToListMappable for DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig {
    #[doc = "Account number of the LivePerson account to connect."]
    pub account_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig {
    pub fn build(self) -> DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig {
        DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig {
            account_number: self.account_number,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfigRef {
        DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_number` after provisioning.\nAccount number of the LivePerson account to connect."]
    pub fn account_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_number", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_HumanAgentHandoffConfigDynamic {
    live_person_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_HumanAgentHandoffConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_person_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig>>,
    dynamic: DialogflowConversationProfile_HumanAgentHandoffConfigDynamic,
}

impl DialogflowConversationProfile_HumanAgentHandoffConfig {
    #[doc = "Set the field `live_person_config`.\n"]
    pub fn set_live_person_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.live_person_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.live_person_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_HumanAgentHandoffConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_HumanAgentHandoffConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_HumanAgentHandoffConfig {}

impl BuildDialogflowConversationProfile_HumanAgentHandoffConfig {
    pub fn build(self) -> DialogflowConversationProfile_HumanAgentHandoffConfig {
        DialogflowConversationProfile_HumanAgentHandoffConfig {
            live_person_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_HumanAgentHandoffConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_HumanAgentHandoffConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_HumanAgentHandoffConfigRef {
        DialogflowConversationProfile_HumanAgentHandoffConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_HumanAgentHandoffConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `live_person_config` after provisioning.\n"]
    pub fn live_person_config(
        &self,
    ) -> terrars::ListRef<DialogflowConversationProfile_HumanAgentHandoffConfig_LivePersonConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_person_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_logging: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowConversationProfile_LoggingConfig {
    #[doc = "Set the field `enable_stackdriver_logging`.\nWhether to log conversation events"]
    pub fn set_enable_stackdriver_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_stackdriver_logging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_LoggingConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_LoggingConfig {}

impl BuildDialogflowConversationProfile_LoggingConfig {
    pub fn build(self) -> DialogflowConversationProfile_LoggingConfig {
        DialogflowConversationProfile_LoggingConfig { enable_stackdriver_logging: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_LoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_LoggingConfigRef {
        DialogflowConversationProfile_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nWhether to log conversation events"]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_NewMessageEventNotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_NewMessageEventNotificationConfig {
    #[doc =
        "Set the field `message_format`.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_NewMessageEventNotificationConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_NewMessageEventNotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_NewMessageEventNotificationConfig {}

impl BuildDialogflowConversationProfile_NewMessageEventNotificationConfig {
    pub fn build(self) -> DialogflowConversationProfile_NewMessageEventNotificationConfig {
        DialogflowConversationProfile_NewMessageEventNotificationConfig {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_NewMessageEventNotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_NewMessageEventNotificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_NewMessageEventNotificationConfigRef {
        DialogflowConversationProfile_NewMessageEventNotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_NewMessageEventNotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `message_format` after provisioning.\nFormat of the message Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_NewRecognitionResultNotificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_NewRecognitionResultNotificationConfig {
    #[doc =
        "Set the field `message_format`.\nFormat of message. Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn set_message_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `topic`.\nName of the Pub/Sub topic to publish conversation events like CONVERSATION_STARTED as serialized ConversationEvent protos.\nFor telephony integration to receive notification, make sure either this topic is in the same project as the conversation or you grant service-<Conversation Project Number>@gcp-sa-dialogflow.iam.gserviceaccount.com the Dialogflow Service Agent role in the topic project.\nFor chat integration to receive notification, make sure API caller has been granted the Dialogflow Service Agent role for the topic.\nFormat: projects/<Project ID>/locations/<Location ID>/topics/<Topic ID>."]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_NewRecognitionResultNotificationConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_NewRecognitionResultNotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_NewRecognitionResultNotificationConfig {}

impl BuildDialogflowConversationProfile_NewRecognitionResultNotificationConfig {
    pub fn build(self) -> DialogflowConversationProfile_NewRecognitionResultNotificationConfig {
        DialogflowConversationProfile_NewRecognitionResultNotificationConfig {
            message_format: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef {
        DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_NewRecognitionResultNotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `message_format` after provisioning.\nFormat of message. Possible values: [\"MESSAGE_FORMAT_UNSPECIFIED\", \"PROTO\", \"JSON\"]"]
    pub fn message_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nName of the Pub/Sub topic to publish conversation events like CONVERSATION_STARTED as serialized ConversationEvent protos.\nFor telephony integration to receive notification, make sure either this topic is in the same project as the conversation or you grant service-<Conversation Project Number>@gcp-sa-dialogflow.iam.gserviceaccount.com the Dialogflow Service Agent role in the topic project.\nFor chat integration to receive notification, make sure API caller has been granted the Dialogflow Service Agent role for the topic.\nFormat: projects/<Project ID>/locations/<Location ID>/topics/<Topic ID>."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_SttConfig_GeminiAsrConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_of_speech_sensitivity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_padding_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    silence_duration_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_of_speech_sensitivity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_SttConfig_GeminiAsrConfig {
    #[doc =
        "Set the field `end_of_speech_sensitivity`.\nEnd of speech sensitivity. Possible values: [\"END_SENSITIVITY_HIGH\", \"END_SENSITIVITY_LOW\"]"]
    pub fn set_end_of_speech_sensitivity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.end_of_speech_sensitivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_id`.\nThe Gemini ASR model ID used for transcription. This value overrides the default model ID configured on the server."]
    pub fn set_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_padding_ms`.\nThe required duration of detected speech before start-of-speech is committed."]
    pub fn set_prefix_padding_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.prefix_padding_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `silence_duration_ms`.\nThe required duration of detected silence (or non-speech) before end-of-speech is committed."]
    pub fn set_silence_duration_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.silence_duration_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_of_speech_sensitivity`.\nStart of speech sensitivity. Possible values: [\"START_SENSITIVITY_HIGH\", \"START_SENSITIVITY_LOW\"]"]
    pub fn set_start_of_speech_sensitivity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.start_of_speech_sensitivity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_SttConfig_GeminiAsrConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_SttConfig_GeminiAsrConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_SttConfig_GeminiAsrConfig {}

impl BuildDialogflowConversationProfile_SttConfig_GeminiAsrConfig {
    pub fn build(self) -> DialogflowConversationProfile_SttConfig_GeminiAsrConfig {
        DialogflowConversationProfile_SttConfig_GeminiAsrConfig {
            end_of_speech_sensitivity: core::default::Default::default(),
            model_id: core::default::Default::default(),
            prefix_padding_ms: core::default::Default::default(),
            silence_duration_ms: core::default::Default::default(),
            start_of_speech_sensitivity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_SttConfig_GeminiAsrConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_SttConfig_GeminiAsrConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_SttConfig_GeminiAsrConfigRef {
        DialogflowConversationProfile_SttConfig_GeminiAsrConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_SttConfig_GeminiAsrConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_of_speech_sensitivity` after provisioning.\nEnd of speech sensitivity. Possible values: [\"END_SENSITIVITY_HIGH\", \"END_SENSITIVITY_LOW\"]"]
    pub fn end_of_speech_sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_of_speech_sensitivity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_id` after provisioning.\nThe Gemini ASR model ID used for transcription. This value overrides the default model ID configured on the server."]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_padding_ms` after provisioning.\nThe required duration of detected speech before start-of-speech is committed."]
    pub fn prefix_padding_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_padding_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `silence_duration_ms` after provisioning.\nThe required duration of detected silence (or non-speech) before end-of-speech is committed."]
    pub fn silence_duration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.silence_duration_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_of_speech_sensitivity` after provisioning.\nStart of speech sensitivity. Possible values: [\"START_SENSITIVITY_HIGH\", \"START_SENSITIVITY_LOW\"]"]
    pub fn start_of_speech_sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_of_speech_sensitivity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_SttConfigDynamic {
    gemini_asr_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowConversationProfile_SttConfig_GeminiAsrConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_SttConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_word_info: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate_hertz: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speech_model_variant: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_gemini_asr: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_timeout_based_endpointing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gemini_asr_config: std::option::Option<std::vec::Vec<DialogflowConversationProfile_SttConfig_GeminiAsrConfig>>,
    dynamic: DialogflowConversationProfile_SttConfigDynamic,
}

impl DialogflowConversationProfile_SttConfig {
    #[doc =
        "Set the field `audio_encoding`.\nAudio encoding of the audio content to process. Possible values: [\"AUDIO_ENCODING_UNSPECIFIED\", \"AUDIO_ENCODING_LINEAR_16\", \"AUDIO_ENCODING_FLAC\", \"AUDIO_ENCODING_MULAW\", \"AUDIO_ENCODING_AMR\", \"AUDIO_ENCODING_AMR_WB\", \"AUDIO_ENCODING_OGG_OPUS\", \"AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE\"]"]
    pub fn set_audio_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_word_info`.\nIf true, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words."]
    pub fn set_enable_word_info(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_word_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code`.\nThe language of the supplied audio."]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model`.\nWhich Speech model to select.\nLeave this field unspecified to use Agent Speech settings for model selection."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate_hertz`.\nSample rate (in Hertz) of the audio content sent in the query."]
    pub fn set_sample_rate_hertz(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate_hertz = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `speech_model_variant`.\nThe speech model used in speech to text. Possible values: [\"SPEECH_MODEL_VARIANT_UNSPECIFIED\", \"USE_BEST_AVAILABLE\", \"USE_STANDARD\", \"USE_ENHANCED\"]"]
    pub fn set_speech_model_variant(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.speech_model_variant = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_gemini_asr`.\nIf true, Gemini ASR will be used for transcription instead of Cloud Speech-to-Text."]
    pub fn set_use_gemini_asr(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_gemini_asr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_timeout_based_endpointing`.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."]
    pub fn set_use_timeout_based_endpointing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_timeout_based_endpointing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gemini_asr_config`.\n"]
    pub fn set_gemini_asr_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowConversationProfile_SttConfig_GeminiAsrConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gemini_asr_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gemini_asr_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_SttConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_SttConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_SttConfig {}

impl BuildDialogflowConversationProfile_SttConfig {
    pub fn build(self) -> DialogflowConversationProfile_SttConfig {
        DialogflowConversationProfile_SttConfig {
            audio_encoding: core::default::Default::default(),
            enable_word_info: core::default::Default::default(),
            language_code: core::default::Default::default(),
            model: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
            speech_model_variant: core::default::Default::default(),
            use_gemini_asr: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
            gemini_asr_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_SttConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_SttConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowConversationProfile_SttConfigRef {
        DialogflowConversationProfile_SttConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_SttConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audio_encoding` after provisioning.\nAudio encoding of the audio content to process. Possible values: [\"AUDIO_ENCODING_UNSPECIFIED\", \"AUDIO_ENCODING_LINEAR_16\", \"AUDIO_ENCODING_FLAC\", \"AUDIO_ENCODING_MULAW\", \"AUDIO_ENCODING_AMR\", \"AUDIO_ENCODING_AMR_WB\", \"AUDIO_ENCODING_OGG_OPUS\", \"AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE\"]"]
    pub fn audio_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_word_info` after provisioning.\nIf true, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words."]
    pub fn enable_word_info(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_word_info", self.base))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the supplied audio."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nWhich Speech model to select.\nLeave this field unspecified to use Agent Speech settings for model selection."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_rate_hertz` after provisioning.\nSample rate (in Hertz) of the audio content sent in the query."]
    pub fn sample_rate_hertz(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate_hertz", self.base))
    }

    #[doc =
        "Get a reference to the value of field `speech_model_variant` after provisioning.\nThe speech model used in speech to text. Possible values: [\"SPEECH_MODEL_VARIANT_UNSPECIFIED\", \"USE_BEST_AVAILABLE\", \"USE_STANDARD\", \"USE_ENHANCED\"]"]
    pub fn speech_model_variant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.speech_model_variant", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_gemini_asr` after provisioning.\nIf true, Gemini ASR will be used for transcription instead of Cloud Speech-to-Text."]
    pub fn use_gemini_asr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_gemini_asr", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_timeout_based_endpointing` after provisioning.\nUse timeout based endpointing, interpreting endpointer sensitivity as seconds of timeout value."]
    pub fn use_timeout_based_endpointing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_timeout_based_endpointing", self.base))
    }

    #[doc = "Get a reference to the value of field `gemini_asr_config` after provisioning.\n"]
    pub fn gemini_asr_config(&self) -> terrars::ListRef<DialogflowConversationProfile_SttConfig_GeminiAsrConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gemini_asr_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_Timeouts {
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

impl terrars::ToListMappable for DialogflowConversationProfile_Timeouts {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_Timeouts {}

impl BuildDialogflowConversationProfile_Timeouts {
    pub fn build(self) -> DialogflowConversationProfile_Timeouts {
        DialogflowConversationProfile_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowConversationProfile_TimeoutsRef {
        DialogflowConversationProfile_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_TimeoutsRef {
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

#[derive(Serialize)]
pub struct DialogflowConversationProfile_TtsConfig_Voice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml_gender: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowConversationProfile_TtsConfig_Voice {
    #[doc = "Set the field `name`.\nThe name of the voice."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssml_gender`.\nThe preferred gender of the voice. Possible values: [\"SSML_VOICE_GENDER_UNSPECIFIED\", \"SSML_VOICE_GENDER_MALE\", \"SSML_VOICE_GENDER_FEMALE\", \"SSML_VOICE_GENDER_NEUTRAL\"]"]
    pub fn set_ssml_gender(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml_gender = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_TtsConfig_Voice {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_TtsConfig_Voice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_TtsConfig_Voice {}

impl BuildDialogflowConversationProfile_TtsConfig_Voice {
    pub fn build(self) -> DialogflowConversationProfile_TtsConfig_Voice {
        DialogflowConversationProfile_TtsConfig_Voice {
            name: core::default::Default::default(),
            ssml_gender: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_TtsConfig_VoiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_TtsConfig_VoiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowConversationProfile_TtsConfig_VoiceRef {
        DialogflowConversationProfile_TtsConfig_VoiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_TtsConfig_VoiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the voice."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml_gender` after provisioning.\nThe preferred gender of the voice. Possible values: [\"SSML_VOICE_GENDER_UNSPECIFIED\", \"SSML_VOICE_GENDER_MALE\", \"SSML_VOICE_GENDER_FEMALE\", \"SSML_VOICE_GENDER_NEUTRAL\"]"]
    pub fn ssml_gender(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml_gender", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowConversationProfile_TtsConfigDynamic {
    voice: std::option::Option<terrars::DynamicBlock<DialogflowConversationProfile_TtsConfig_Voice>>,
}

#[derive(Serialize)]
pub struct DialogflowConversationProfile_TtsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effects_profile_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pitch: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speaking_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_gain_db: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice: std::option::Option<std::vec::Vec<DialogflowConversationProfile_TtsConfig_Voice>>,
    dynamic: DialogflowConversationProfile_TtsConfigDynamic,
}

impl DialogflowConversationProfile_TtsConfig {
    #[doc =
        "Set the field `effects_profile_id`.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn set_effects_profile_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.effects_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pitch`.\nSpeaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub fn set_pitch(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pitch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `speaking_rate`.\nSpeaking rate/speed, in the range [0.25, 4.0]."]
    pub fn set_speaking_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.speaking_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_gain_db`.\nVolume gain (in dB) of the normal native volume supported by the specific voice."]
    pub fn set_volume_gain_db(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_gain_db = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `voice`.\n"]
    pub fn set_voice(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowConversationProfile_TtsConfig_Voice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.voice = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.voice = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowConversationProfile_TtsConfig {
    type O = terrars::BlockAssignable<DialogflowConversationProfile_TtsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowConversationProfile_TtsConfig {}

impl BuildDialogflowConversationProfile_TtsConfig {
    pub fn build(self) -> DialogflowConversationProfile_TtsConfig {
        DialogflowConversationProfile_TtsConfig {
            effects_profile_id: core::default::Default::default(),
            pitch: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
            voice: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowConversationProfile_TtsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowConversationProfile_TtsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowConversationProfile_TtsConfigRef {
        DialogflowConversationProfile_TtsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowConversationProfile_TtsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effects_profile_id` after provisioning.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn effects_profile_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effects_profile_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pitch` after provisioning.\nSpeaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
    pub fn pitch(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pitch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `speaking_rate` after provisioning.\nSpeaking rate/speed, in the range [0.25, 4.0]."]
    pub fn speaking_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.speaking_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_gain_db` after provisioning.\nVolume gain (in dB) of the normal native volume supported by the specific voice."]
    pub fn volume_gain_db(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_gain_db", self.base))
    }

    #[doc = "Get a reference to the value of field `voice` after provisioning.\n"]
    pub fn voice(&self) -> terrars::ListRef<DialogflowConversationProfile_TtsConfig_VoiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}
