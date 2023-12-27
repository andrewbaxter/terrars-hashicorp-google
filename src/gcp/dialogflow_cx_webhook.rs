use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxWebhookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_spell_correction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_stackdriver_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_web_service: Option<Vec<DialogflowCxWebhookGenericWebServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory: Option<Vec<DialogflowCxWebhookServiceDirectoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxWebhookTimeoutsEl>,
    dynamic: DialogflowCxWebhookDynamic,
}

struct DialogflowCxWebhook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxWebhookData>,
}

#[derive(Clone)]
pub struct DialogflowCxWebhook(Rc<DialogflowCxWebhook_>);

impl DialogflowCxWebhook {
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

    #[doc= "Set the field `disabled`.\nIndicates whether the webhook is disabled."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
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

    #[doc= "Set the field `parent`.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }

    #[doc= "Set the field `security_settings`.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn set_security_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\nWebhook execution timeout."]
    pub fn set_timeout(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(self, v: impl Into<BlockAssignable<DialogflowCxWebhookGenericWebServiceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generic_web_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generic_web_service = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service_directory`.\n"]
    pub fn set_service_directory(self, v: impl Into<BlockAssignable<DialogflowCxWebhookServiceDirectoryEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_directory = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_directory = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxWebhookTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIndicates whether the webhook is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the webhook, unique within the agent."]
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the webhook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowCxWebhookGenericWebServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> ListRef<DialogflowCxWebhookServiceDirectoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DialogflowCxWebhook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DialogflowCxWebhook { }

impl ToListMappable for DialogflowCxWebhook {
    type O = ListRef<DialogflowCxWebhookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DialogflowCxWebhook_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_webhook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxWebhook {
    pub tf_id: String,
    #[doc= "The human-readable name of the webhook, unique within the agent."]
    pub display_name: PrimField<String>,
}

impl BuildDialogflowCxWebhook {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxWebhook {
        let out = DialogflowCxWebhook(Rc::new(DialogflowCxWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxWebhookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxWebhookRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxWebhookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxWebhookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIndicates whether the webhook is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the webhook, unique within the agent."]
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the webhook.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a webhook for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_settings` after provisioning.\nName of the SecuritySettings reference for the agent. Format: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn security_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `start_flow` after provisioning.\nName of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn start_flow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_flow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\nWebhook execution timeout."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowCxWebhookGenericWebServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_directory` after provisioning.\n"]
    pub fn service_directory(&self) -> ListRef<DialogflowCxWebhookServiceDirectoryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookGenericWebServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ca_certs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers: Option<RecField<PrimField<String>>>,
    uri: PrimField<String>,
}

impl DialogflowCxWebhookGenericWebServiceEl {
    #[doc= "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn set_allowed_ca_certs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_ca_certs = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers`.\nThe HTTP request headers to send together with webhook requests."]
    pub fn set_request_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.request_headers = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxWebhookGenericWebServiceEl {
    type O = BlockAssignable<DialogflowCxWebhookGenericWebServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookGenericWebServiceEl {
    #[doc= "Whether to use speech adaptation for speech recognition."]
    pub uri: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxWebhookGenericWebServiceElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookGenericWebServiceElRef {
        DialogflowCxWebhookGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookGenericWebServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn allowed_ca_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_ca_certs", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_ca_certs: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers: Option<RecField<PrimField<String>>>,
    uri: PrimField<String>,
}

impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[doc= "Set the field `allowed_ca_certs`.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn set_allowed_ca_certs(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_ca_certs = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers`.\nThe HTTP request headers to send together with webhook requests."]
    pub fn set_request_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.request_headers = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    type O = BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookServiceDirectoryElGenericWebServiceEl {
    #[doc= "Whether to use speech adaptation for speech recognition."]
    pub uri: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
        DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_ca_certs` after provisioning.\nSpecifies a list of allowed custom CA certificates (in DER format) for HTTPS verification."]
    pub fn allowed_ca_certs(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_ca_certs", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with webhook requests."]
    pub fn request_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\nWhether to use speech adaptation for speech recognition."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxWebhookServiceDirectoryElDynamic {
    generic_web_service: Option<DynamicBlock<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookServiceDirectoryEl {
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_web_service: Option<Vec<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
    dynamic: DialogflowCxWebhookServiceDirectoryElDynamic,
}

impl DialogflowCxWebhookServiceDirectoryEl {
    #[doc= "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxWebhookServiceDirectoryElGenericWebServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.generic_web_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_web_service = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DialogflowCxWebhookServiceDirectoryEl {
    type O = BlockAssignable<DialogflowCxWebhookServiceDirectoryEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxWebhookServiceDirectoryEl {
    #[doc= "The name of Service Directory service."]
    pub service: PrimField<String>,
}

impl BuildDialogflowCxWebhookServiceDirectoryEl {
    pub fn build(self) -> DialogflowCxWebhookServiceDirectoryEl {
        DialogflowCxWebhookServiceDirectoryEl {
            service: self.service,
            generic_web_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DialogflowCxWebhookServiceDirectoryElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxWebhookServiceDirectoryElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookServiceDirectoryElRef {
        DialogflowCxWebhookServiceDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookServiceDirectoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nThe name of Service Directory service."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc= "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowCxWebhookServiceDirectoryElGenericWebServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxWebhookTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DialogflowCxWebhookTimeoutsEl {
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

impl ToListMappable for DialogflowCxWebhookTimeoutsEl {
    type O = BlockAssignable<DialogflowCxWebhookTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowCxWebhookTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxWebhookTimeoutsElRef {
        DialogflowCxWebhookTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxWebhookTimeoutsElRef {
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
struct DialogflowCxWebhookDynamic {
    generic_web_service: Option<DynamicBlock<DialogflowCxWebhookGenericWebServiceEl>>,
    service_directory: Option<DynamicBlock<DialogflowCxWebhookServiceDirectoryEl>>,
}
