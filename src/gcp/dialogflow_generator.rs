#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowGeneratorData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generator_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    published_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_coaching_context: std::option::Option<std::vec::Vec<DialogflowGenerator_AgentCoachingContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    free_form_context: std::option::Option<std::vec::Vec<DialogflowGenerator_FreeFormContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_parameter: std::option::Option<std::vec::Vec<DialogflowGenerator_InferenceParameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_context: std::option::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowGenerator_Timeouts>,
    dynamic: DialogflowGeneratorDynamic,
}

struct DialogflowGenerator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowGeneratorData_>,
}

#[derive(Clone)]
pub struct DialogflowGenerator(std::rc::Rc<DialogflowGenerator_>);

impl DialogflowGenerator {
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

    #[doc = "Set the field `description`.\nOptional. Human readable description of the generator."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `generator_id`.\nOptional. The ID to use for the generator, which will become the final component of the generator's resource name."]
    pub fn set_generator_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().generator_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `published_model`.\nOptional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: text-bison * To use a stable model version, specify the version number as well. Example: text-bison@002."]
    pub fn set_published_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().published_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tools`.\nOptional. Resource names of the tools that the generator can choose from. Format: 'projects/<Project ID>/locations/<Location ID>/tools/<tool ID>'."]
    pub fn set_tools(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trigger_event`.\nOptional. The trigger event of the generator. It defines when the generator is triggered in a conversation. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\"]"]
    pub fn set_trigger_event(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().trigger_event = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_coaching_context`.\n"]
    pub fn set_agent_coaching_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowGenerator_AgentCoachingContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_coaching_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_coaching_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `free_form_context`.\n"]
    pub fn set_free_form_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowGenerator_FreeFormContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().free_form_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.free_form_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_parameter`.\n"]
    pub fn set_inference_parameter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowGenerator_InferenceParameter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `summarization_context`.\n"]
    pub fn set_summarization_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowGenerator_SummarizationContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().summarization_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.summarization_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowGenerator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Human readable description of the generator."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generator_id` after provisioning.\nOptional. The ID to use for the generator, which will become the final component of the generator's resource name."]
    pub fn generator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\ndesc"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the generator."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `published_model` after provisioning.\nOptional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: text-bison * To use a stable model version, specify the version number as well. Example: text-bison@002."]
    pub fn published_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.published_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nOptional. Resource names of the tools that the generator can choose from. Format: 'projects/<Project ID>/locations/<Location ID>/tools/<tool ID>'."]
    pub fn tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_event` after provisioning.\nOptional. The trigger event of the generator. It defines when the generator is triggered in a conversation. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\"]"]
    pub fn trigger_event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_event", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_coaching_context` after provisioning.\n"]
    pub fn agent_coaching_context(&self) -> terrars::ListRef<DialogflowGenerator_AgentCoachingContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_coaching_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `free_form_context` after provisioning.\n"]
    pub fn free_form_context(&self) -> terrars::ListRef<DialogflowGenerator_FreeFormContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.free_form_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_parameter` after provisioning.\n"]
    pub fn inference_parameter(&self) -> terrars::ListRef<DialogflowGenerator_InferenceParameterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_parameter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summarization_context` after provisioning.\n"]
    pub fn summarization_context(&self) -> terrars::ListRef<DialogflowGenerator_SummarizationContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowGenerator_TimeoutsRef {
        DialogflowGenerator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowGenerator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowGenerator { }

impl terrars::ToListMappable for DialogflowGenerator {
    type O = terrars::ListRef<DialogflowGeneratorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowGenerator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_generator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowGenerator {
    pub tf_id: std::string::String,
    #[doc = "desc"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowGenerator {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowGenerator {
        let out = DialogflowGenerator(std::rc::Rc::new(DialogflowGenerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowGeneratorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                generator_id: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                published_model: core::default::Default::default(),
                tools: core::default::Default::default(),
                trigger_event: core::default::Default::default(),
                agent_coaching_context: core::default::Default::default(),
                free_form_context: core::default::Default::default(),
                inference_parameter: core::default::Default::default(),
                summarization_context: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowGeneratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGeneratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowGeneratorRef {
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
        "Get a reference to the value of field `description` after provisioning.\nOptional. Human readable description of the generator."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generator_id` after provisioning.\nOptional. The ID to use for the generator, which will become the final component of the generator's resource name."]
    pub fn generator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\ndesc"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the generator."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `published_model` after provisioning.\nOptional. The published Large Language Model name. * To use the latest model version, specify the model name without version number. Example: text-bison * To use a stable model version, specify the version number as well. Example: text-bison@002."]
    pub fn published_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.published_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tools` after provisioning.\nOptional. Resource names of the tools that the generator can choose from. Format: 'projects/<Project ID>/locations/<Location ID>/tools/<tool ID>'."]
    pub fn tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_event` after provisioning.\nOptional. The trigger event of the generator. It defines when the generator is triggered in a conversation. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\"]"]
    pub fn trigger_event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_event", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_coaching_context` after provisioning.\n"]
    pub fn agent_coaching_context(&self) -> terrars::ListRef<DialogflowGenerator_AgentCoachingContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_coaching_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `free_form_context` after provisioning.\n"]
    pub fn free_form_context(&self) -> terrars::ListRef<DialogflowGenerator_FreeFormContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.free_form_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_parameter` after provisioning.\n"]
    pub fn inference_parameter(&self) -> terrars::ListRef<DialogflowGenerator_InferenceParameterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_parameter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summarization_context` after provisioning.\n"]
    pub fn summarization_context(&self) -> terrars::ListRef<DialogflowGenerator_SummarizationContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_context", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowGenerator_TimeoutsRef {
        DialogflowGenerator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGeneratorDynamic {
    agent_coaching_context: std::option::Option<terrars::DynamicBlock<DialogflowGenerator_AgentCoachingContext>>,
    free_form_context: std::option::Option<terrars::DynamicBlock<DialogflowGenerator_FreeFormContext>>,
    inference_parameter: std::option::Option<terrars::DynamicBlock<DialogflowGenerator_InferenceParameter>>,
    summarization_context: std::option::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext>>,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_AgentCoachingContext_Instructions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggering_event: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowGenerator_AgentCoachingContext_Instructions {
    #[doc = "Set the field `agent_action`.\nOptional. The action that human agent should take."]
    pub fn set_agent_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\nOptional. The condition of the instruction."]
    pub fn set_condition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_details`.\nOptional. The detailed description of this instruction."]
    pub fn set_display_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nOptional. Display name for the instruction."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_action`.\nOptional. The action that system should take."]
    pub fn set_system_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `triggering_event`.\nOptional. The trigger event of the instruction. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\", \"TOOL_CALL_COMPLETION\"]"]
    pub fn set_triggering_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.triggering_event = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_AgentCoachingContext_Instructions {
    type O = terrars::BlockAssignable<DialogflowGenerator_AgentCoachingContext_Instructions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_AgentCoachingContext_Instructions {}

impl BuildDialogflowGenerator_AgentCoachingContext_Instructions {
    pub fn build(self) -> DialogflowGenerator_AgentCoachingContext_Instructions {
        DialogflowGenerator_AgentCoachingContext_Instructions {
            agent_action: core::default::Default::default(),
            condition: core::default::Default::default(),
            display_details: core::default::Default::default(),
            display_name: core::default::Default::default(),
            system_action: core::default::Default::default(),
            triggering_event: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_AgentCoachingContext_InstructionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_AgentCoachingContext_InstructionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_AgentCoachingContext_InstructionsRef {
        DialogflowGenerator_AgentCoachingContext_InstructionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_AgentCoachingContext_InstructionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_action` after provisioning.\nOptional. The action that human agent should take."]
    pub fn agent_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nOptional. The condition of the instruction."]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_details` after provisioning.\nOptional. The detailed description of this instruction."]
    pub fn display_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. Display name for the instruction."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `system_action` after provisioning.\nOptional. The action that system should take."]
    pub fn system_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `triggering_event` after provisioning.\nOptional. The trigger event of the instruction. Possible values: [\"END_OF_UTTERANCE\", \"MANUAL_CALL\", \"CUSTOMER_MESSAGE\", \"AGENT_MESSAGE\", \"TOOL_CALL_COMPLETION\"]"]
    pub fn triggering_event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.triggering_event", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_AgentCoachingContextDynamic {
    instructions: std::option::Option<terrars::DynamicBlock<DialogflowGenerator_AgentCoachingContext_Instructions>>,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_AgentCoachingContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overarching_guidance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instructions: std::option::Option<std::vec::Vec<DialogflowGenerator_AgentCoachingContext_Instructions>>,
    dynamic: DialogflowGenerator_AgentCoachingContextDynamic,
}

impl DialogflowGenerator_AgentCoachingContext {
    #[doc = "Set the field `output_language_code`.\nOptional. Output language code."]
    pub fn set_output_language_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `overarching_guidance`.\nOptional. The overarching guidance for the agent coaching. This should be set only for v1.5 and later versions."]
    pub fn set_overarching_guidance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.overarching_guidance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nOptional. Version of the feature. If not set, default to latest version. Current candidates are [\"2.5\"]."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instructions`.\n"]
    pub fn set_instructions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowGenerator_AgentCoachingContext_Instructions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instructions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instructions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_AgentCoachingContext {
    type O = terrars::BlockAssignable<DialogflowGenerator_AgentCoachingContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_AgentCoachingContext {}

impl BuildDialogflowGenerator_AgentCoachingContext {
    pub fn build(self) -> DialogflowGenerator_AgentCoachingContext {
        DialogflowGenerator_AgentCoachingContext {
            output_language_code: core::default::Default::default(),
            overarching_guidance: core::default::Default::default(),
            version: core::default::Default::default(),
            instructions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_AgentCoachingContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_AgentCoachingContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowGenerator_AgentCoachingContextRef {
        DialogflowGenerator_AgentCoachingContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_AgentCoachingContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_language_code` after provisioning.\nOptional. Output language code."]
    pub fn output_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `overarching_guidance` after provisioning.\nOptional. The overarching guidance for the agent coaching. This should be set only for v1.5 and later versions."]
    pub fn overarching_guidance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overarching_guidance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional. Version of the feature. If not set, default to latest version. Current candidates are [\"2.5\"]."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> terrars::ListRef<DialogflowGenerator_AgentCoachingContext_InstructionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instructions", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_FreeFormContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowGenerator_FreeFormContext {
    #[doc = "Set the field `text`.\nOptional. Free form text input to LLM."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_FreeFormContext {
    type O = terrars::BlockAssignable<DialogflowGenerator_FreeFormContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_FreeFormContext {}

impl BuildDialogflowGenerator_FreeFormContext {
    pub fn build(self) -> DialogflowGenerator_FreeFormContext {
        DialogflowGenerator_FreeFormContext { text: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_FreeFormContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_FreeFormContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowGenerator_FreeFormContextRef {
        DialogflowGenerator_FreeFormContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_FreeFormContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nOptional. Free form text input to LLM."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_InferenceParameter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_output_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl DialogflowGenerator_InferenceParameter {
    #[doc = "Set the field `max_output_tokens`.\nOptional. Maximum number of the output tokens for the generator."]
    pub fn set_max_output_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_output_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temperature`.\nOptional. Controls the randomness of LLM predictions. Low temperature = less random. High temperature = more random. If unset (or 0), uses a default value of 0."]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `top_k`.\nOptional. Top-k changes how the model selects tokens for output. A top-k of 1 means the selected token is the most probable among all tokens in the model's vocabulary (also called greedy decoding), while a top-k of 3 means that the next token is selected from among the 3 most probable tokens (using temperature). For each token selection step, the top K tokens with the highest probabilities are sampled. Then tokens are further filtered based on topP with the final token selected using temperature sampling. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [1, 40], default to 40."]
    pub fn set_top_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_k = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `top_p`.\nOptional. Top-p changes how the model selects tokens for output. Tokens are selected from most K (see topK parameter) probable to least until the sum of their probabilities equals the top-p value. For example, if tokens A, B, and C have a probability of 0.3, 0.2, and 0.1 and the top-p value is 0.5, then the model will select either A or B as the next token (using temperature) and doesn't consider C. The default top-p value is 0.95. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [0.0, 1.0], default to 0.95."]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_InferenceParameter {
    type O = terrars::BlockAssignable<DialogflowGenerator_InferenceParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_InferenceParameter {}

impl BuildDialogflowGenerator_InferenceParameter {
    pub fn build(self) -> DialogflowGenerator_InferenceParameter {
        DialogflowGenerator_InferenceParameter {
            max_output_tokens: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_k: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_InferenceParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_InferenceParameterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowGenerator_InferenceParameterRef {
        DialogflowGenerator_InferenceParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_InferenceParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_output_tokens` after provisioning.\nOptional. Maximum number of the output tokens for the generator."]
    pub fn max_output_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_output_tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temperature` after provisioning.\nOptional. Controls the randomness of LLM predictions. Low temperature = less random. High temperature = more random. If unset (or 0), uses a default value of 0."]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `top_k` after provisioning.\nOptional. Top-k changes how the model selects tokens for output. A top-k of 1 means the selected token is the most probable among all tokens in the model's vocabulary (also called greedy decoding), while a top-k of 3 means that the next token is selected from among the 3 most probable tokens (using temperature). For each token selection step, the top K tokens with the highest probabilities are sampled. Then tokens are further filtered based on topP with the final token selected using temperature sampling. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [1, 40], default to 40."]
    pub fn top_k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_k", self.base))
    }

    #[doc =
        "Get a reference to the value of field `top_p` after provisioning.\nOptional. Top-p changes how the model selects tokens for output. Tokens are selected from most K (see topK parameter) probable to least until the sum of their probabilities equals the top-p value. For example, if tokens A, B, and C have a probability of 0.3, 0.2, and 0.1 and the top-p value is 0.5, then the model will select either A or B as the next token (using temperature) and doesn't consider C. The default top-p value is 0.95. Specify a lower value for less random responses and a higher value for more random responses. Acceptable value is [0.0, 1.0], default to 0.95."]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {
    #[doc = "Set the field `create_time`.\nOptional. Create time of the message entry."]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code`.\nOptional. The language of the text."]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `role`.\nOptional. Participant role of the message. Possible values: [\"HUMAN_AGENT\", \"AUTOMATED_AGENT\", \"END_USER\"]"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nOptional. Transcript content of the message."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {
    pub fn build(
        self,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {
        DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries {
            create_time: core::default::Default::default(),
            language_code: core::default::Default::default(),
            role: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntriesRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOptional. Create time of the message entry."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nOptional. The language of the text."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nOptional. Participant role of the message. Possible values: [\"HUMAN_AGENT\", \"AUTOMATED_AGENT\", \"END_USER\"]"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nOptional. Transcript content of the message."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextDynamic {
    message_entries: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_entries: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries>,
    >,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {
    #[doc = "Set the field `message_entries`.\n"]
    pub fn set_message_entries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.message_entries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.message_entries = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {
    type O = terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {
        DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext {
            message_entries: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_entries` after provisioning.\n"]
    pub fn message_entries(
        &self,
    ) -> terrars
    ::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext_MessageEntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_entries", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections {
    section: terrars::PrimField<std::string::String>,
    summary: terrars::PrimField<std::string::String>,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections { }

impl terrars
::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections {
    #[doc = "Required. Name of the section."]
    pub section: terrars::PrimField<std::string::String>,
    #[doc = "Required. Summary text for the section."]
    pub summary: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections {
    pub fn build(
        self,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections {
            section: self.section,
            summary: self.summary,
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySectionsRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `section` after provisioning.\nRequired. Name of the section."]
    pub fn section(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.section", self.base))
    }

    #[doc =
        "Get a reference to the value of field `summary` after provisioning.\nRequired. Summary text for the section."]
    pub fn summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionDynamic {
    summary_sections: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary_sections: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections>,
    >,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {
    #[doc = "Set the field `summary_sections`.\n"]
    pub fn set_summary_sections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySections,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summary_sections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summary_sections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {
    type O =
        terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion {
            summary_sections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `summary_sections` after provisioning.\n"]
    pub fn summary_sections(
        &self,
    ) -> terrars
    ::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion_SummarySectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summary_sections", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {
    #[doc = "Set the field `action`.\nThe name of the tool's action associated with this call."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool`.\nThe tool associated with this call."]
    pub fn set_tool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {
    type O =
        terrars
        ::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall {
            action: core::default::Default::default(),
            tool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe name of the tool's action associated with this call."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\nThe tool associated with this call."]
    pub fn tool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retryable: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {
    #[doc = "Set the field `message`.\nThe error message of the function."]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retryable`.\nSpecifies whether the tool call is retryable."]
    pub fn set_retryable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retryable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {
    pub fn build(
        self,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error {
            message: core::default::Default::default(),
            retryable: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_ErrorRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\nThe error message of the function."]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retryable` after provisioning.\nSpecifies whether the tool call is retryable."]
    pub fn retryable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retryable", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultDynamic {
    error: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error,
        >,
    >,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error>,
    >,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {
    #[doc = "Set the field `action`.\nThe name of the tool's action associated with this call."]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_Error,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {
    pub fn build(
        self,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult {
            action: core::default::Default::default(),
            error: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe name of the tool's action associated with this call."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> terrars
    ::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoDynamic {
    tool_call: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall>,
    >,
    tool_call_result: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_call: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_call_result: std
    ::option
    ::Option<
        std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult>,
    >,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {
    #[doc = "Set the field `tool_call`.\n"]
    pub fn set_tool_call(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCall,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_call_result`.\n"]
    pub fn set_tool_call_result(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResult,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_call_result = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_call_result = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {
    type O = terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo {
            tool_call: core::default::Default::default(),
            tool_call_result: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool_call` after provisioning.\n"]
    pub fn tool_call(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_call", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_call_result` after provisioning.\n"]
    pub fn tool_call_result(
        &self,
    ) -> terrars
    ::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo_ToolCallResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_call_result", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamples_OutputDynamic {
    summary_suggestion: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion>,
    >,
    tool_call_info: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo>>,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_Output {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary_suggestion: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_call_info: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo>>,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamples_OutputDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_Output {
    #[doc = "Set the field `summary_suggestion`.\n"]
    pub fn set_summary_suggestion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestion,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summary_suggestion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summary_suggestion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_call_info`.\n"]
    pub fn set_tool_call_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_call_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_call_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_Output {
    type O = terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_Output>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_Output {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples_Output {
        DialogflowGenerator_SummarizationContext_FewShotExamples_Output {
            summary_suggestion: core::default::Default::default(),
            tool_call_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_OutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_OutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_OutputRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_OutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_OutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `summary_suggestion` after provisioning.\n"]
    pub fn summary_suggestion(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_SummarySuggestionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summary_suggestion", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_call_info` after provisioning.\n"]
    pub fn tool_call_info(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_Output_ToolCallInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_call_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_SummarizationSections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowGenerator_SummarizationContext_SummarizationSections {
    #[doc =
        "Set the field `definition`.\nOptional. Definition of the section, for example, \"what the customer needs help with or has question about.\""]
    pub fn set_definition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nOptional. Name of the section, for example, \"situation\"."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. Type of the summarization section. Possible values: [\"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\", \"CUSTOMER_DEFINED\", \"SITUATION_CONCISE\", \"ACTION_CONCISE\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_SummarizationSections {
    type O = terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_SummarizationSections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_SummarizationSections {}

impl BuildDialogflowGenerator_SummarizationContext_SummarizationSections {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_SummarizationSections {
        DialogflowGenerator_SummarizationContext_SummarizationSections {
            definition: core::default::Default::default(),
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_SummarizationSectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_SummarizationSectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_SummarizationSectionsRef {
        DialogflowGenerator_SummarizationContext_SummarizationSectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_SummarizationSectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `definition` after provisioning.\nOptional. Definition of the section, for example, \"what the customer needs help with or has question about.\""]
    pub fn definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nOptional. Name of the section, for example, \"situation\"."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. Type of the summarization section. Possible values: [\"SITUATION\", \"ACTION\", \"RESOLUTION\", \"REASON_FOR_CANCELLATION\", \"CUSTOMER_SATISFACTION\", \"ENTITIES\", \"CUSTOMER_DEFINED\", \"SITUATION_CONCISE\", \"ACTION_CONCISE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListDynamic {
    summarization_sections: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_SummarizationSections>>,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_sections: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_SummarizationSections>>,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {
    #[doc = "Set the field `summarization_sections`.\n"]
    pub fn set_summarization_sections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_SummarizationSections>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summarization_sections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_sections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {
    type O =
        terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {
        DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList {
            summarization_sections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListRef {
        DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `summarization_sections` after provisioning.\n"]
    pub fn summarization_sections(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_SummarizationSectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_sections", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContext_FewShotExamplesDynamic {
    conversation_context: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext>>,
    output: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_Output>>,
    summarization_section_list: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamples {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_info: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_context: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output: std::option::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_Output>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_section_list: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList>>,
    dynamic: DialogflowGenerator_SummarizationContext_FewShotExamplesDynamic,
}

impl DialogflowGenerator_SummarizationContext_FewShotExamples {
    #[doc =
        "Set the field `extra_info`.\nOptional. Key is the placeholder field name in input, value is the value of the placeholder. E.g. instruction contains \"@price\", and ingested data has <\"price\", \"10\">"]
    pub fn set_extra_info(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.extra_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_context`.\n"]
    pub fn set_conversation_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContext,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output`.\n"]
    pub fn set_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples_Output>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `summarization_section_list`.\n"]
    pub fn set_summarization_section_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summarization_section_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_section_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext_FewShotExamples {
    type O = terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext_FewShotExamples {}

impl BuildDialogflowGenerator_SummarizationContext_FewShotExamples {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext_FewShotExamples {
        DialogflowGenerator_SummarizationContext_FewShotExamples {
            extra_info: core::default::Default::default(),
            conversation_context: core::default::Default::default(),
            output: core::default::Default::default(),
            summarization_section_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContext_FewShotExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContext_FewShotExamplesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowGenerator_SummarizationContext_FewShotExamplesRef {
        DialogflowGenerator_SummarizationContext_FewShotExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContext_FewShotExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `extra_info` after provisioning.\nOptional. Key is the placeholder field name in input, value is the value of the placeholder. E.g. instruction contains \"@price\", and ingested data has <\"price\", \"10\">"]
    pub fn extra_info(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.extra_info", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_context` after provisioning.\n"]
    pub fn conversation_context(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_ConversationContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_context", self.base))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\n"]
    pub fn output(&self) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_OutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization_section_list` after provisioning.\n"]
    pub fn summarization_section_list(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamples_SummarizationSectionListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_section_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowGenerator_SummarizationContextDynamic {
    few_shot_examples: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_FewShotExamples>>,
    summarization_sections: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowGenerator_SummarizationContext_SummarizationSections>>,
}

#[derive(Serialize)]
pub struct DialogflowGenerator_SummarizationContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    few_shot_examples: std::option::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_FewShotExamples>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_sections: std
    ::option
    ::Option<std::vec::Vec<DialogflowGenerator_SummarizationContext_SummarizationSections>>,
    dynamic: DialogflowGenerator_SummarizationContextDynamic,
}

impl DialogflowGenerator_SummarizationContext {
    #[doc =
        "Set the field `output_language_code`.\nOptional. The target language of the generated summary. The language code for conversation will be used if this field is empty. Supported 2.0 and later versions."]
    pub fn set_output_language_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nOptional. Version of the feature. If not set, default to latest version. Current candidates are [\"5.0\", \"6.0\"]."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `few_shot_examples`.\n"]
    pub fn set_few_shot_examples(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_FewShotExamples>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.few_shot_examples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.few_shot_examples = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `summarization_sections`.\n"]
    pub fn set_summarization_sections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowGenerator_SummarizationContext_SummarizationSections>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summarization_sections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_sections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowGenerator_SummarizationContext {
    type O = terrars::BlockAssignable<DialogflowGenerator_SummarizationContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_SummarizationContext {}

impl BuildDialogflowGenerator_SummarizationContext {
    pub fn build(self) -> DialogflowGenerator_SummarizationContext {
        DialogflowGenerator_SummarizationContext {
            output_language_code: core::default::Default::default(),
            version: core::default::Default::default(),
            few_shot_examples: core::default::Default::default(),
            summarization_sections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_SummarizationContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_SummarizationContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowGenerator_SummarizationContextRef {
        DialogflowGenerator_SummarizationContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_SummarizationContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_language_code` after provisioning.\nOptional. The target language of the generated summary. The language code for conversation will be used if this field is empty. Supported 2.0 and later versions."]
    pub fn output_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional. Version of the feature. If not set, default to latest version. Current candidates are [\"5.0\", \"6.0\"]."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `few_shot_examples` after provisioning.\n"]
    pub fn few_shot_examples(&self) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_FewShotExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.few_shot_examples", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization_sections` after provisioning.\n"]
    pub fn summarization_sections(
        &self,
    ) -> terrars::ListRef<DialogflowGenerator_SummarizationContext_SummarizationSectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_sections", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowGenerator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowGenerator_Timeouts {
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

impl terrars::ToListMappable for DialogflowGenerator_Timeouts {
    type O = terrars::BlockAssignable<DialogflowGenerator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowGenerator_Timeouts {}

impl BuildDialogflowGenerator_Timeouts {
    pub fn build(self) -> DialogflowGenerator_Timeouts {
        DialogflowGenerator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowGenerator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowGenerator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowGenerator_TimeoutsRef {
        DialogflowGenerator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowGenerator_TimeoutsRef {
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
