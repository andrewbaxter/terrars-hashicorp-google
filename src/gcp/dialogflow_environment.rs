#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowEnvironmentData_ {
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
    agent_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    environmentid: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fulfillment: std::option::Option<std::vec::Vec<DialogflowEnvironment_Fulfillment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_to_speech_settings: std::option::Option<std::vec::Vec<DialogflowEnvironment_TextToSpeechSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowEnvironment_Timeouts>,
    dynamic: DialogflowEnvironmentDynamic,
}

struct DialogflowEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowEnvironmentData_>,
}

#[derive(Clone)]
pub struct DialogflowEnvironment(std::rc::Rc<DialogflowEnvironment_>);

impl DialogflowEnvironment {
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
        "Set the field `agent_version`.\nThe agent version loaded into this environment. Supported formats:\n- projects/<Project ID>/agent/versions/<Version ID>\n- projects/<Project ID>/locations/<Location ID>/agent/versions/<Version ID>"]
    pub fn set_agent_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe developer-provided description for this environment."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fulfillment`.\n"]
    pub fn set_fulfillment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowEnvironment_Fulfillment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fulfillment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fulfillment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_to_speech_settings`.\n"]
    pub fn set_text_to_speech_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowEnvironment_TextToSpeechSettings>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `agent_version` after provisioning.\nThe agent version loaded into this environment. Supported formats:\n- projects/<Project ID>/agent/versions/<Version ID>\n- projects/<Project ID>/locations/<Location ID>/agent/versions/<Version ID>"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe developer-provided description for this environment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environmentid` after provisioning.\n"]
    pub fn environmentid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environmentid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this agent environment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this environment."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment` after provisioning.\n"]
    pub fn fulfillment(&self) -> terrars::ListRef<DialogflowEnvironment_FulfillmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> terrars::ListRef<DialogflowEnvironment_TextToSpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_to_speech_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowEnvironment_TimeoutsRef {
        DialogflowEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowEnvironment { }

impl terrars::ToListMappable for DialogflowEnvironment {
    type O = terrars::ListRef<DialogflowEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub environmentid: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowEnvironment {
        let out = DialogflowEnvironment(std::rc::Rc::new(DialogflowEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_version: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                environmentid: self.environmentid,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                fulfillment: core::default::Default::default(),
                text_to_speech_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent_version` after provisioning.\nThe agent version loaded into this environment. Supported formats:\n- projects/<Project ID>/agent/versions/<Version ID>\n- projects/<Project ID>/locations/<Location ID>/agent/versions/<Version ID>"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe developer-provided description for this environment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environmentid` after provisioning.\n"]
    pub fn environmentid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environmentid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of this agent environment."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this environment."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fulfillment` after provisioning.\n"]
    pub fn fulfillment(&self) -> terrars::ListRef<DialogflowEnvironment_FulfillmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `text_to_speech_settings` after provisioning.\n"]
    pub fn text_to_speech_settings(&self) -> terrars::ListRef<DialogflowEnvironment_TextToSpeechSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_to_speech_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowEnvironment_TimeoutsRef {
        DialogflowEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowEnvironmentDynamic {
    fulfillment: std::option::Option<terrars::DynamicBlock<DialogflowEnvironment_Fulfillment>>,
    text_to_speech_settings: std::option::Option<terrars::DynamicBlock<DialogflowEnvironment_TextToSpeechSettings>>,
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_Fulfillment_Features {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DialogflowEnvironment_Fulfillment_Features { }

impl terrars::ToListMappable for DialogflowEnvironment_Fulfillment_Features {
    type O = terrars::BlockAssignable<DialogflowEnvironment_Fulfillment_Features>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_Fulfillment_Features {
    #[doc =
        "The type of the feature that enabled for fulfillment. Possible values: [\"TYPE_UNSPECIFIED\", \"SMALLTALK\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowEnvironment_Fulfillment_Features {
    pub fn build(self) -> DialogflowEnvironment_Fulfillment_Features {
        DialogflowEnvironment_Fulfillment_Features { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_Fulfillment_FeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_Fulfillment_FeaturesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowEnvironment_Fulfillment_FeaturesRef {
        DialogflowEnvironment_Fulfillment_FeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_Fulfillment_FeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the feature that enabled for fulfillment. Possible values: [\"TYPE_UNSPECIFIED\", \"SMALLTALK\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_Fulfillment_GenericWebService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowEnvironment_Fulfillment_GenericWebService {
    #[doc = "Set the field `password`.\nThe password for HTTP Basic authentication."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers`.\nThe HTTP request headers to send together with fulfillment requests"]
    pub fn set_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe user name for HTTP Basic authentication."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowEnvironment_Fulfillment_GenericWebService {
    type O = terrars::BlockAssignable<DialogflowEnvironment_Fulfillment_GenericWebService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_Fulfillment_GenericWebService {
    #[doc = "The fulfillment URI for receiving POST requests. It must use https protocol."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowEnvironment_Fulfillment_GenericWebService {
    pub fn build(self) -> DialogflowEnvironment_Fulfillment_GenericWebService {
        DialogflowEnvironment_Fulfillment_GenericWebService {
            password: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_Fulfillment_GenericWebServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_Fulfillment_GenericWebServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowEnvironment_Fulfillment_GenericWebServiceRef {
        DialogflowEnvironment_Fulfillment_GenericWebServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_Fulfillment_GenericWebServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nThe password for HTTP Basic authentication."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with fulfillment requests"]
    pub fn request_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe fulfillment URI for receiving POST requests. It must use https protocol."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe user name for HTTP Basic authentication."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowEnvironment_FulfillmentDynamic {
    features: std::option::Option<terrars::DynamicBlock<DialogflowEnvironment_Fulfillment_Features>>,
    generic_web_service: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowEnvironment_Fulfillment_GenericWebService>>,
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_Fulfillment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features: std::option::Option<std::vec::Vec<DialogflowEnvironment_Fulfillment_Features>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_web_service: std::option::Option<std::vec::Vec<DialogflowEnvironment_Fulfillment_GenericWebService>>,
    dynamic: DialogflowEnvironment_FulfillmentDynamic,
}

impl DialogflowEnvironment_Fulfillment {
    #[doc = "Set the field `display_name`.\nThe human-readable name of the fulfillment, unique within the agent."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe unique identifier of the fulfillment. Supports the following formats:\n- projects/<Project ID>/agent/fulfillment\n- projects/<Project ID>/locations/<Location ID>/agent/fulfillment"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features`.\n"]
    pub fn set_features(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowEnvironment_Fulfillment_Features>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.features = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.features = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowEnvironment_Fulfillment_GenericWebService>>,
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

impl terrars::ToListMappable for DialogflowEnvironment_Fulfillment {
    type O = terrars::BlockAssignable<DialogflowEnvironment_Fulfillment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_Fulfillment {}

impl BuildDialogflowEnvironment_Fulfillment {
    pub fn build(self) -> DialogflowEnvironment_Fulfillment {
        DialogflowEnvironment_Fulfillment {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
            features: core::default::Default::default(),
            generic_web_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_FulfillmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_FulfillmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowEnvironment_FulfillmentRef {
        DialogflowEnvironment_FulfillmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_FulfillmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the fulfillment, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the fulfillment. Supports the following formats:\n- projects/<Project ID>/agent/fulfillment\n- projects/<Project ID>/locations/<Location ID>/agent/fulfillment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> terrars::ListRef<DialogflowEnvironment_Fulfillment_FeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.features", self.base))
    }

    #[doc = "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> terrars::ListRef<DialogflowEnvironment_Fulfillment_GenericWebServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssml_gender: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {
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

impl terrars::ToListMappable for DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {
    type O = terrars::BlockAssignable<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {}

impl BuildDialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {
    pub fn build(self) -> DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {
        DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice {
            name: core::default::Default::default(),
            ssml_gender: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_VoiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_VoiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_VoiceRef {
        DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_VoiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_VoiceRef {
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
struct DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsDynamic {
    voice: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice>>,
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effects_profile_id: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    language: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pitch: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speaking_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_gain_db: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice: std::option::Option<std::vec::Vec<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice>>,
    dynamic: DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsDynamic,
}

impl DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_Voice>,
                    >,
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

impl terrars::ToListMappable for DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
    type O = terrars::BlockAssignable<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
    #[doc = ""]
    pub language: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
    pub fn build(self) -> DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
        DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs {
            effects_profile_id: core::default::Default::default(),
            language: self.language,
            pitch: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
            voice: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsRef {
        DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effects_profile_id` after provisioning.\nAn identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
    pub fn effects_profile_id(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effects_profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
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
    pub fn voice(&self) -> terrars::ListRef<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs_VoiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowEnvironment_TextToSpeechSettingsDynamic {
    synthesize_speech_configs: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs>>,
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_TextToSpeechSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_text_to_speech: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_audio_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_rate_hertz: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synthesize_speech_configs: std
    ::option
    ::Option<std::vec::Vec<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs>>,
    dynamic: DialogflowEnvironment_TextToSpeechSettingsDynamic,
}

impl DialogflowEnvironment_TextToSpeechSettings {
    #[doc =
        "Set the field `enable_text_to_speech`.\nIndicates whether text to speech is enabled. Even when this field is false, other settings in this proto are still retained."]
    pub fn set_enable_text_to_speech(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_text_to_speech = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_audio_encoding`.\nAudio encoding of the synthesized audio content. Possible values: [\"OUTPUT_AUDIO_ENCODING_UNSPECIFIED\", \"OUTPUT_AUDIO_ENCODING_LINEAR_16\", \"OUTPUT_AUDIO_ENCODING_MP3\", \"OUTPUT_AUDIO_ENCODING_MP3_64_KBPS\", \"OUTPUT_AUDIO_ENCODING_OGG_OPUS\", \"OUTPUT_AUDIO_ENCODING_MULAW\", \"OUTPUT_AUDIO_ENCODING_ALAW\"]"]
    pub fn set_output_audio_encoding(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.output_audio_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_rate_hertz`.\nThe synthesis sample rate (in hertz) for this audio."]
    pub fn set_sample_rate_hertz(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_rate_hertz = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `synthesize_speech_configs`.\n"]
    pub fn set_synthesize_speech_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowEnvironment_TextToSpeechSettings_SynthesizeSpeechConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.synthesize_speech_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.synthesize_speech_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowEnvironment_TextToSpeechSettings {
    type O = terrars::BlockAssignable<DialogflowEnvironment_TextToSpeechSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_TextToSpeechSettings {}

impl BuildDialogflowEnvironment_TextToSpeechSettings {
    pub fn build(self) -> DialogflowEnvironment_TextToSpeechSettings {
        DialogflowEnvironment_TextToSpeechSettings {
            enable_text_to_speech: core::default::Default::default(),
            output_audio_encoding: core::default::Default::default(),
            sample_rate_hertz: core::default::Default::default(),
            synthesize_speech_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_TextToSpeechSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_TextToSpeechSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowEnvironment_TextToSpeechSettingsRef {
        DialogflowEnvironment_TextToSpeechSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_TextToSpeechSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_text_to_speech` after provisioning.\nIndicates whether text to speech is enabled. Even when this field is false, other settings in this proto are still retained."]
    pub fn enable_text_to_speech(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_text_to_speech", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_audio_encoding` after provisioning.\nAudio encoding of the synthesized audio content. Possible values: [\"OUTPUT_AUDIO_ENCODING_UNSPECIFIED\", \"OUTPUT_AUDIO_ENCODING_LINEAR_16\", \"OUTPUT_AUDIO_ENCODING_MP3\", \"OUTPUT_AUDIO_ENCODING_MP3_64_KBPS\", \"OUTPUT_AUDIO_ENCODING_OGG_OPUS\", \"OUTPUT_AUDIO_ENCODING_MULAW\", \"OUTPUT_AUDIO_ENCODING_ALAW\"]"]
    pub fn output_audio_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_audio_encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_rate_hertz` after provisioning.\nThe synthesis sample rate (in hertz) for this audio."]
    pub fn sample_rate_hertz(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_rate_hertz", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowEnvironment_Timeouts {
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

impl terrars::ToListMappable for DialogflowEnvironment_Timeouts {
    type O = terrars::BlockAssignable<DialogflowEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowEnvironment_Timeouts {}

impl BuildDialogflowEnvironment_Timeouts {
    pub fn build(self) -> DialogflowEnvironment_Timeouts {
        DialogflowEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowEnvironment_TimeoutsRef {
        DialogflowEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowEnvironment_TimeoutsRef {
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
