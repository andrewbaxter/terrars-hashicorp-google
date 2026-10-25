#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxFlowData_ {
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
    advanced_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_AdvancedSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_handlers: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_connector_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nlu_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_NluSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxFlow_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_routes: std::option::Option<std::vec::Vec<DialogflowCxFlow_TransitionRoutes>>,
    dynamic: DialogflowCxFlowDynamic,
}

struct DialogflowCxFlow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxFlowData_>,
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_EventHandlers>>,
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

    #[doc = "Set the field `knowledge_connector_settings`.\n"]
    pub fn set_knowledge_connector_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings>>,
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

    #[doc = "Set the field `nlu_settings`.\n"]
    pub fn set_nlu_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_NluSettings>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxFlow_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transition_routes`.\n"]
    pub fn set_transition_routes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_TransitionRoutes>>,
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> terrars::ListRef<DialogflowCxFlow_EventHandlersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(&self) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_connector_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_settings` after provisioning.\n"]
    pub fn nlu_settings(&self) -> terrars::ListRef<DialogflowCxFlow_NluSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nlu_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxFlow_TimeoutsRef {
        DialogflowCxFlow_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> terrars::ListRef<DialogflowCxFlow_TransitionRoutesRef> {
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
            data: std::cell::RefCell::new(DialogflowCxFlowData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                is_default_start_flow: core::default::Default::default(),
                language_code: core::default::Default::default(),
                parent: core::default::Default::default(),
                transition_route_groups: core::default::Default::default(),
                advanced_settings: core::default::Default::default(),
                event_handlers: core::default::Default::default(),
                knowledge_connector_settings: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn advanced_settings(&self) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_handlers` after provisioning.\n"]
    pub fn event_handlers(&self) -> terrars::ListRef<DialogflowCxFlow_EventHandlersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_connector_settings` after provisioning.\n"]
    pub fn knowledge_connector_settings(&self) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_connector_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nlu_settings` after provisioning.\n"]
    pub fn nlu_settings(&self) -> terrars::ListRef<DialogflowCxFlow_NluSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nlu_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxFlow_TimeoutsRef {
        DialogflowCxFlow_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transition_routes` after provisioning.\n"]
    pub fn transition_routes(&self) -> terrars::ListRef<DialogflowCxFlow_TransitionRoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transition_routes", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlowDynamic {
    advanced_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_AdvancedSettings>>,
    event_handlers: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers>>,
    knowledge_connector_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings>>,
    nlu_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_NluSettings>>,
    transition_routes: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_TransitionRoutes>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination {
    #[doc =
        "Set the field `uri`.\nThe Google Cloud Storage URI for the exported objects. Whether a full object name, or just a prefix, its usage depends on the Dialogflow operation.\nFormat: gs://bucket/object-name-or-prefix"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination {
    type O = terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination {}

impl BuildDialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination {
    pub fn build(self) -> DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination {
        DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestinationRef {
        DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestinationRef {
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
pub struct DialogflowCxFlow_AdvancedSettings_DtmfSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_digits: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxFlow_AdvancedSettings_DtmfSettings {
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

impl terrars::ToListMappable for DialogflowCxFlow_AdvancedSettings_DtmfSettings {
    type O = terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_DtmfSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_AdvancedSettings_DtmfSettings {}

impl BuildDialogflowCxFlow_AdvancedSettings_DtmfSettings {
    pub fn build(self) -> DialogflowCxFlow_AdvancedSettings_DtmfSettings {
        DialogflowCxFlow_AdvancedSettings_DtmfSettings {
            enabled: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_AdvancedSettings_DtmfSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_AdvancedSettings_DtmfSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_AdvancedSettings_DtmfSettingsRef {
        DialogflowCxFlow_AdvancedSettings_DtmfSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_AdvancedSettings_DtmfSettingsRef {
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
pub struct DialogflowCxFlow_AdvancedSettings_LoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_consent_based_redaction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_interaction_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_logging: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxFlow_AdvancedSettings_LoggingSettings {
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

impl terrars::ToListMappable for DialogflowCxFlow_AdvancedSettings_LoggingSettings {
    type O = terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_LoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_AdvancedSettings_LoggingSettings {}

impl BuildDialogflowCxFlow_AdvancedSettings_LoggingSettings {
    pub fn build(self) -> DialogflowCxFlow_AdvancedSettings_LoggingSettings {
        DialogflowCxFlow_AdvancedSettings_LoggingSettings {
            enable_consent_based_redaction: core::default::Default::default(),
            enable_interaction_logging: core::default::Default::default(),
            enable_stackdriver_logging: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef {
        DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef {
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
pub struct DialogflowCxFlow_AdvancedSettings_SpeechSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpointer_sensitivity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    models: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_speech_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_timeout_based_endpointing: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxFlow_AdvancedSettings_SpeechSettings {
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

impl terrars::ToListMappable for DialogflowCxFlow_AdvancedSettings_SpeechSettings {
    type O = terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_SpeechSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_AdvancedSettings_SpeechSettings {}

impl BuildDialogflowCxFlow_AdvancedSettings_SpeechSettings {
    pub fn build(self) -> DialogflowCxFlow_AdvancedSettings_SpeechSettings {
        DialogflowCxFlow_AdvancedSettings_SpeechSettings {
            endpointer_sensitivity: core::default::Default::default(),
            models: core::default::Default::default(),
            no_speech_timeout: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_AdvancedSettings_SpeechSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_AdvancedSettings_SpeechSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_AdvancedSettings_SpeechSettingsRef {
        DialogflowCxFlow_AdvancedSettings_SpeechSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_AdvancedSettings_SpeechSettingsRef {
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
struct DialogflowCxFlow_AdvancedSettingsDynamic {
    audio_export_gcs_destination: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination>>,
    dtmf_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_AdvancedSettings_DtmfSettings>>,
    logging_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_AdvancedSettings_LoggingSettings>>,
    speech_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_AdvancedSettings_SpeechSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_AdvancedSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_export_gcs_destination: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_AdvancedSettings_DtmfSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_AdvancedSettings_LoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speech_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_AdvancedSettings_SpeechSettings>>,
    dynamic: DialogflowCxFlow_AdvancedSettingsDynamic,
}

impl DialogflowCxFlow_AdvancedSettings {
    #[doc = "Set the field `audio_export_gcs_destination`.\n"]
    pub fn set_audio_export_gcs_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestination>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_DtmfSettings>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_LoggingSettings>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_SpeechSettings>>,
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

impl terrars::ToListMappable for DialogflowCxFlow_AdvancedSettings {
    type O = terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_AdvancedSettings {}

impl BuildDialogflowCxFlow_AdvancedSettings {
    pub fn build(self) -> DialogflowCxFlow_AdvancedSettings {
        DialogflowCxFlow_AdvancedSettings {
            audio_export_gcs_destination: core::default::Default::default(),
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_AdvancedSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_AdvancedSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlow_AdvancedSettingsRef {
        DialogflowCxFlow_AdvancedSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_AdvancedSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_export_gcs_destination` after provisioning.\n"]
    pub fn audio_export_gcs_destination(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettings_AudioExportGcsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_export_gcs_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(&self) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettings_DtmfSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]
    pub fn speech_settings(&self) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettings_SpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.speech_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cases: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {
    #[doc =
        "Set the field `cases`.\nA JSON encoded list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored.\nSee [Case](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/Fulfillment#case) for the schema."]
    pub fn set_cases(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cases = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases {
            cases: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {
    type O =
        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess {
            metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {
    #[doc = "Set the field `metadata`.\nCustom metadata. Dialogflow doesn't impose any structure on this."]
    pub fn set_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff {
            metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {
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

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioTextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioTextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioTextRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioTextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioTextRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio {
    audio_uri: terrars::PrimField<std::string::String>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio { }

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio {
    #[doc =
        "URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
    pub audio_uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio { audio_uri: self.audio_uri }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall {
    phone_number: terrars::PrimField<std::string::String>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall { }

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall {
    type O =
        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall {
    #[doc = "Transfer the call to a phone number in E.164 format."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall {
            phone_number: self.phone_number,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text { text: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TextRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TextRef {
    #[allow(dead_code)]
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
struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesDynamic {
    conversation_success: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess>>,
    live_agent_handoff: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>>,
    output_audio_text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText>>,
    play_audio: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>>,
    telephony_transfer_call: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall>>,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text>>,
    dynamic: DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesDynamic,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {
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
                            DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess,
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
                        ::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>,
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
                        ::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioText>,
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
                        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>,
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
                            DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall,
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
                    ::Into<terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_Text>>,
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

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages {
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

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_OutputAudioTextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {
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

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions {
            parameter: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef {
    #[allow(dead_code)]
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
struct DialogflowCxFlow_EventHandlers_TriggerFulfillmentDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>>,
    messages: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_generative_fallback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>>,
    dynamic: DialogflowCxFlow_EventHandlers_TriggerFulfillmentDynamic,
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillment {
    #[doc =
        "Set the field `enable_generative_fallback`.\nIf the flag is true, the agent will utilize LLM to generate a text response.\nIf LLM generation fails, the defined responses in the fulfillment will be respected.\nThis flag is only useful for fulfillments associated with no-match event handlers."]
    pub fn set_enable_generative_fallback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_generative_fallback = std::option::Option::Some(v.into());
        self
    }

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
                        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>,
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
                    ::Into<terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>>,
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
                        ::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>,
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

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers_TriggerFulfillment {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment {}

impl BuildDialogflowCxFlow_EventHandlers_TriggerFulfillment {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers_TriggerFulfillment {
        DialogflowCxFlow_EventHandlers_TriggerFulfillment {
            enable_generative_fallback: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlers_TriggerFulfillmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlers_TriggerFulfillmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_EventHandlers_TriggerFulfillmentRef {
        DialogflowCxFlow_EventHandlers_TriggerFulfillmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlers_TriggerFulfillmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_generative_fallback` after provisioning.\nIf the flag is true, the agent will utilize LLM to generate a text response.\nIf LLM generation fails, the defined responses in the fulfillment will be respected.\nThis flag is only useful for fulfillments associated with no-match event handlers."]
    pub fn enable_generative_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_generative_fallback", self.base))
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
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlow_EventHandlersDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_EventHandlers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment>>,
    dynamic: DialogflowCxFlow_EventHandlersDynamic,
}

impl DialogflowCxFlow_EventHandlers {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment>>,
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

impl terrars::ToListMappable for DialogflowCxFlow_EventHandlers {
    type O = terrars::BlockAssignable<DialogflowCxFlow_EventHandlers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_EventHandlers {}

impl BuildDialogflowCxFlow_EventHandlers {
    pub fn build(self) -> DialogflowCxFlow_EventHandlers {
        DialogflowCxFlow_EventHandlers {
            event: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_EventHandlersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_EventHandlersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlow_EventHandlersRef {
        DialogflowCxFlow_EventHandlersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_EventHandlersRef {
    #[allow(dead_code)]
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
    pub fn trigger_fulfillment(&self) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_processing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {
    #[doc =
        "Set the field `data_store`.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn set_data_store(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_store = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_store_type`.\nThe type of the connected data store.\n* PUBLIC_WEB: A data store that contains public web content.\n* UNSTRUCTURED: A data store that contains unstructured private data.\n* STRUCTURED: A data store that contains structured data (for example FAQ). Possible values: [\"PUBLIC_WEB\", \"UNSTRUCTURED\", \"STRUCTURED\"]"]
    pub fn set_data_store_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_store_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `document_processing_mode`.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\n* DOCUMENTS: Documents are processed as documents.\n* CHUNKS: Documents are converted to chunks. Possible values: [\"DOCUMENTS\", \"CHUNKS\"]"]
    pub fn set_document_processing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.document_processing_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {
    type O = terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {
        DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections {
            data_store: core::default::Default::default(),
            data_store_type: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnectionsRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_store` after provisioning.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn data_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_store_type` after provisioning.\nThe type of the connected data store.\n* PUBLIC_WEB: A data store that contains public web content.\n* UNSTRUCTURED: A data store that contains unstructured private data.\n* STRUCTURED: A data store that contains structured data (for example FAQ). Possible values: [\"PUBLIC_WEB\", \"UNSTRUCTURED\", \"STRUCTURED\"]"]
    pub fn data_store_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\n* DOCUMENTS: Documents are processed as documents.\n* CHUNKS: Documents are converted to chunks. Possible values: [\"DOCUMENTS\", \"CHUNKS\"]"]
    pub fn document_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_processing_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpointing_timeout_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interdigit_timeout_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_digits: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {
    #[doc =
        "Set the field `enabled`.\nIf true, incoming audio is processed for DTMF (dual tone multi frequtectency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will de the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpointing_timeout_duration`.\nEndpoint timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
    pub fn set_endpointing_timeout_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.endpointing_timeout_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finish_digit`.\nThe digit that terminates a DTMF digit sequence."]
    pub fn set_finish_digit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finish_digit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interdigit_timeout_duration`.\nInterdigit timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
    pub fn set_interdigit_timeout_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.interdigit_timeout_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_digits`.\nMax length of DTMF digits."]
    pub fn set_max_digits(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_digits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {
    pub fn build(
        self,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings {
            enabled: core::default::Default::default(),
            endpointing_timeout_duration: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
            interdigit_timeout_duration: core::default::Default::default(),
            max_digits: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettingsRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf true, incoming audio is processed for DTMF (dual tone multi frequtectency) events. For example, if the caller presses a button on their telephone keypad and DTMF processing is enabled, Dialogflow will de the event (e.g. a \"3\" was pressed) in the incoming audio and pass the event to the bot to drive business logic (e.g. when 3 is pressed, return the account balance)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpointing_timeout_duration` after provisioning.\nEndpoint timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
    pub fn endpointing_timeout_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpointing_timeout_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `finish_digit` after provisioning.\nThe digit that terminates a DTMF digit sequence."]
    pub fn finish_digit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interdigit_timeout_duration` after provisioning.\nInterdigit timeout setting for matching dtmf input to regex.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
    pub fn interdigit_timeout_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interdigit_timeout_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `max_digits` after provisioning.\nMax length of DTMF digits."]
    pub fn max_digits(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_digits", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpointer_sensitivity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    models: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_speech_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_timeout_based_endpointing: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {
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
        "Set the field `no_speech_timeout`.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
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

impl terrars
::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {
    pub fn build(
        self,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings {
            endpointer_sensitivity: core::default::Default::default(),
            models: core::default::Default::default(),
            no_speech_timeout: core::default::Default::default(),
            use_timeout_based_endpointing: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettingsRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettingsRef {
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
        "Get a reference to the value of field `no_speech_timeout` after provisioning.\nTimeout before detecting no speech.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.500s\"."]
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
struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsDynamic {
    dtmf_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings>,
    >,
    logging_settings: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_AdvancedSettings_LoggingSettings>>,
    speech_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf_settings: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_settings: std::option::Option<std::vec::Vec<DialogflowCxFlow_AdvancedSettings_LoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speech_settings: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings>,
    >,
    dynamic: DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsDynamic,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {
    #[doc = "Set the field `dtmf_settings`.\n"]
    pub fn set_dtmf_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettings,
                        >,
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

    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_AdvancedSettings_LoggingSettings>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettings,
                        >,
                    >,
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

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {
    type O =
        terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings {
            dtmf_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            speech_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dtmf_settings` after provisioning.\n"]
    pub fn dtmf_settings(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_DtmfSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> terrars::ListRef<DialogflowCxFlow_AdvancedSettings_LoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `speech_settings` after provisioning.\n"]
    pub fn speech_settings(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings_SpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.speech_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction {}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction { }

impl terrars
::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteraction {}
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteractionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteractionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteractionRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteractionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteractionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_playback_interruption: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {
    #[doc = "Set the field `allow_playback_interruption`.\n"]
    pub fn set_allow_playback_interruption(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_playback_interruption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio`.\n"]
    pub fn set_audio(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {
    pub fn build(
        self,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments {
            allow_playback_interruption: core::default::Default::default(),
            audio: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_SegmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_SegmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_SegmentsRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_SegmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_SegmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_playback_interruption` after provisioning.\n"]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc = "Get a reference to the value of field `audio` after provisioning.\n"]
    pub fn audio(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segments: std
    ::option
    ::Option<
        terrars
        ::ListField<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments>,
    >,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {
    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_Segments,
                        >,
                    >,
    ) -> Self {
        self.segments = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {
    type O =
        terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio {
            segments: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudioRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudioRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudio_SegmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segments", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard {}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard { }

impl terrars
::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard {}
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCardRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {
    #[doc =
        "Set the field `ssml`.\nThe SSML text to be synthesized. For more information, see SSML.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn set_ssml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssml = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `text`.\nThe raw text to be synthesized.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText {
            ssml: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioTextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioTextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioTextRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioTextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioTextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssml` after provisioning.\nThe SSML text to be synthesized. For more information, see SSML.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn ssml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssml", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nThe raw text to be synthesized.\nThis field is part of a union field 'source': Only one of 'text' or 'ssml' may be set."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {
    #[doc =
        "Set the field `text`.\nA collection of text response variants. If multiple variants are defined, only one text response variant is returned at runtime.\nrequired: true"]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {
    type O = terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text {
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_TextRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_TextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_playback_interruption` after provisioning.\nWhether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
    pub fn allow_playback_interruption(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_playback_interruption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nA collection of text response variants. If multiple variants are defined, only one text response variant is returned at runtime.\nrequired: true"]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesDynamic {
    conversation_success: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess>>,
    knowledge_info_card: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard>,
    >,
    live_agent_handoff: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>>,
    output_audio_text: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText>,
    >,
    play_audio: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>>,
    telephony_transfer_call: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall>>,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_success: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_info_card: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    live_agent_handoff: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    play_audio: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telephony_transfer_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text>>,
    dynamic: DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesDynamic,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {
    #[doc =
        "Set the field `channel`.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payload`.\nReturns a response containing a custom, platform-specific payload.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
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
                            DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccess,
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

    #[doc = "Set the field `knowledge_info_card`.\n"]
    pub fn set_knowledge_info_card(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCard,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knowledge_info_card = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_info_card = std::option::Option::Some(d);
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
                        ::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoff>,
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
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioText,
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
                        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudio>,
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
                            DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCall,
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
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_Text,
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

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {
    type O = terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages {
            channel: core::default::Default::default(),
            payload: core::default::Default::default(),
            conversation_success: core::default::Default::default(),
            knowledge_info_card: core::default::Default::default(),
            live_agent_handoff: core::default::Default::default(),
            output_audio_text: core::default::Default::default(),
            play_audio: core::default::Default::default(),
            telephony_transfer_call: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe channel which the response is associated with. Clients can specify the channel via QueryParameters.channel, and only associated channel response will be returned."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc =
        "Get a reference to the value of field `end_interaction` after provisioning.\nThis type has no fields.\nIndicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn end_interaction(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_EndInteractionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_interaction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mixed_audio` after provisioning.\nRepresents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via playAudio. This message is generated by Dialogflow only and not supposed to be defined by the user.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn mixed_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_MixedAudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mixed_audio", self.base))
    }

    #[doc =
        "Get a reference to the value of field `payload` after provisioning.\nReturns a response containing a custom, platform-specific payload.\nThis field is part of a union field 'message': Only one of 'text', 'payload', 'conversationSuccess', 'outputAudioText', 'liveAgentHandoff', 'endInteraction', 'playAudio', 'mixedAudio', 'telephonyTransferCall', or 'knowledgeInfoCard' may be set."]
    pub fn payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_success` after provisioning.\n"]
    pub fn conversation_success(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_ConversationSuccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_success", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_info_card` after provisioning.\n"]
    pub fn knowledge_info_card(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_KnowledgeInfoCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_info_card", self.base))
    }

    #[doc = "Get a reference to the value of field `live_agent_handoff` after provisioning.\n"]
    pub fn live_agent_handoff(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_LiveAgentHandoffRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.live_agent_handoff", self.base))
    }

    #[doc = "Get a reference to the value of field `output_audio_text` after provisioning.\n"]
    pub fn output_audio_text(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_OutputAudioTextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_audio_text", self.base))
    }

    #[doc = "Get a reference to the value of field `play_audio` after provisioning.\n"]
    pub fn play_audio(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_PlayAudioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.play_audio", self.base))
    }

    #[doc = "Get a reference to the value of field `telephony_transfer_call` after provisioning.\n"]
    pub fn telephony_transfer_call(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages_TelephonyTransferCallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telephony_transfer_call", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentDynamic {
    advanced_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings>>,
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>>,
    messages: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_generative_fallback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_settings: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>>,
    dynamic: DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentDynamic,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {
    #[doc =
        "Set the field `enable_generative_fallback`.\nIf the flag is true, the agent will utilize LLM to generate a text response. If LLM generation fails, the defined responses in the fulfillment will be respected. This flag is only useful for fulfillments associated with no-match event handlers."]
    pub fn set_enable_generative_fallback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_generative_fallback = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `advanced_settings`.\n"]
    pub fn set_advanced_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettings,
                        >,
                    >,
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

    #[doc = "Set the field `conditional_cases`.\n"]
    pub fn set_conditional_cases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>,
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
                        ::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_Messages>,
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
                        ::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>,
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

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {
    type O = terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment {
            enable_generative_fallback: core::default::Default::default(),
            return_partial_responses: core::default::Default::default(),
            tag: core::default::Default::default(),
            webhook: core::default::Default::default(),
            advanced_settings: core::default::Default::default(),
            conditional_cases: core::default::Default::default(),
            messages: core::default::Default::default(),
            set_parameter_actions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentRef {
        DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_generative_fallback` after provisioning.\nIf the flag is true, the agent will utilize LLM to generate a text response. If LLM generation fails, the defined responses in the fulfillment will be respected. This flag is only useful for fulfillments associated with no-match event handlers."]
    pub fn enable_generative_fallback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_generative_fallback", self.base))
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

    #[doc = "Get a reference to the value of field `advanced_settings` after provisioning.\n"]
    pub fn advanced_settings(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_AdvancedSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `conditional_cases` after provisioning.\n"]
    pub fn conditional_cases(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment_MessagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlow_KnowledgeConnectorSettingsDynamic {
    data_store_connections: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections>>,
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_connections: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment>>,
    dynamic: DialogflowCxFlow_KnowledgeConnectorSettingsDynamic,
}

impl DialogflowCxFlow_KnowledgeConnectorSettings {
    #[doc = "Set the field `enabled`.\nWhether Knowledge Connector is enabled or not."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_flow`.\nThe target flow to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>.\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn set_target_flow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_flow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_page`.\nThe target page to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>/pages/<PageID>.\nThe page must be in the same host flow (the flow that owns this 'KnowledgeConnectorSettings').\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn set_target_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store_connections`.\n"]
    pub fn set_data_store_connections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnections>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store_connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_connections = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger_fulfillment`.\n"]
    pub fn set_trigger_fulfillment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillment>>,
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

impl terrars::ToListMappable for DialogflowCxFlow_KnowledgeConnectorSettings {
    type O = terrars::BlockAssignable<DialogflowCxFlow_KnowledgeConnectorSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_KnowledgeConnectorSettings {}

impl BuildDialogflowCxFlow_KnowledgeConnectorSettings {
    pub fn build(self) -> DialogflowCxFlow_KnowledgeConnectorSettings {
        DialogflowCxFlow_KnowledgeConnectorSettings {
            enabled: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            data_store_connections: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_KnowledgeConnectorSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_KnowledgeConnectorSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_KnowledgeConnectorSettingsRef {
        DialogflowCxFlow_KnowledgeConnectorSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_KnowledgeConnectorSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Knowledge Connector is enabled or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_flow` after provisioning.\nThe target flow to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>.\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn target_flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_flow", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_page` after provisioning.\nThe target page to transition to. Format: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/flows/<FlowID>/pages/<PageID>.\nThe page must be in the same host flow (the flow that owns this 'KnowledgeConnectorSettings').\nThis field is part of a union field 'target': Only one of 'targetPage' or 'targetFlow' may be set."]
    pub fn target_page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_page", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store_connections` after provisioning.\n"]
    pub fn data_store_connections(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_DataStoreConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_fulfillment` after provisioning.\n"]
    pub fn trigger_fulfillment(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_KnowledgeConnectorSettings_TriggerFulfillmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_NluSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classification_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_training_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_NluSettings {
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

impl terrars::ToListMappable for DialogflowCxFlow_NluSettings {
    type O = terrars::BlockAssignable<DialogflowCxFlow_NluSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_NluSettings {}

impl BuildDialogflowCxFlow_NluSettings {
    pub fn build(self) -> DialogflowCxFlow_NluSettings {
        DialogflowCxFlow_NluSettings {
            classification_threshold: core::default::Default::default(),
            model_training_mode: core::default::Default::default(),
            model_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_NluSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_NluSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlow_NluSettingsRef {
        DialogflowCxFlow_NluSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_NluSettingsRef {
    #[allow(dead_code)]
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
pub struct DialogflowCxFlow_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxFlow_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxFlow_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxFlow_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_Timeouts {}

impl BuildDialogflowCxFlow_Timeouts {
    pub fn build(self) -> DialogflowCxFlow_Timeouts {
        DialogflowCxFlow_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlow_TimeoutsRef {
        DialogflowCxFlow_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_TimeoutsRef {
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

#[derive(Serialize, Default)]
struct DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentDynamic {
    conditional_cases: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>>,
    messages: std::option::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>>,
    set_parameter_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_TransitionRoutes_TriggerFulfillment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_partial_responses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_cases: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_parameter_actions: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>>,
    dynamic: DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentDynamic,
}

impl DialogflowCxFlow_TransitionRoutes_TriggerFulfillment {
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
                        terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCases>,
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
                    ::Into<terrars::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_Messages>>,
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
                        ::BlockAssignable<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActions>,
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

impl terrars::ToListMappable for DialogflowCxFlow_TransitionRoutes_TriggerFulfillment {
    type O = terrars::BlockAssignable<DialogflowCxFlow_TransitionRoutes_TriggerFulfillment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_TransitionRoutes_TriggerFulfillment {}

impl BuildDialogflowCxFlow_TransitionRoutes_TriggerFulfillment {
    pub fn build(self) -> DialogflowCxFlow_TransitionRoutes_TriggerFulfillment {
        DialogflowCxFlow_TransitionRoutes_TriggerFulfillment {
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

#[allow(dead_code)]
pub struct DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentRef {
        DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_ConditionalCasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditional_cases", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_MessagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `set_parameter_actions` after provisioning.\n"]
    pub fn set_parameter_actions(
        &self,
    ) -> terrars::ListRef<DialogflowCxFlow_EventHandlers_TriggerFulfillment_SetParameterActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_parameter_actions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxFlow_TransitionRoutesDynamic {
    trigger_fulfillment: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxFlow_TransitionRoutes_TriggerFulfillment>>,
}

#[derive(Serialize)]
pub struct DialogflowCxFlow_TransitionRoutes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_fulfillment: std::option::Option<std::vec::Vec<DialogflowCxFlow_TransitionRoutes_TriggerFulfillment>>,
    dynamic: DialogflowCxFlow_TransitionRoutesDynamic,
}

impl DialogflowCxFlow_TransitionRoutes {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxFlow_TransitionRoutes_TriggerFulfillment>>,
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

impl terrars::ToListMappable for DialogflowCxFlow_TransitionRoutes {
    type O = terrars::BlockAssignable<DialogflowCxFlow_TransitionRoutes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxFlow_TransitionRoutes {}

impl BuildDialogflowCxFlow_TransitionRoutes {
    pub fn build(self) -> DialogflowCxFlow_TransitionRoutes {
        DialogflowCxFlow_TransitionRoutes {
            condition: core::default::Default::default(),
            intent: core::default::Default::default(),
            target_flow: core::default::Default::default(),
            target_page: core::default::Default::default(),
            trigger_fulfillment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxFlow_TransitionRoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxFlow_TransitionRoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxFlow_TransitionRoutesRef {
        DialogflowCxFlow_TransitionRoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxFlow_TransitionRoutesRef {
    #[allow(dead_code)]
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
    pub fn trigger_fulfillment(&self) -> terrars::ListRef<DialogflowCxFlow_TransitionRoutes_TriggerFulfillmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_fulfillment", self.base))
    }
}
