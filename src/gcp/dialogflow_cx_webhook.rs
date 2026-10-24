use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxWebhookData {
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
    generic_web_service: std::option::Option<std::vec::Vec<DialogflowCxWebhookGenericWebServiceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory: std::option::Option<std::vec::Vec<DialogflowCxWebhookServiceDirectoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxWebhookTimeoutsEl>,
    dynamic: DialogflowCxWebhookDynamic,
}

struct DialogflowCxWebhook_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxWebhookData>,
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

    #[doc = "Set the field `disabled`.\nIndicates whether the webhook is disabled."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
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
        "Set the field `parent`.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_settings`.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxWebhookGenericWebServiceEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxWebhookServiceDirectoryEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxWebhookTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
        "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowCxWebhookGenericWebServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> terrars::ListRef<DialogflowCxWebhookServiceDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DialogflowCxWebhookData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
        "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowCxWebhookGenericWebServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> terrars::ListRef<DialogflowCxWebhookServiceDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookGenericWebServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ca_certs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxWebhookGenericWebServiceEl {
    #[doc =
        "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn set_allowed_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_ca_certs = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DialogflowCxWebhookGenericWebServiceEl {
    type O = terrars::BlockAssignable<DialogflowCxWebhookGenericWebServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookGenericWebServiceEl {
    #[doc = "Whether to use speech adaptation for speech recognition."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhookGenericWebServiceEl {
    pub fn build(self) -> DialogflowCxWebhookGenericWebServiceEl {
        DialogflowCxWebhookGenericWebServiceEl {
            allowed_ca_certs: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct DialogflowCxWebhookGenericWebServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhookGenericWebServiceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxWebhookGenericWebServiceElRef {
        DialogflowCxWebhookGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookGenericWebServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn allowed_ca_certs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ca_certs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ca_certs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[doc =
        "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn set_allowed_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_ca_certs = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    type O = terrars::BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[doc = "Whether to use speech adaptation for speech recognition."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    pub fn build(self) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
            allowed_ca_certs: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn allowed_ca_certs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ca_certs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxWebhookServiceDirectoryElDynamic {
    generic_web_service: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryEl {
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_web_service: std::option::Option<std::vec::Vec<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
    dynamic: DialogflowCxWebhookServiceDirectoryElDynamic,
}

impl DialogflowCxWebhookServiceDirectoryEl {
    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
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

impl terrars::ToListMappable for DialogflowCxWebhookServiceDirectoryEl {
    type O = terrars::BlockAssignable<DialogflowCxWebhookServiceDirectoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookServiceDirectoryEl {
    #[doc = "The name of Service Directory service."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxWebhookServiceDirectoryEl {
    pub fn build(self) -> DialogflowCxWebhookServiceDirectoryEl {
        DialogflowCxWebhookServiceDirectoryEl {
            service: self.service,
            generic_web_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxWebhookServiceDirectoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhookServiceDirectoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxWebhookServiceDirectoryElRef {
        DialogflowCxWebhookServiceDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookServiceDirectoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of Service Directory service."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxWebhookTimeoutsEl {
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

impl terrars::ToListMappable for DialogflowCxWebhookTimeoutsEl {
    type O = terrars::BlockAssignable<DialogflowCxWebhookTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookTimeoutsEl {}

impl BuildDialogflowCxWebhookTimeoutsEl {
    pub fn build(self) -> DialogflowCxWebhookTimeoutsEl {
        DialogflowCxWebhookTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxWebhookTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxWebhookTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DialogflowCxWebhookDynamic {
    generic_web_service: std::option::Option<terrars::DynamicBlock<DialogflowCxWebhookGenericWebServiceEl>>,
    service_directory: std::option::Option<terrars::DynamicBlock<DialogflowCxWebhookServiceDirectoryEl>>,
}
