use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxFlowData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default_start_flow: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_route_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_settings: std::option::Option<std::vec::Vec<DialogflowCxFlowAdvancedSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_handlers: std::option::Option<std::vec::Vec<DialogflowCxFlowEventHandlersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nlu_settings: std::option::Option<std::vec::Vec<DialogflowCxFlowNluSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxFlowTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_routes: std::option::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesEl>>,
    dynamic: DialogflowCxFlowDynamic,
}

struct DialogflowCxFlow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxFlowData>,
}

#[derive(Clone)]
pub struct DialogflowCxFlow(std::rc::Rc<DialogflowCxFlow_>);

impl DialogflowCxFlow {
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
        "Set the field `description`.\nThe description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
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
        "Set the field `is_default_start_flow`.\nMarks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.\nThe Default Start Flow cannot be deleted; deleting the 'google_dialogflow_cx_flow' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_flow' resources linked to the same agent with 'is_default_start_flow = true' because they will compete to control a single Default Start Flow resource in GCP."]
    pub fn set_is_default_start_flow(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_default_start_flow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `language_code`.\nThe language of the following fields in flow:\nFlow.event_handlers.trigger_fulfillment.messages\nFlow.event_handlers.trigger_fulfillment.conditional_cases\nFlow.transition_routes.trigger_fulfillment.messages\nFlow.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn set_language_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transition_route_groups`.\nA flow's transition route group serve two purposes:\nThey are responsible for matching the user's first utterances in the flow.\nThey are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowAdvancedSettingsEl>>,
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

    #[doc = "Set the field `event_handlers`.\n"]
    pub fn set_event_handlers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowEventHandlersEl>>,
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

    #[doc = "Set the field `nlu_settings`.\n"]
    pub fn set_nlu_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowNluSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nlu_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nlu_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxFlowTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transition_routes`.\n"]
    pub fn set_transition_routes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesEl>>,
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
        "Get a reference to the value of field `description` after provisioning.\nThe description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the flow."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default_start_flow` after provisioning.\nMarks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.\nThe Default Start Flow cannot be deleted; deleting the 'google_dialogflow_cx_flow' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_flow' resources linked to the same agent with 'is_default_start_flow = true' because they will compete to control a single Default Start Flow resource in GCP."]
    pub fn is_default_start_flow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_start_flow", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in flow:\nFlow.event_handlers.trigger_fulfillment.messages\nFlow.event_handlers.trigger_fulfillment.conditional_cases\nFlow.transition_routes.trigger_fulfillment.messages\nFlow.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the flow.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transition_route_groups` after provisioning.\nA flow's transition route group serve two purposes:\nThey are responsible for matching the user's first utterances in the flow.\nThey are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_route_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxFlowAdvancedSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> terrars::ListRef<DialogflowCxFlowEventHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_settings` after provisioning.\n"]
    pub fn nlu_settings(&self) -> terrars::ListRef<DialogflowCxFlowNluSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nlu_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxFlowTimeoutsElRef {
        DialogflowCxFlowTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_routes", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxFlow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxFlow { }

impl terrars::ToListMappable for DialogflowCxFlow {
    type O = terrars::ListRef<DialogflowCxFlowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxFlow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_flow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxFlow {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the flow."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlow {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxFlow {
        let out = DialogflowCxFlow(std::rc::Rc::new(DialogflowCxFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxFlowData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                is_default_start_flow: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                transition_route_groups: core::default::Default::default(),
                advanced_settings: core::default::Default::default(),
                event_handlers: core::default::Default::default(),
                nlu_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transition_routes: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxFlowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxFlowRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the flow."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default_start_flow` after provisioning.\nMarks this as the [Default Start Flow](https://cloud.google.com/dialogflow/cx/docs/concept/flow#start) for an agent. When you create an agent, the Default Start Flow is created automatically.\nThe Default Start Flow cannot be deleted; deleting the 'google_dialogflow_cx_flow' resource does nothing to the underlying GCP resources.\n\n~> Avoid having multiple 'google_dialogflow_cx_flow' resources linked to the same agent with 'is_default_start_flow = true' because they will compete to control a single Default Start Flow resource in GCP."]
    pub fn is_default_start_flow(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_start_flow", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the following fields in flow:\nFlow.event_handlers.trigger_fulfillment.messages\nFlow.event_handlers.trigger_fulfillment.conditional_cases\nFlow.transition_routes.trigger_fulfillment.messages\nFlow.transition_routes.trigger_fulfillment.conditional_cases\nIf not specified, the agent's default language is used. Many languages are supported. Note: languages must be enabled in the agent before they can be used."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the flow.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a flow for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transition_route_groups` after provisioning.\nA flow's transition route group serve two purposes:\nThey are responsible for matching the user's first utterances in the flow.\nThey are inherited by every page's [transition route groups][Page.transition_route_groups]. Transition route groups defined in the page have higher priority than those defined in the flow.\nFormat:projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/transitionRouteGroups/<TransitionRouteGroup ID>."]
    pub fn transition_route_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_route_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxFlowAdvancedSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> terrars::ListRef<DialogflowCxFlowEventHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_settings` after provisioning.\n"]
    pub fn nlu_settings(&self) -> terrars::ListRef<DialogflowCxFlowNluSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nlu_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxFlowTimeoutsElRef {
        DialogflowCxFlowTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_routes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    #[doc =
        "Set the field `uri`.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {}

impl BuildDialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl {
        DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl { uri: core::default::Default::default() }
    }
}

pub struct DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
        DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef {
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
pub struct DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_digits: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
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

impl terrars::ToListMappable for DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {}

impl BuildDialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
        DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
        DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef {
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
struct DialogflowCxFlowAdvancedSettingsElDynamic {
    audio_export_gcs_destination: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>>,
    dtmf_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowAdvancedSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_export_gcs_destination: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_settings: std::option::Option<std::vec::Vec<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>>,
    dynamic: DialogflowCxFlowAdvancedSettingsElDynamic,
}

impl DialogflowCxFlowAdvancedSettingsEl {
    #[doc = "Set the field `audio_export_gcs_destination`.\n"]
    pub fn set_audio_export_gcs_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowAdvancedSettingsElDtmfSettingsEl>>,
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

impl terrars::ToListMappable for DialogflowCxFlowAdvancedSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowAdvancedSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowAdvancedSettingsEl {}

impl BuildDialogflowCxFlowAdvancedSettingsEl {
    pub fn build(self) -> DialogflowCxFlowAdvancedSettingsEl {
        DialogflowCxFlowAdvancedSettingsEl {
            audio_export_gcs_destination: core::default::Default::default(),
            dtmf_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowAdvancedSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowAdvancedSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlowAdvancedSettingsElRef {
        DialogflowCxFlowAdvancedSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowAdvancedSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_export_gcs_destination` after provisioning.\n"]
    pub fn audio_export_gcs_destination(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowAdvancedSettingsElAudioExportGcsDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_export_gcs_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> terrars::ListRef<DialogflowCxFlowAdvancedSettingsElDtmfSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    type O =
        terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    type O =
        terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
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

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    type O =
        terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { }

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl { audio_uri: self.audio_uri }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
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
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef {
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
struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    live_agent_handoff: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>>,
    output_audio_text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    play_audio: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    telephony_transfer_call: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>,
    >,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElDynamic,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
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
                            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessEl,
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
                            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
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
                            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextEl,
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
                        ::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioEl>,
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
                            DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
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
                        terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextEl>,
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

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl {
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

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef {
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
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElConversationSuccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElOutputAudioTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElPlayAudioElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
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

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef {
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
struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    messages: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxFlowEventHandlersElTriggerFulfillmentElDynamic,
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
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
                        ::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesEl>,
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
                    ::Into<terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesEl>>,
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
                        ::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsEl>,
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

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentEl {}

impl BuildDialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentEl {
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

pub struct DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
        DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef {
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
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElConditionalCasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElMessagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElSetParameterActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlowEventHandlersElDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowEventHandlersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxFlowEventHandlersElDynamic,
}

impl DialogflowCxFlowEventHandlersEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowEventHandlersElTriggerFulfillmentEl>>,
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

impl terrars::ToListMappable for DialogflowCxFlowEventHandlersEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowEventHandlersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowEventHandlersEl {}

impl BuildDialogflowCxFlowEventHandlersEl {
    pub fn build(self) -> DialogflowCxFlowEventHandlersEl {
        DialogflowCxFlowEventHandlersEl {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowEventHandlersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowEventHandlersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlowEventHandlersElRef {
        DialogflowCxFlowEventHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowEventHandlersElRef {
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
    pub fn trigger_fulfillment(&self) -> terrars::ListRef<DialogflowCxFlowEventHandlersElTriggerFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlowNluSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classification_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_training_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowNluSettingsEl {
    #[doc =
        "Set the field `classification_threshold`.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold.\nIf the returned score value is less than the threshold value, then a no-match event will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used."]
    pub fn set_classification_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.classification_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_training_mode`.\nIndicates NLU model training mode.\n* MODEL_TRAINING_MODE_AUTOMATIC: NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode.\n* MODEL_TRAINING_MODE_MANUAL: User needs to manually trigger NLU model training. Best for large flows whose models take long time to train. Possible values: [\"MODEL_TRAINING_MODE_AUTOMATIC\", \"MODEL_TRAINING_MODE_MANUAL\"]"]
    pub fn set_model_training_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.model_training_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_type`.\nIndicates the type of NLU model.\n* MODEL_TYPE_STANDARD: Use standard NLU model.\n* MODEL_TYPE_ADVANCED: Use advanced NLU model. Possible values: [\"MODEL_TYPE_STANDARD\", \"MODEL_TYPE_ADVANCED\"]"]
    pub fn set_model_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowNluSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowNluSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowNluSettingsEl {}

impl BuildDialogflowCxFlowNluSettingsEl {
    pub fn build(self) -> DialogflowCxFlowNluSettingsEl {
        DialogflowCxFlowNluSettingsEl {
            classification_threshold: core::default::Default::default(),
            model_training_mode: core::default::Default::default(),
            model_type: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowNluSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowNluSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlowNluSettingsElRef {
        DialogflowCxFlowNluSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowNluSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `classification_threshold` after provisioning.\nTo filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold.\nIf the returned score value is less than the threshold value, then a no-match event will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used."]
    pub fn classification_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_training_mode` after provisioning.\nIndicates NLU model training mode.\n* MODEL_TRAINING_MODE_AUTOMATIC: NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode.\n* MODEL_TRAINING_MODE_MANUAL: User needs to manually trigger NLU model training. Best for large flows whose models take long time to train. Possible values: [\"MODEL_TRAINING_MODE_AUTOMATIC\", \"MODEL_TRAINING_MODE_MANUAL\"]"]
    pub fn model_training_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_training_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_type` after provisioning.\nIndicates the type of NLU model.\n* MODEL_TYPE_STANDARD: Use standard NLU model.\n* MODEL_TYPE_ADVANCED: Use advanced NLU model. Possible values: [\"MODEL_TYPE_STANDARD\", \"MODEL_TYPE_ADVANCED\"]"]
    pub fn model_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlowTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowTimeoutsEl {
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

impl terrars::ToListMappable for DialogflowCxFlowTimeoutsEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTimeoutsEl {}

impl BuildDialogflowCxFlowTimeoutsEl {
    pub fn build(self) -> DialogflowCxFlowTimeoutsEl {
        DialogflowCxFlowTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlowTimeoutsElRef {
        DialogflowCxFlowTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTimeoutsElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl {
            cases: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    type O =
        terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl {
            metadata: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
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

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    type O =
        terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl { }

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl { audio_uri: self.audio_uri }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl { }

impl terrars
::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl {
            phone_number: self.phone_number,
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef {
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
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef {
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
struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElDynamic {
    conversation_success: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>,
    >,
    live_agent_handoff: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>,
    >,
    output_audio_text: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>,
    >,
    play_audio: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    telephony_transfer_call: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>,
    >,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>>,
    dynamic: DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElDynamic,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
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
                            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessEl,
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
                            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffEl,
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
                            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextEl,
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
                        ::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioEl>,
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
                            DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallEl,
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
                        ::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextEl>,
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

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl {
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

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef {
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
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElConversationSuccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElLiveAgentHandoffElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElOutputAudioTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElPlayAudioElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTelephonyTransferCallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
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

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef {
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
struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>>,
    messages: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>>,
    dynamic: DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElDynamic,
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
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
                        ::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesEl>,
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
                        terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesEl>,
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
                        ::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsEl>,
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

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {}

impl BuildDialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl {
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

pub struct DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
        DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef {
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
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElConditionalCasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElMessagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElSetParameterActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlowTransitionRoutesElDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlowTransitionRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>>,
    dynamic: DialogflowCxFlowTransitionRoutesElDynamic,
}

impl DialogflowCxFlowTransitionRoutesEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentEl>>,
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

impl terrars::ToListMappable for DialogflowCxFlowTransitionRoutesEl {
    type O = terrars::BlockAssignable<DialogflowCxFlowTransitionRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlowTransitionRoutesEl {}

impl BuildDialogflowCxFlowTransitionRoutesEl {
    pub fn build(self) -> DialogflowCxFlowTransitionRoutesEl {
        DialogflowCxFlowTransitionRoutesEl {
            condition: core::default::Default::default(),
            intent: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxFlowTransitionRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlowTransitionRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlowTransitionRoutesElRef {
        DialogflowCxFlowTransitionRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlowTransitionRoutesElRef {
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
    pub fn trigger_fulfillment(&self) -> terrars::ListRef<DialogflowCxFlowTransitionRoutesElTriggerFulfillmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlowDynamic {
    advanced_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlowAdvancedSettingsEl>>,
    event_handlers: std::option::Option<terrars::DynamicBlock<DialogflowCxFlowEventHandlersEl>>,
    nlu_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlowNluSettingsEl>>,
    transition_routes: std::option::Option<terrars::DynamicBlock<DialogflowCxFlowTransitionRoutesEl>>,
}
