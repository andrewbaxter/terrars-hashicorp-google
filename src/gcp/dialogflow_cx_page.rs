use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxPageData {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_route_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_settings: std::option::Option<std::vec::Vec<DialogflowCxPageAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_handlers: std::option::Option<std::vec::Vec<DialogflowCxPageEventHandlersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    form: std::option::Option<std::vec::Vec<DialogflowCxPageFormEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxPageTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_routes: std::option::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesEl>>,
    dynamic: DialogflowCxPageDynamic,
}

struct DialogflowCxPage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxPageData>,
}

#[derive(Clone)]
pub struct DialogflowCxPage(std::rc::Rc<DialogflowCxPage_>);

impl DialogflowCxPage {
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
        "Set the field `language_code`.\nThe language of the following fields in page:\n\nPage.entry_fulfillment.messages\nPage.entry_fulfillment.conditional_cases\nPage.event_handlers.trigger_fulfillment.messages\nPage.event_handlers.trigger_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.messages\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.reprompt_event_handlers.messages\nPage.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases\nPage.transition_routes.trigger_fulfillment.messages\nPage.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn set_language_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe flow to create a page for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transition_route_groups`.\nOrdered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.\nIf multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes.\nIf multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn set_transition_route_groups(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().transition_route_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageAdvancedSettingsEl>>,
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

    #[doc = "Set the field `entry_fulfillment`.\n"]
    pub fn set_entry_fulfillment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry_fulfillment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry_fulfillment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_handlers`.\n"]
    pub fn set_event_handlers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageEventHandlersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_handlers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_handlers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `form`.\n"]
    pub fn set_form(self, v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageFormEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().form = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.form = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxPageTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transition_routes`.\n"]
    pub fn set_transition_routes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageTransitionRoutesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transition_routes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transition_routes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the page, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in page:\n\nPage.entry_fulfillment.messages\nPage.entry_fulfillment.conditional_cases\nPage.event_handlers.trigger_fulfillment.messages\nPage.event_handlers.trigger_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.messages\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.reprompt_event_handlers.messages\nPage.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases\nPage.transition_routes.trigger_fulfillment.messages\nPage.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe flow to create a page for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transition_route_groups` after provisioning.\nOrdered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.\nIf multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes.\nIf multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_route_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxPageAdvancedSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_fulfillment` after provisioning.\n"]
    pub fn entry_fulfillment(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> terrars::ListRef<DialogflowCxPageEventHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `form` after provisioning.\n"]
    pub fn form(&self) -> terrars::ListRef<DialogflowCxPageFormElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.form", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPageTimeoutsElRef {
        DialogflowCxPageTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_routes", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxPage {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxPage { }

impl terrars::ToListMappable for DialogflowCxPage {
    type O = terrars::ListRef<DialogflowCxPageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxPage_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_page".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxPage {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the page, unique within the agent."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPage {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxPage {
        let out = DialogflowCxPage(std::rc::Rc::new(DialogflowCxPage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxPageData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                transition_route_groups: core::default::Default::default(),
                advanced_settings: core::default::Default::default(),
                entry_fulfillment: core::default::Default::default(),
                event_handlers: core::default::Default::default(),
                form: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transition_routes: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxPageRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the page, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in page:\n\nPage.entry_fulfillment.messages\nPage.entry_fulfillment.conditional_cases\nPage.event_handlers.trigger_fulfillment.messages\nPage.event_handlers.trigger_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.messages\nPage.form.parameters.fill_behavior.initial_prompt_fulfillment.conditional_cases\nPage.form.parameters.fill_behavior.reprompt_event_handlers.messages\nPage.form.parameters.fill_behavior.reprompt_event_handlers.conditional_cases\nPage.transition_routes.trigger_fulfillment.messages\nPage.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe flow to create a page for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transition_route_groups` after provisioning.\nOrdered list of TransitionRouteGroups associated with the page. Transition route groups must be unique within a page.\nIf multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes.\nIf multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_route_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxPageAdvancedSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_fulfillment` after provisioning.\n"]
    pub fn entry_fulfillment(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> terrars::ListRef<DialogflowCxPageEventHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `form` after provisioning.\n"]
    pub fn form(&self) -> terrars::ListRef<DialogflowCxPageFormElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.form", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxPageTimeoutsElRef {
        DialogflowCxPageTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_routes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_digits: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
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

impl terrars::ToListMappable for DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageAdvancedSettingsElDtmfSettingsEl {}

impl BuildDialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxPageAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef {
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

#[derive(Serialize, Default)]
struct DialogflowCxPageAdvancedSettingsElDynamic {
    dtmf_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageAdvancedSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_settings: std::option::Option<std::vec::Vec<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>>,
    dynamic: DialogflowCxPageAdvancedSettingsElDynamic,
}

impl DialogflowCxPageAdvancedSettingsEl {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageAdvancedSettingsElDtmfSettingsEl>>,
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
}

impl terrars::ToListMappable for DialogflowCxPageAdvancedSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageAdvancedSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageAdvancedSettingsEl {}

impl BuildDialogflowCxPageAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxPageAdvancedSettingsEl {
        DialogflowCxPageAdvancedSettingsEl {
            dtmf_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageAdvancedSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageAdvancedSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageAdvancedSettingsElRef {
        DialogflowCxPageAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageAdvancedSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> terrars::ListRef<DialogflowCxPageAdvancedSettingsElDtmfSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxPageEntryFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElConditionalCasesEl {
        DialogflowCxPageEntryFulfillmentElConditionalCasesEl { cases: core::default::Default::default() }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
        DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl { }

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl { audio_uri: self.audio_uri }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl { }

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl { phone_number: self.phone_number }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesElTextEl {
        DialogflowCxPageEntryFulfillmentElMessagesElTextEl { text: core::default::Default::default() }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageEntryFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>>,
    live_agent_handoff: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>>,
    output_audio_text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>>,
    play_audio: std::option::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>>,
    telephony_transfer_call: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>>,
    text: std::option::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxPageEntryFulfillmentElMessagesElDynamic,
}

impl DialogflowCxPageEntryFulfillmentElMessagesEl {
    #[doc =
        "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.live_agent_handoff = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_audio_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.play_audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesElTextEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElMessagesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElMessagesEl {}

impl BuildDialogflowCxPageEntryFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElMessagesEl {
        DialogflowCxPageEntryFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElMessagesElRef {
        DialogflowCxPageEntryFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElMessagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElConversationSuccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElLiveAgentHandoffElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElOutputAudioTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElPlayAudioElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElTelephonyTransferCallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
        DialogflowCxPageEntryFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageEntryFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>>,
    messages: std::option::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElMessagesEl>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageEntryFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std::option::Option<std::vec::Vec<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxPageEntryFulfillmentElDynamic,
}

impl DialogflowCxPageEntryFulfillmentEl {
    #[doc =
        "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_partial_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webhook = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElConditionalCasesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_cases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElMessagesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.messages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentElSetParameterActionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.set_parameter_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEntryFulfillmentEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEntryFulfillmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEntryFulfillmentEl {}

impl BuildDialogflowCxPageEntryFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageEntryFulfillmentEl {
        DialogflowCxPageEntryFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEntryFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEntryFulfillmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageEntryFulfillmentElRef {
        DialogflowCxPageEntryFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEntryFulfillmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_partial_responses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElConditionalCasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElMessagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(&self) -> terrars::ListRef<DialogflowCxPageEntryFulfillmentElSetParameterActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    type O =
        terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    type O =
        terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    type O =
        terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { }

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { audio_uri: self.audio_uri }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    live_agent_handoff: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>>,
    output_audio_text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    play_audio: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    telephony_transfer_call: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>,
    >,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElDynamic,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    #[doc =
        "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.live_agent_handoff = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_audio_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.play_audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageEventHandlersElTriggerFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    messages: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxPageEventHandlersElTriggerFulfillmentElDynamic,
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    #[doc =
        "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_partial_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webhook = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_cases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.messages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.set_parameter_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersElTriggerFulfillmentEl {}

impl BuildDialogflowCxPageEventHandlersElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
        DialogflowCxPageEventHandlersElTriggerFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
        DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElTriggerFulfillmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_partial_responses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElConditionalCasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElMessagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElSetParameterActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageEventHandlersElDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageEventHandlersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxPageEventHandlersElDynamic,
}

impl DialogflowCxPageEventHandlersEl {
    #[doc = "Set the field `event`.\nThe name of the event to handle."]
    pub fn set_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_flow`.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_target_flow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_flow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_page`.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_target_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageEventHandlersElTriggerFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageEventHandlersEl {
    type O = terrars::BlockAssignable<DialogflowCxPageEventHandlersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageEventHandlersEl {}

impl BuildDialogflowCxPageEventHandlersEl {
    pub fn build(self) -> DialogflowCxPageEventHandlersEl {
        DialogflowCxPageEventHandlersEl {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageEventHandlersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageEventHandlersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageEventHandlersElRef {
        DialogflowCxPageEventHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageEventHandlersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\nThe name of the event to handle."]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this event handler."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn target_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn target_page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(&self) -> terrars::ListRef<DialogflowCxPageEventHandlersElTriggerFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_digits: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
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

impl terrars::ToListMappable for DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {}

impl BuildDialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef {
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

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElAdvancedSettingsElDynamic {
    dtmf_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElAdvancedSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_settings: std::option::Option<std::vec::Vec<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>>,
    dynamic: DialogflowCxPageFormElParametersElAdvancedSettingsElDynamic,
}

impl DialogflowCxPageFormElParametersElAdvancedSettingsEl {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsEl>,
                    >,
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
}

impl terrars::ToListMappable for DialogflowCxPageFormElParametersElAdvancedSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElAdvancedSettingsEl {}

impl BuildDialogflowCxPageFormElParametersElAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElAdvancedSettingsEl {
        DialogflowCxPageFormElParametersElAdvancedSettingsEl {
            dtmf_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
        DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElAdvancedSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageFormElParametersElAdvancedSettingsElDtmfSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl { }

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl { }

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    live_agent_handoff: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    output_audio_text: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    play_audio: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl,
        >,
    >,
    telephony_transfer_call: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    text: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl>,
    >,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElDynamic,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
    #[doc =
        "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.live_agent_handoff = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_audio_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.play_audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElConversationSuccessElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElLiveAgentHandoffElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElOutputAudioTextElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElPlayAudioElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTelephonyTransferCallElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl>,
    >,
    messages: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl>,
    >,
    set_parameter_actions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl>,
    >,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElDynamic,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    #[doc =
        "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_partial_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webhook = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_cases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.messages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.set_parameter_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_partial_responses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElConditionalCasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElMessagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElSetParameterActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {

}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
            audio_uri: self.audio_uri,
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {

}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    live_agent_handoff: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    output_audio_text: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    play_audio: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl,
        >,
    >,
    telephony_transfer_call: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    text: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl,
        >,
    >,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElDynamic,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {
    #[doc =
        "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.live_agent_handoff = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_audio_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.play_audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElTextElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl,
        >,
    >,
    messages: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl,
        >,
    >,
    set_parameter_actions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl,
        >,
    >,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElDynamic,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
    #[doc =
        "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_partial_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webhook = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_cases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.messages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.set_parameter_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
    pub fn build(
        self,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_partial_responses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElConditionalCasesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElMessagesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars
    ::ListRef<
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElSetParameterActionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl>,
    >,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElDynamic,
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    #[doc = "Set the field `event`.\nThe name of the event to handle."]
    pub fn set_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_flow`.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_target_flow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_flow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_page`.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_target_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\nThe name of the event to handle."]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this event handler."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn target_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn target_page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElTriggerFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElFillBehaviorElDynamic {
    initial_prompt_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>>,
    reprompt_event_handlers: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersElFillBehaviorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_prompt_fulfillment: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reprompt_event_handlers: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>>,
    dynamic: DialogflowCxPageFormElParametersElFillBehaviorElDynamic,
}

impl DialogflowCxPageFormElParametersElFillBehaviorEl {
    #[doc = "Set the field `initial_prompt_fulfillment`.\n"]
    pub fn set_initial_prompt_fulfillment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initial_prompt_fulfillment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_prompt_fulfillment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reprompt_event_handlers`.\n"]
    pub fn set_reprompt_event_handlers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reprompt_event_handlers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reprompt_event_handlers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageFormElParametersElFillBehaviorEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersElFillBehaviorEl {}

impl BuildDialogflowCxPageFormElParametersElFillBehaviorEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersElFillBehaviorEl {
        DialogflowCxPageFormElParametersElFillBehaviorEl {
            initial_prompt_fulfillment: core::default::Default::default(),
            reprompt_event_handlers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElFillBehaviorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElFillBehaviorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageFormElParametersElFillBehaviorElRef {
        DialogflowCxPageFormElParametersElFillBehaviorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElFillBehaviorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_prompt_fulfillment` after provisioning.\n"]
    pub fn initial_prompt_fulfillment(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElInitialPromptFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_prompt_fulfillment", self.base))
    }

    #[doc = "Get a reference to the value of field `reprompt_event_handlers` after provisioning.\n"]
    pub fn reprompt_event_handlers(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElRepromptEventHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reprompt_event_handlers", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElParametersElDynamic {
    advanced_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageFormElParametersElAdvancedSettingsEl>>,
    fill_behavior: std::option::Option<terrars::DynamicBlock<DialogflowCxPageFormElParametersElFillBehaviorEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormElParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_list: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redact: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_settings: std::option::Option<std::vec::Vec<DialogflowCxPageFormElParametersElAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fill_behavior: std::option::Option<std::vec::Vec<DialogflowCxPageFormElParametersElFillBehaviorEl>>,
    dynamic: DialogflowCxPageFormElParametersElDynamic,
}

impl DialogflowCxPageFormElParametersEl {
    #[doc =
        "Set the field `default_value`.\nThe default value of an optional parameter. If the parameter is required, the default value will be ignored."]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe human-readable name of the parameter, unique within the form."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entity_type`.\nThe entity type of the parameter.\nFormat: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types."]
    pub fn set_entity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_list`.\nIndicates whether the parameter represents a list of values."]
    pub fn set_is_list(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_list = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redact`.\nIndicates whether the parameter content should be redacted in log.\nIf redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub fn set_redact(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.redact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required`.\nIndicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.\nRequired parameters must be filled before form filling concludes."]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageFormElParametersElAdvancedSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fill_behavior`.\n"]
    pub fn set_fill_behavior(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageFormElParametersElFillBehaviorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fill_behavior = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fill_behavior = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageFormElParametersEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormElParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormElParametersEl {}

impl BuildDialogflowCxPageFormElParametersEl {
    pub fn build(self) -> DialogflowCxPageFormElParametersEl {
        DialogflowCxPageFormElParametersEl {
            default_value: core::default::Default::default(),
            display_name: core::default::Default::default(),
            entity_type: core::default::Default::default(),
            is_list: core::default::Default::default(),
            redact: core::default::Default::default(),
            required: core::default::Default::default(),
            advanced_settings: core::default::Default::default(),
            fill_behavior: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageFormElParametersElRef {
        DialogflowCxPageFormElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_value` after provisioning.\nThe default value of an optional parameter. If the parameter is required, the default value will be ignored."]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the parameter, unique within the form."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entity_type` after provisioning.\nThe entity type of the parameter.\nFormat: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types."]
    pub fn entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_list` after provisioning.\nIndicates whether the parameter represents a list of values."]
    pub fn is_list(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redact` after provisioning.\nIndicates whether the parameter content should be redacted in log.\nIf redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
    pub fn redact(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nIndicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.\nRequired parameters must be filled before form filling concludes."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxPageFormElParametersElAdvancedSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `fill_behavior` after provisioning.\n"]
    pub fn fill_behavior(&self) -> terrars::ListRef<DialogflowCxPageFormElParametersElFillBehaviorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fill_behavior", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageFormElDynamic {
    parameters: std::option::Option<terrars::DynamicBlock<DialogflowCxPageFormElParametersEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageFormEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DialogflowCxPageFormElParametersEl>>,
    dynamic: DialogflowCxPageFormElDynamic,
}

impl DialogflowCxPageFormEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageFormElParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageFormEl {
    type O = terrars::BlockAssignable<DialogflowCxPageFormEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageFormEl {}

impl BuildDialogflowCxPageFormEl {
    pub fn build(self) -> DialogflowCxPageFormEl {
        DialogflowCxPageFormEl {
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageFormElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageFormElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageFormElRef {
        DialogflowCxPageFormElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageFormElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DialogflowCxPageFormElParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageTimeoutsEl {
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

impl terrars::ToListMappable for DialogflowCxPageTimeoutsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTimeoutsEl {}

impl BuildDialogflowCxPageTimeoutsEl {
    pub fn build(self) -> DialogflowCxPageTimeoutsEl {
        DialogflowCxPageTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageTimeoutsElRef {
        DialogflowCxPageTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTimeoutsElRef {
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
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cases` after provisioning.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn cases(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cases", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    type O =
        terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[doc = "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn set_ssml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe raw text to be synthesized."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    type O =
        terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML."]
    pub fn ssml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl { }

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl { audio_uri: self.audio_uri }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audio_uri` after provisioning.\nURI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub fn audio_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }

impl terrars
::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nTransfer the call to a phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    live_agent_handoff: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    output_audio_text: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>,
    >,
    play_audio: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    telephony_transfer_call: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>,
    >,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElDynamic,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    #[doc =
        "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload`.\nA custom, platform-specific payload."]
    pub fn set_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_success`.\n"]
    pub fn set_conversation_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_success = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `live_agent_handoff`.\n"]
    pub fn set_live_agent_handoff(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.live_agent_handoff = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.live_agent_handoff = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_audio_text`.\n"]
    pub fn set_output_audio_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_audio_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_audio_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `play_audio`.\n"]
    pub fn set_play_audio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.play_audio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.play_audio = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `telephony_transfer_call`.\n"]
    pub fn set_telephony_transfer_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.telephony_transfer_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.telephony_transfer_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `payload` after provisioning.\nA custom, platform-specific payload."]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    #[doc = "Set the field `parameter`.\nDisplay name of the parameter."]
    pub fn set_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameter` after provisioning.\nDisplay name of the parameter."]
    pub fn parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe new JSON-encoded value of the parameter. A null value clears the parameter."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>>,
    messages: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxPageTransitionRoutesElTriggerFulfillmentElDynamic,
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    #[doc =
        "Set the field `return_partial_responses`.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn set_return_partial_responses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_partial_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `webhook`.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn set_webhook(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.webhook = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_cases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_cases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.messages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.messages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `set_parameter_actions`.\n"]
    pub fn set_set_parameter_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.set_parameter_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.set_parameter_actions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {}

impl BuildDialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl {
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
        DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_partial_responses` after provisioning.\nWhether Dialogflow should return currently queued fulfillment response messages in streaming APIs. If a webhook is specified, it happens before Dialogflow invokes webhook. Warning: 1) This flag only affects streaming API. Responses are still queued and returned once in non-streaming API. 2) The flag can be enabled in any fulfillment but only the first 3 partial responses will be returned. You may only want to apply it to fulfillments that have slow webhooks."]
    pub fn return_partial_responses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_partial_responses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe tag used by the webhook to identify which fulfillment is being called. This field is required if webhook is specified."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `webhook` after provisioning.\nThe webhook to call. Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/webhooks/<Webhook ID>."]
    pub fn webhook(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhook", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElMessagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageTransitionRoutesElDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxPageTransitionRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxPageTransitionRoutesElDynamic,
}

impl DialogflowCxPageTransitionRoutesEl {
    #[doc =
        "Set the field `condition`.\nThe condition to evaluate against form parameters or session parameters.\nAt least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn set_condition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `intent`.\nThe unique identifier of an Intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>. Indicates that the transition can only happen when the given intent is matched. At least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn set_intent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.intent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_flow`.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn set_target_flow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_flow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_page`.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_target_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxPageTransitionRoutesElTriggerFulfillmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger_fulfillment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_fulfillment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxPageTransitionRoutesEl {
    type O = terrars::BlockAssignable<DialogflowCxPageTransitionRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxPageTransitionRoutesEl {}

impl BuildDialogflowCxPageTransitionRoutesEl {
    pub fn build(self) -> DialogflowCxPageTransitionRoutesEl {
        DialogflowCxPageTransitionRoutesEl {
            condition: core::default::Default::default(),
            intent: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxPageTransitionRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxPageTransitionRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxPageTransitionRoutesElRef {
        DialogflowCxPageTransitionRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxPageTransitionRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nThe condition to evaluate against form parameters or session parameters.\nAt least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `intent` after provisioning.\nThe unique identifier of an Intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>. Indicates that the transition can only happen when the given intent is matched. At least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled."]
    pub fn intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this transition route."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn target_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn target_page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(&self) -> terrars::ListRef<DialogflowCxPageTransitionRoutesElTriggerFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxPageDynamic {
    advanced_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxPageAdvancedSettingsEl>>,
    entry_fulfillment: std::option::Option<terrars::DynamicBlock<DialogflowCxPageEntryFulfillmentEl>>,
    event_handlers: std::option::Option<terrars::DynamicBlock<DialogflowCxPageEventHandlersEl>>,
    form: std::option::Option<terrars::DynamicBlock<DialogflowCxPageFormEl>>,
    transition_routes: std::option::Option<terrars::DynamicBlock<DialogflowCxPageTransitionRoutesEl>>,
}
