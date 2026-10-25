#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxTestCaseData_ {
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
    notes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_case_conversation_turns: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_config: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxTestCase_Timeouts>,
    dynamic: DialogflowCxTestCaseDynamic,
}

struct DialogflowCxTestCase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxTestCaseData_>,
}

#[derive(Clone)]
pub struct DialogflowCxTestCase(std::rc::Rc<DialogflowCxTestCase_>);

impl DialogflowCxTestCase {
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

    #[doc = "Set the field `notes`.\nAdditional freeform notes about the test case. Limit of 400 characters."]
    pub fn set_notes(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().notes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe agent to create the test case for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nTags are short descriptions that users may apply to test cases for organizational and filtering purposes.\nEach tag should start with \"#\" and has a limit of 30 characters"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_case_conversation_turns`.\n"]
    pub fn set_test_case_conversation_turns(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test_case_conversation_turns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.test_case_conversation_turns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `test_config`.\n"]
    pub fn set_test_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTestCase_TestConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.test_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxTestCase_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nWhen the test was created. A timestamp in RFC3339 text format."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_test_result` after provisioning.\nThe latest test result."]
    pub fn last_test_result(&self) -> terrars::ListRef<DialogflowCxTestCase_LastTestResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_test_result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the test case.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/testCases/<TestCase ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notes` after provisioning.\nAdditional freeform notes about the test case. Limit of 400 characters."]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create the test case for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nTags are short descriptions that users may apply to test cases for organizational and filtering purposes.\nEach tag should start with \"#\" and has a limit of 30 characters"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_case_conversation_turns` after provisioning.\n"]
    pub fn test_case_conversation_turns(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_case_conversation_turns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_config` after provisioning.\n"]
    pub fn test_config(&self) -> terrars::ListRef<DialogflowCxTestCase_TestConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTestCase_TimeoutsRef {
        DialogflowCxTestCase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DialogflowCxTestCase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxTestCase { }

impl terrars::ToListMappable for DialogflowCxTestCase {
    type O = terrars::ListRef<DialogflowCxTestCaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxTestCase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_test_case".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxTestCase {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTestCase {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxTestCase {
        let out = DialogflowCxTestCase(std::rc::Rc::new(DialogflowCxTestCase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxTestCaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                notes: core::default::Default::default(),
                parent: core::default::Default::default(),
                tags: core::default::Default::default(),
                test_case_conversation_turns: core::default::Default::default(),
                test_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxTestCaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxTestCaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nWhen the test was created. A timestamp in RFC3339 text format."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_test_result` after provisioning.\nThe latest test result."]
    pub fn last_test_result(&self) -> terrars::ListRef<DialogflowCxTestCase_LastTestResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_test_result", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the test case.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/testCases/<TestCase ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notes` after provisioning.\nAdditional freeform notes about the test case. Limit of 400 characters."]
    pub fn notes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe agent to create the test case for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nTags are short descriptions that users may apply to test cases for organizational and filtering purposes.\nEach tag should start with \"#\" and has a limit of 30 characters"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_case_conversation_turns` after provisioning.\n"]
    pub fn test_case_conversation_turns(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_case_conversation_turns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_config` after provisioning.\n"]
    pub fn test_config(&self) -> terrars::ListRef<DialogflowCxTestCase_TestConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTestCase_TimeoutsRef {
        DialogflowCxTestCase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCaseDynamic {
    test_case_conversation_turns: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns>>,
    test_config: std::option::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestConfig>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digits: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {
    #[doc = "Set the field `digits`.\n"]
    pub fn set_digits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finish_digit`.\n"]
    pub fn set_finish_digit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finish_digit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf {
            digits: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_DtmfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_DtmfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_DtmfRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_DtmfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_DtmfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `digits` after provisioning.\n"]
    pub fn digits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digits", self.base))
    }

    #[doc = "Get a reference to the value of field `finish_digit` after provisioning.\n"]
    pub fn finish_digit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {
    #[doc = "Set the field `event`.\n"]
    pub fn set_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event {
            event: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_EventRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_EventRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_EventRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_EventRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_EventRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text {
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_TextRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_TextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {
    #[doc = "Set the field `dtmf`.\n"]
    pub fn set_dtmf(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Dtmf>,
                    >,
    ) -> Self {
        self.dtmf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event`.\n"]
    pub fn set_event(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Event>,
                    >,
    ) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
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
                        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_Text>,
                    >,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input {
            dtmf: core::default::Default::default(),
            event: core::default::Default::default(),
            language_code: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_InputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_InputRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dtmf` after provisioning.\n"]
    pub fn dtmf(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_DtmfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_EventRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sentiment_analysis: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    injected_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_webhook_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {
    #[doc = "Set the field `enable_sentiment_analysis`.\n"]
    pub fn set_enable_sentiment_analysis(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_sentiment_analysis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `injected_parameters`.\n"]
    pub fn set_injected_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.injected_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_Input>>,
    ) -> Self {
        self.input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_webhook_enabled`.\n"]
    pub fn set_is_webhook_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_webhook_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput {
            enable_sentiment_analysis: core::default::Default::default(),
            injected_parameters: core::default::Default::default(),
            input: core::default::Default::default(),
            is_webhook_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInputRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_sentiment_analysis` after provisioning.\n"]
    pub fn enable_sentiment_analysis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sentiment_analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `injected_parameters` after provisioning.\n"]
    pub fn injected_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.injected_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `is_webhook_enabled` after provisioning.\n"]
    pub fn is_webhook_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_webhook_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPageRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences {
            description: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_DifferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_DifferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_DifferencesRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_DifferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_DifferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_StatusRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses {
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponsesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponsesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponsesRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponsesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponsesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_page: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    differences: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_responses: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggered_intent: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent>,
    >,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {
    #[doc = "Set the field `current_page`.\n"]
    pub fn set_current_page(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPage,
                        >,
                    >,
    ) -> Self {
        self.current_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `differences`.\n"]
    pub fn set_differences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Differences,
                        >,
                    >,
    ) -> Self {
        self.differences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_parameters`.\n"]
    pub fn set_session_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.session_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_Status>,
                    >,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_responses`.\n"]
    pub fn set_text_responses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponses,
                        >,
                    >,
    ) -> Self {
        self.text_responses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `triggered_intent`.\n"]
    pub fn set_triggered_intent(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntent,
                        >,
                    >,
    ) -> Self {
        self.triggered_intent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput {
            current_page: core::default::Default::default(),
            differences: core::default::Default::default(),
            session_parameters: core::default::Default::default(),
            status: core::default::Default::default(),
            text_responses: core::default::Default::default(),
            triggered_intent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutputRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_page` after provisioning.\n"]
    pub fn current_page(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_CurrentPageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_page", self.base))
    }

    #[doc = "Get a reference to the value of field `differences` after provisioning.\n"]
    pub fn differences(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_DifferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.differences", self.base))
    }

    #[doc = "Get a reference to the value of field `session_parameters` after provisioning.\n"]
    pub fn session_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `text_responses` after provisioning.\n"]
    pub fn text_responses(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TextResponsesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_responses", self.base))
    }

    #[doc = "Get a reference to the value of field `triggered_intent` after provisioning.\n"]
    pub fn triggered_intent(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput_TriggeredIntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggered_intent", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_input: std::option::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_agent_output: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput>>,
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurns {
    #[doc = "Set the field `user_input`.\n"]
    pub fn set_user_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInput>>,
    ) -> Self {
        self.user_input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_agent_output`.\n"]
    pub fn set_virtual_agent_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutput>,
                    >,
    ) -> Self {
        self.virtual_agent_output = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult_ConversationTurns {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult_ConversationTurns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult_ConversationTurns {}

impl BuildDialogflowCxTestCase_LastTestResult_ConversationTurns {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult_ConversationTurns {
        DialogflowCxTestCase_LastTestResult_ConversationTurns {
            user_input: core::default::Default::default(),
            virtual_agent_output: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResult_ConversationTurnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResult_ConversationTurnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_LastTestResult_ConversationTurnsRef {
        DialogflowCxTestCase_LastTestResult_ConversationTurnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResult_ConversationTurnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_input` after provisioning.\n"]
    pub fn user_input(&self) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_UserInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_input", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_agent_output` after provisioning.\n"]
    pub fn virtual_agent_output(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurns_VirtualAgentOutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_agent_output", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_LastTestResult {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_turns: std::option::Option<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_LastTestResult {
    #[doc = "Set the field `conversation_turns`.\n"]
    pub fn set_conversation_turns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DialogflowCxTestCase_LastTestResult_ConversationTurns>>,
    ) -> Self {
        self.conversation_turns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_result`.\n"]
    pub fn set_test_result(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_time`.\n"]
    pub fn set_test_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_LastTestResult {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_LastTestResult>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_LastTestResult {}

impl BuildDialogflowCxTestCase_LastTestResult {
    pub fn build(self) -> DialogflowCxTestCase_LastTestResult {
        DialogflowCxTestCase_LastTestResult {
            conversation_turns: core::default::Default::default(),
            environment: core::default::Default::default(),
            name: core::default::Default::default(),
            test_result: core::default::Default::default(),
            test_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_LastTestResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_LastTestResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTestCase_LastTestResultRef {
        DialogflowCxTestCase_LastTestResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_LastTestResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conversation_turns` after provisioning.\n"]
    pub fn conversation_turns(&self) -> terrars::ListRef<DialogflowCxTestCase_LastTestResult_ConversationTurnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_turns", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `test_result` after provisioning.\n"]
    pub fn test_result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_result", self.base))
    }

    #[doc = "Get a reference to the value of field `test_time` after provisioning.\n"]
    pub fn test_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digits: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {
    #[doc = "Set the field `digits`.\nThe dtmf digits."]
    pub fn set_digits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finish_digit`.\nThe finish digit (if any)."]
    pub fn set_finish_digit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finish_digit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf {
            digits: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_DtmfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_DtmfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_DtmfRef {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_DtmfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_DtmfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `digits` after provisioning.\nThe dtmf digits."]
    pub fn digits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digits", self.base))
    }

    #[doc = "Get a reference to the value of field `finish_digit` after provisioning.\nThe finish digit (if any)."]
    pub fn finish_digit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finish_digit", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event {
    event: terrars::PrimField<std::string::String>,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event { }

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event {
    #[doc = "Name of the event."]
    pub event: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event { event: self.event }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_EventRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_EventRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_EventRef {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_EventRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_EventRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\nName of the event."]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text {
    text: terrars::PrimField<std::string::String>,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text { }

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text {
    #[doc = "The natural language text to be processed. Text length must not exceed 256 characters."]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text { text: self.text }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_TextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_TextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_TextRef {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_TextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_TextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nThe natural language text to be processed. Text length must not exceed 256 characters."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputDynamic {
    dtmf: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf>>,
    event: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event>>,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text>>,
    dynamic: DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputDynamic,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {
    #[doc =
        "Set the field `language_code`.\nThe language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.\nNote that queries in the same session do not necessarily need to specify the same language."]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dtmf`.\n"]
    pub fn set_dtmf(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Dtmf>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dtmf = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dtmf = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event`.\n"]
    pub fn set_event(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Event>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event = std::option::Option::Some(d);
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
                        terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_Text>,
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

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input {
            language_code: core::default::Default::default(),
            dtmf: core::default::Default::default(),
            event: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputRef {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.\nNote that queries in the same session do not necessarily need to specify the same language."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf` after provisioning.\n"]
    pub fn dtmf(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_DtmfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_EventRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input_TextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCase_TestCaseConversationTurns_UserInputDynamic {
    input: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sentiment_analysis: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    injected_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_webhook_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input>>,
    dynamic: DialogflowCxTestCase_TestCaseConversationTurns_UserInputDynamic,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInput {
    #[doc = "Set the field `enable_sentiment_analysis`.\nWhether sentiment analysis is enabled."]
    pub fn set_enable_sentiment_analysis(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_sentiment_analysis = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `injected_parameters`.\nParameters that need to be injected into the conversation during intent detection."]
    pub fn set_injected_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.injected_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_webhook_enabled`.\nIf webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
    pub fn set_is_webhook_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_webhook_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input`.\n"]
    pub fn set_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_Input>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_UserInput {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_UserInput {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInput {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInput {
            enable_sentiment_analysis: core::default::Default::default(),
            injected_parameters: core::default::Default::default(),
            is_webhook_enabled: core::default::Default::default(),
            input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_UserInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_UserInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_UserInputRef {
        DialogflowCxTestCase_TestCaseConversationTurns_UserInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_UserInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_sentiment_analysis` after provisioning.\nWhether sentiment analysis is enabled."]
    pub fn enable_sentiment_analysis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sentiment_analysis", self.base))
    }

    #[doc =
        "Get a reference to the value of field `injected_parameters` after provisioning.\nParameters that need to be injected into the conversation during intent detection."]
    pub fn injected_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.injected_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_webhook_enabled` after provisioning.\nIf webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
    pub fn is_webhook_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_webhook_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `input` after provisioning.\n"]
    pub fn input(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_UserInput_InputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {
    #[doc =
        "Set the field `name`.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage {
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPageRef {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the page, unique within the flow."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses {
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponsesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponsesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponsesRef {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponsesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponsesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {
    #[doc =
        "Set the field `name`.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent {
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the intent, unique within the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputDynamic {
    current_page: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage>>,
    text_responses: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses>>,
    triggered_intent: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_page: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_responses: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggered_intent: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent>>,
    dynamic: DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputDynamic,
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {
    #[doc = "Set the field `session_parameters`.\nThe session parameters available to the bot at this point."]
    pub fn set_session_parameters(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.session_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `current_page`.\n"]
    pub fn set_current_page(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.current_page = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.current_page = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_responses`.\n"]
    pub fn set_text_responses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponses,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_responses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_responses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `triggered_intent`.\n"]
    pub fn set_triggered_intent(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntent,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.triggered_intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.triggered_intent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput {
            session_parameters: core::default::Default::default(),
            current_page: core::default::Default::default(),
            text_responses: core::default::Default::default(),
            triggered_intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputRef {
        DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `session_parameters` after provisioning.\nThe session parameters available to the bot at this point."]
    pub fn session_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `current_page` after provisioning.\n"]
    pub fn current_page(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_CurrentPageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_page", self.base))
    }

    #[doc = "Get a reference to the value of field `text_responses` after provisioning.\n"]
    pub fn text_responses(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TextResponsesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_responses", self.base))
    }

    #[doc = "Get a reference to the value of field `triggered_intent` after provisioning.\n"]
    pub fn triggered_intent(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput_TriggeredIntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggered_intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCase_TestCaseConversationTurnsDynamic {
    user_input: std::option::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_UserInput>>,
    virtual_agent_output: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestCaseConversationTurns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_input: std::option::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_UserInput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_agent_output: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput>>,
    dynamic: DialogflowCxTestCase_TestCaseConversationTurnsDynamic,
}

impl DialogflowCxTestCase_TestCaseConversationTurns {
    #[doc = "Set the field `user_input`.\n"]
    pub fn set_user_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_UserInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_input = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `virtual_agent_output`.\n"]
    pub fn set_virtual_agent_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutput>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_agent_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_agent_output = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestCaseConversationTurns {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestCaseConversationTurns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestCaseConversationTurns {}

impl BuildDialogflowCxTestCase_TestCaseConversationTurns {
    pub fn build(self) -> DialogflowCxTestCase_TestCaseConversationTurns {
        DialogflowCxTestCase_TestCaseConversationTurns {
            user_input: core::default::Default::default(),
            virtual_agent_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestCaseConversationTurnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestCaseConversationTurnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCase_TestCaseConversationTurnsRef {
        DialogflowCxTestCase_TestCaseConversationTurnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestCaseConversationTurnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_input` after provisioning.\n"]
    pub fn user_input(&self) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_UserInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_input", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_agent_output` after provisioning.\n"]
    pub fn virtual_agent_output(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCase_TestCaseConversationTurns_VirtualAgentOutputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_agent_output", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_TestConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracking_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxTestCase_TestConfig {
    #[doc =
        "Set the field `flow`.\nFlow name to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn set_flow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `page`.\nThe page to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn set_page(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tracking_parameters`.\nSession parameters to be compared when calculating differences."]
    pub fn set_tracking_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tracking_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCase_TestConfig {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_TestConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_TestConfig {}

impl BuildDialogflowCxTestCase_TestConfig {
    pub fn build(self) -> DialogflowCxTestCase_TestConfig {
        DialogflowCxTestCase_TestConfig {
            flow: core::default::Default::default(),
            page: core::default::Default::default(),
            tracking_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TestConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TestConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTestCase_TestConfigRef {
        DialogflowCxTestCase_TestConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TestConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `flow` after provisioning.\nFlow name to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow", self.base))
    }

    #[doc =
        "Get a reference to the value of field `page` after provisioning.\nThe page to start the test case with.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.\nOnly one of flow and page should be set to indicate the starting point of the test case. If neither is set, the test case will start with start page on the default start flow."]
    pub fn page(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tracking_parameters` after provisioning.\nSession parameters to be compared when calculating differences."]
    pub fn tracking_parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tracking_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCase_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCase_Timeouts {
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

impl terrars::ToListMappable for DialogflowCxTestCase_Timeouts {
    type O = terrars::BlockAssignable<DialogflowCxTestCase_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCase_Timeouts {}

impl BuildDialogflowCxTestCase_Timeouts {
    pub fn build(self) -> DialogflowCxTestCase_Timeouts {
        DialogflowCxTestCase_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DialogflowCxTestCase_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCase_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTestCase_TimeoutsRef {
        DialogflowCxTestCase_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCase_TimeoutsRef {
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
