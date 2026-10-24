use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxTestCaseData {
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
    notes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_case_conversation_turns: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_config: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxTestCaseTimeoutsEl>,
    dynamic: DialogflowCxTestCaseDynamic,
}

struct DialogflowCxTestCase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxTestCaseData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxTestCaseTestConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxTestCaseTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nWhen the test was created. A timestamp in RFC3339 text format."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
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
    pub fn last_test_result(&self) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElRef> {
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
    pub fn test_case_conversation_turns(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_case_conversation_turns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_config` after provisioning.\n"]
    pub fn test_config(&self) -> terrars::ListRef<DialogflowCxTestCaseTestConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTestCaseTimeoutsElRef {
        DialogflowCxTestCaseTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DialogflowCxTestCaseData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the test case, unique within the agent. Limit of 200 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_test_result` after provisioning.\nThe latest test result."]
    pub fn last_test_result(&self) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElRef> {
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
    pub fn test_case_conversation_turns(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_case_conversation_turns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test_config` after provisioning.\n"]
    pub fn test_config(&self) -> terrars::ListRef<DialogflowCxTestCaseTestConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxTestCaseTimeoutsElRef {
        DialogflowCxTestCaseTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digits: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
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

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl {
            digits: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef {
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
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    #[doc = "Set the field `event`.\n"]
    pub fn set_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl {
            event: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    #[doc = "Set the field `dtmf`.\n"]
    pub fn set_dtmf(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfEl>,
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
                        ::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventEl>,
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
                        terrars
                        ::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextEl>,
                    >,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl {
            dtmf: core::default::Default::default(),
            event: core::default::Default::default(),
            language_code: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dtmf` after provisioning.\n"]
    pub fn dtmf(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElDtmfElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElEventElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sentiment_analysis: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    injected_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_webhook_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
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
                    ::Into<
                        terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputEl>,
                    >,
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

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl {
            enable_sentiment_analysis: core::default::Default::default(),
            injected_parameters: core::default::Default::default(),
            input: core::default::Default::default(),
            is_webhook_enabled: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef {
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
    pub fn input(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }

    #[doc = "Get a reference to the value of field `is_webhook_enabled` after provisioning.\n"]
    pub fn is_webhook_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_webhook_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
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
::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
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
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
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

impl terrars
::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl {
            description: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef {
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
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
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

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef {
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
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
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
::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
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
::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    type O =
        terrars
        ::BlockAssignable<
            DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    pub fn build(
        self,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
            display_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
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
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_page: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    differences: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_responses: std
    ::option
    ::Option<
        terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggered_intent: std
    ::option
    ::Option<
        terrars
        ::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl>,
    >,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
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
                            DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageEl,
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
                            DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesEl,
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
                        ::ListField<
                            DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusEl,
                        >,
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
                            DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesEl,
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
                            DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentEl,
                        >,
                    >,
    ) -> Self {
        self.triggered_intent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl {
            current_page: core::default::Default::default(),
            differences: core::default::Default::default(),
            session_parameters: core::default::Default::default(),
            status: core::default::Default::default(),
            text_responses: core::default::Default::default(),
            triggered_intent: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_page` after provisioning.\n"]
    pub fn current_page(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElCurrentPageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_page", self.base))
    }

    #[doc = "Get a reference to the value of field `differences` after provisioning.\n"]
    pub fn differences(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElDifferencesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.differences", self.base))
    }

    #[doc = "Get a reference to the value of field `session_parameters` after provisioning.\n"]
    pub fn session_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `text_responses` after provisioning.\n"]
    pub fn text_responses(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTextResponsesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_responses", self.base))
    }

    #[doc = "Get a reference to the value of field `triggered_intent` after provisioning.\n"]
    pub fn triggered_intent(
        &self,
    ) -> terrars
    ::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggered_intent", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_input: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_agent_output: std
    ::option
    ::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl>>,
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    #[doc = "Set the field `user_input`.\n"]
    pub fn set_user_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputEl>>,
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
                        terrars
                        ::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputEl>,
                    >,
    ) -> Self {
        self.virtual_agent_output = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultElConversationTurnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultElConversationTurnsEl {}

impl BuildDialogflowCxTestCaseLastTestResultElConversationTurnsEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
        DialogflowCxTestCaseLastTestResultElConversationTurnsEl {
            user_input: core::default::Default::default(),
            virtual_agent_output: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
        DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElConversationTurnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_input` after provisioning.\n"]
    pub fn user_input(&self) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElUserInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_input", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_agent_output` after provisioning.\n"]
    pub fn virtual_agent_output(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElVirtualAgentOutputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_agent_output", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseLastTestResultEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_turns: std::option::Option<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseLastTestResultEl {
    #[doc = "Set the field `conversation_turns`.\n"]
    pub fn set_conversation_turns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DialogflowCxTestCaseLastTestResultElConversationTurnsEl>>,
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

impl terrars::ToListMappable for DialogflowCxTestCaseLastTestResultEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseLastTestResultEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseLastTestResultEl {}

impl BuildDialogflowCxTestCaseLastTestResultEl {
    pub fn build(self) -> DialogflowCxTestCaseLastTestResultEl {
        DialogflowCxTestCaseLastTestResultEl {
            conversation_turns: core::default::Default::default(),
            environment: core::default::Default::default(),
            name: core::default::Default::default(),
            test_result: core::default::Default::default(),
            test_time: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseLastTestResultElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseLastTestResultElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTestCaseLastTestResultElRef {
        DialogflowCxTestCaseLastTestResultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseLastTestResultElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conversation_turns` after provisioning.\n"]
    pub fn conversation_turns(&self) -> terrars::ListRef<DialogflowCxTestCaseLastTestResultElConversationTurnsElRef> {
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
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digits: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finish_digit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
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

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl {
            digits: core::default::Default::default(),
            finish_digit: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef {
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
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    event: terrars::PrimField<std::string::String>,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl { }

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    #[doc = "Name of the event."]
    pub event: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl { event: self.event }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\nName of the event."]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    text: terrars::PrimField<std::string::String>,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl { }

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    #[doc = "The natural language text to be processed. Text length must not exceed 256 characters."]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl { text: self.text }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef {
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
struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDynamic {
    dtmf: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>>,
    event: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>>,
    text: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dtmf: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDynamic,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
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
                        terrars
                        ::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfEl>,
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
                        terrars
                        ::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventEl>,
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
                        terrars
                        ::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextEl>,
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

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl {
            language_code: core::default::Default::default(),
            dtmf: core::default::Default::default(),
            event: core::default::Default::default(),
            text: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nThe language of the input. See [Language Support](https://cloud.google.com/dialogflow/cx/docs/reference/language) for a list of the currently supported language codes.\nNote that queries in the same session do not necessarily need to specify the same language."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `dtmf` after provisioning.\n"]
    pub fn dtmf(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElDtmfElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dtmf", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElEventElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElTextElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElDynamic {
    input: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sentiment_analysis: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    injected_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_webhook_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElDynamic,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
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
                    ::Into<
                        terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputEl>,
                    >,
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

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl {
            enable_sentiment_analysis: core::default::Default::default(),
            injected_parameters: core::default::Default::default(),
            is_webhook_enabled: core::default::Default::default(),
            input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef {
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
    pub fn input(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    #[doc =
        "Set the field `name`.\nThe unique identifier of the page.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef {
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
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    #[doc = "Set the field `text`.\nA collection of text responses."]
    pub fn set_text(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    type O =
        terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl {
            text: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nA collection of text responses."]
    pub fn text(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    #[doc =
        "Set the field `name`.\nThe unique identifier of the intent.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    type O =
        terrars
        ::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef {
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
struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElDynamic {
    current_page: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl>>,
    text_responses: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl>,
    >,
    triggered_intent: std
    ::option
    ::Option<
        terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl>,
    >,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_page: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_responses: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggered_intent: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElDynamic,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
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
                            DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageEl,
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
                            DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesEl,
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
                            DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentEl,
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

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl {
            session_parameters: core::default::Default::default(),
            current_page: core::default::Default::default(),
            text_responses: core::default::Default::default(),
            triggered_intent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef {
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
    ) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElCurrentPageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_page", self.base))
    }

    #[doc = "Get a reference to the value of field `text_responses` after provisioning.\n"]
    pub fn text_responses(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTextResponsesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_responses", self.base))
    }

    #[doc = "Get a reference to the value of field `triggered_intent` after provisioning.\n"]
    pub fn triggered_intent(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElTriggeredIntentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.triggered_intent", self.base))
    }
}

#[derive(Serialize, Default)]
struct DialogflowCxTestCaseTestCaseConversationTurnsElDynamic {
    user_input: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>>,
    virtual_agent_output: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>>,
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestCaseConversationTurnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_input: std::option::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_agent_output: std
    ::option
    ::Option<std::vec::Vec<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>>,
    dynamic: DialogflowCxTestCaseTestCaseConversationTurnsElDynamic,
}

impl DialogflowCxTestCaseTestCaseConversationTurnsEl {
    #[doc = "Set the field `user_input`.\n"]
    pub fn set_user_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputEl>>,
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
                        terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputEl>,
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

impl terrars::ToListMappable for DialogflowCxTestCaseTestCaseConversationTurnsEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestCaseConversationTurnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestCaseConversationTurnsEl {}

impl BuildDialogflowCxTestCaseTestCaseConversationTurnsEl {
    pub fn build(self) -> DialogflowCxTestCaseTestCaseConversationTurnsEl {
        DialogflowCxTestCaseTestCaseConversationTurnsEl {
            user_input: core::default::Default::default(),
            virtual_agent_output: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestCaseConversationTurnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestCaseConversationTurnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxTestCaseTestCaseConversationTurnsElRef {
        DialogflowCxTestCaseTestCaseConversationTurnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestCaseConversationTurnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_input` after provisioning.\n"]
    pub fn user_input(&self) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElUserInputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_input", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_agent_output` after provisioning.\n"]
    pub fn virtual_agent_output(
        &self,
    ) -> terrars::ListRef<DialogflowCxTestCaseTestCaseConversationTurnsElVirtualAgentOutputElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_agent_output", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxTestCaseTestConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tracking_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DialogflowCxTestCaseTestConfigEl {
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

impl terrars::ToListMappable for DialogflowCxTestCaseTestConfigEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTestConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTestConfigEl {}

impl BuildDialogflowCxTestCaseTestConfigEl {
    pub fn build(self) -> DialogflowCxTestCaseTestConfigEl {
        DialogflowCxTestCaseTestConfigEl {
            flow: core::default::Default::default(),
            page: core::default::Default::default(),
            tracking_parameters: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTestConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTestConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTestCaseTestConfigElRef {
        DialogflowCxTestCaseTestConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTestConfigElRef {
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
pub struct DialogflowCxTestCaseTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxTestCaseTimeoutsEl {
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

impl terrars::ToListMappable for DialogflowCxTestCaseTimeoutsEl {
    type O = terrars::BlockAssignable<DialogflowCxTestCaseTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxTestCaseTimeoutsEl {}

impl BuildDialogflowCxTestCaseTimeoutsEl {
    pub fn build(self) -> DialogflowCxTestCaseTimeoutsEl {
        DialogflowCxTestCaseTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxTestCaseTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxTestCaseTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxTestCaseTimeoutsElRef {
        DialogflowCxTestCaseTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxTestCaseTimeoutsElRef {
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
struct DialogflowCxTestCaseDynamic {
    test_case_conversation_turns: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestCaseConversationTurnsEl>>,
    test_config: std::option::Option<terrars::DynamicBlock<DialogflowCxTestCaseTestConfigEl>>,
}
