#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxWebhookData_ {
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
    disabled: std::option::Option<terrars::PrimField<bool>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_spell_correction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_web_service: std::option::Option<std::vec::Vec<DialogflowCxWebhook_GenericWebService>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory: std::option::Option<std::vec::Vec<DialogflowCxWebhook_ServiceDirectory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxWebhook_Timeouts>,
    dynamic: DialogflowCxWebhookDynamic,
}

struct DialogflowCxWebhook_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxWebhookData_>,
}

#[derive(Clone)]
pub struct DialogflowCxWebhook(std::rc::Rc<DialogflowCxWebhook_>);

impl DialogflowCxWebhook {
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

    #[doc = "Set the field `disabled`.\nIndicates whether the webhook is disabled."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_spell_correction`.\nDeprecated. Indicates if automatic spell correction is enabled in detect intent requests."]
    pub fn set_enable_spell_correction(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_spell_correction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_stackdriver_logging`.\nDeprecated. Determines whether this agent should log conversation queries."]
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
        "Set the field `parent`.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_settings`.\nDeprecated. Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn set_security_settings(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nWebhook execution timeout."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generic_web_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generic_web_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_directory`.\n"]
    pub fn set_service_directory(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxWebhook_ServiceDirectory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_directory = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_directory = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxWebhook_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIndicates whether the webhook is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the webhook, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_spell_correction` after provisioning.\nDeprecated. Indicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_spell_correction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDeprecated. Determines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the webhook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nDeprecated. Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_flow` after provisioning.\nDeprecated. Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowCxWebhook_GenericWebServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> terrars::ListRef<DialogflowCxWebhook_ServiceDirectoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhook_TimeoutsRef {
        DialogflowCxWebhook_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxWebhook {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxWebhook { }

impl terrars::ToListMappable for DialogflowCxWebhook {
    type O = terrars::ListRef<DialogflowCxWebhookRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxWebhook_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_webhook".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxWebhook {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the webhook, unique within the agent."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhook {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxWebhook {
        let out = DialogflowCxWebhook(std::rc::Rc::new(DialogflowCxWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxWebhookData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: self.display_name,
                enable_spell_correction: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                security_settings: core::default::Default::default(),
                timeout: core::default::Default::default(),
                generic_web_service: core::default::Default::default(),
                service_directory: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxWebhookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxWebhookRef {
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
        "Get a reference to the value of field `disabled` after provisioning.\nIndicates whether the webhook is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the webhook, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_spell_correction` after provisioning.\nDeprecated. Indicates if automatic spell correction is enabled in detect intent requests."]
    pub fn enable_spell_correction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_spell_correction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nDeprecated. Determines whether this agent should log conversation queries."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the webhook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_settings` after provisioning.\nDeprecated. Name of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_flow` after provisioning.\nDeprecated. Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowCxWebhook_GenericWebServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> terrars::ListRef<DialogflowCxWebhook_ServiceDirectoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhook_TimeoutsRef {
        DialogflowCxWebhook_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxWebhookDynamic {
    generic_web_service: std::option::Option<terrars::DynamicBlock<DialogflowCxWebhook_GenericWebService>>,
    service_directory: std::option::Option<terrars::DynamicBlock<DialogflowCxWebhook_ServiceDirectory>>,
}

#[derive(Serialize)]
pub struct DialogflowCxWebhook_GenericWebService_OauthConfig {
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    token_endpoint: terrars::PrimField<std::string::String>,
}

impl DialogflowCxWebhook_GenericWebService_OauthConfig {
    #[doc =
        "Set the field `client_secret`.\nThe client secret provided by the 3rd party platform.  If the\n'secret_version_for_client_secret' field is set, this field will be\nignored."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_version_for_client_secret`.\nThe name of the SecretManager secret version resource storing the\nclient secret. If this field is set, the 'client_secret' field will be\nignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxWebhook_GenericWebService_OauthConfig {
    type O = terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhook_GenericWebService_OauthConfig {
    #[doc = "The client ID provided by the 3rd party platform."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "The token endpoint provided by the 3rd party platform to exchange an\naccess token."]
    pub token_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhook_GenericWebService_OauthConfig {
    pub fn build(self) -> DialogflowCxWebhook_GenericWebService_OauthConfig {
        DialogflowCxWebhook_GenericWebService_OauthConfig {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            scopes: core::default::Default::default(),
            secret_version_for_client_secret: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxWebhook_GenericWebService_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhook_GenericWebService_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxWebhook_GenericWebService_OauthConfigRef {
        DialogflowCxWebhook_GenericWebService_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhook_GenericWebService_OauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID provided by the 3rd party platform."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret provided by the 3rd party platform.  If the\n'secret_version_for_client_secret' field is set, this field will be\nignored."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_client_secret` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret. If this field is set, the 'client_secret' field will be\nignored.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint provided by the 3rd party platform to exchange an\naccess token."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders {
    key: terrars::PrimField<std::string::String>,
    secret_version: terrars::PrimField<std::string::String>,
}

impl DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders { }

impl terrars::ToListMappable for DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders {
    type O = terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc =
        "The SecretManager secret version resource storing the header value.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders {
    pub fn build(self) -> DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders {
        DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders {
            key: self.key,
            secret_version: self.secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeadersRef {
        DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe SecretManager secret version resource storing the header value.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig {
    service_account: terrars::PrimField<std::string::String>,
}

impl DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig { }

impl terrars::ToListMappable for DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig {
    type O = terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig {
    #[doc =
        "The email address of the service account used to authenticate the webhook call.\nDialogflow uses this service account to exchange an access token and the access\ntoken is then sent in the **Authorization** header of the webhook request.\n\nThe service account must have the **roles/iam.serviceAccountTokenCreator** role\ngranted to the\n[Dialogflow service agent](https://cloud.google.com/iam/docs/service-agents?_gl=1*1jsujvh*_ga*NjYxMzU3OTg2LjE3Njc3MzQ4NjM.*_ga_WH2QY8WWF5*czE3Njc3MzQ2MjgkbzIkZzEkdDE3Njc3MzQ3NzQkajYwJGwwJGgw#dialogflow-service-agent)."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig {
    pub fn build(self) -> DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig {
        DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig { service_account: self.service_account }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfigRef {
        DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used to authenticate the webhook call.\nDialogflow uses this service account to exchange an access token and the access\ntoken is then sent in the **Authorization** header of the webhook request.\n\nThe service account must have the **roles/iam.serviceAccountTokenCreator** role\ngranted to the\n[Dialogflow service agent](https://cloud.google.com/iam/docs/service-agents?_gl=1*1jsujvh*_ga*NjYxMzU3OTg2LjE3Njc3MzQ4NjM.*_ga_WH2QY8WWF5*czE3Njc3MzQ2MjgkbzIkZzEkdDE3Njc3MzQ3NzQkajYwJGwwJGgw#dialogflow-service-agent)."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxWebhook_GenericWebServiceDynamic {
    oauth_config: std::option::Option<terrars::DynamicBlock<DialogflowCxWebhook_GenericWebService_OauthConfig>>,
    secret_versions_for_request_headers: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders>>,
    service_account_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowCxWebhook_GenericWebService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ca_certs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_version_for_username_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<DialogflowCxWebhook_GenericWebService_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_versions_for_request_headers: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig>>,
    dynamic: DialogflowCxWebhook_GenericWebServiceDynamic,
}

impl DialogflowCxWebhook_GenericWebService {
    #[doc =
        "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, Dialogflow will use Google's default trust store\nto verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt\nname\". For instance a certificate can be self-signed using the following\ncommand,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn set_allowed_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_ca_certs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http_method`.\nHTTP method for the flexible webhook calls. Standard webhook always uses\nPOST. Possible values: [\"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn set_http_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameter_mapping`.\nMaps the values extracted from specific fields of the flexible webhook\nresponse into session parameters.\n- Key: session parameter name\n- Value: field path in the webhook response"]
    pub fn set_parameter_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameter_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_body`.\nDefines a custom JSON object as request body to send to flexible webhook."]
    pub fn set_request_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers`.\nThe HTTP request headers to send together with webhook requests."]
    pub fn set_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_version_for_username_password`.\nThe SecretManager secret version resource storing the username:password\npair for HTTP Basic authentication.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn set_secret_version_for_username_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_version_for_username_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_agent_auth`.\nIndicate the auth token type generated from the [Diglogflow service\nagent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent).\nThe generated token is sent in the Authorization header. Possible values: [\"NONE\", \"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn set_service_agent_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_agent_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `webhook_type`.\nType of the webhook. Possible values: [\"STANDARD\", \"FLEXIBLE\"]"]
    pub fn set_webhook_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webhook_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService_OauthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_versions_for_request_headers`.\n"]
    pub fn set_secret_versions_for_request_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxWebhook_GenericWebService_SecretVersionsForRequestHeaders>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_versions_for_request_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_versions_for_request_headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxWebhook_GenericWebService {
    type O = terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhook_GenericWebService {
    #[doc = "The webhook URI for receiving POST requests. It must use https protocol."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhook_GenericWebService {
    pub fn build(self) -> DialogflowCxWebhook_GenericWebService {
        DialogflowCxWebhook_GenericWebService {
            allowed_ca_certs: core::default::Default::default(),
            http_method: core::default::Default::default(),
            parameter_mapping: core::default::Default::default(),
            request_body: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            secret_version_for_username_password: core::default::Default::default(),
            service_agent_auth: core::default::Default::default(),
            uri: self.uri,
            webhook_type: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            secret_versions_for_request_headers: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxWebhook_GenericWebServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhook_GenericWebServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxWebhook_GenericWebServiceRef {
        DialogflowCxWebhook_GenericWebServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhook_GenericWebServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, Dialogflow will use Google's default trust store\nto verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt\nname\". For instance a certificate can be self-signed using the following\ncommand,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn allowed_ca_certs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ca_certs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_method` after provisioning.\nHTTP method for the flexible webhook calls. Standard webhook always uses\nPOST. Possible values: [\"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameter_mapping` after provisioning.\nMaps the values extracted from specific fields of the flexible webhook\nresponse into session parameters.\n- Key: session parameter name\n- Value: field path in the webhook response"]
    pub fn parameter_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameter_mapping", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body` after provisioning.\nDefines a custom JSON object as request body to send to flexible webhook."]
    pub fn request_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_version_for_username_password` after provisioning.\nThe SecretManager secret version resource storing the username:password\npair for HTTP Basic authentication.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'"]
    pub fn secret_version_for_username_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version_for_username_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_agent_auth` after provisioning.\nIndicate the auth token type generated from the [Diglogflow service\nagent](https://cloud.google.com/iam/docs/service-agents#dialogflow-service-agent).\nThe generated token is sent in the Authorization header. Possible values: [\"NONE\", \"ID_TOKEN\", \"ACCESS_TOKEN\"]"]
    pub fn service_agent_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent_auth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe webhook URI for receiving POST requests. It must use https protocol."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook_type` after provisioning.\nType of the webhook. Possible values: [\"STANDARD\", \"FLEXIBLE\"]"]
    pub fn webhook_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook_type", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<DialogflowCxWebhook_GenericWebService_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<DialogflowCxWebhook_GenericWebService_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxWebhook_ServiceDirectoryDynamic {
    generic_web_service: std::option::Option<terrars::DynamicBlock<DialogflowCxWebhook_GenericWebService>>,
}

#[derive(Serialize)]
pub struct DialogflowCxWebhook_ServiceDirectory {
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_web_service: std::option::Option<std::vec::Vec<DialogflowCxWebhook_GenericWebService>>,
    dynamic: DialogflowCxWebhook_ServiceDirectoryDynamic,
}

impl DialogflowCxWebhook_ServiceDirectory {
    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxWebhook_GenericWebService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generic_web_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_web_service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxWebhook_ServiceDirectory {
    type O = terrars::BlockAssignable<DialogflowCxWebhook_ServiceDirectory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhook_ServiceDirectory {
    #[doc = "The name of Service Directory service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhook_ServiceDirectory {
    pub fn build(self) -> DialogflowCxWebhook_ServiceDirectory {
        DialogflowCxWebhook_ServiceDirectory {
            service: self.service,
            generic_web_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxWebhook_ServiceDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhook_ServiceDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxWebhook_ServiceDirectoryRef {
        DialogflowCxWebhook_ServiceDirectoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhook_ServiceDirectoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of Service Directory service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowCxWebhook_GenericWebServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhook_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxWebhook_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxWebhook_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxWebhook_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhook_Timeouts {}

impl BuildDialogflowCxWebhook_Timeouts {
    pub fn build(self) -> DialogflowCxWebhook_Timeouts {
        DialogflowCxWebhook_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxWebhook_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhook_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxWebhook_TimeoutsRef {
        DialogflowCxWebhook_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhook_TimeoutsRef {
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
