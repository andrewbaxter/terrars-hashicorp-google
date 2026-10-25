#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContactCenterInsightsQaQuestionData_ {
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
    abbreviation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    answer_instructions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    qa_scorecard: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    question_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    question_type: std::option::Option<terrars::PrimField<std::string::String>>,
    revision: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    answer_choices: std::option::Option<std::vec::Vec<ContactCenterInsightsQaQuestion_AnswerChoices>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<std::vec::Vec<ContactCenterInsightsQaQuestion_Metrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_question_config: std
    ::option
    ::Option<std::vec::Vec<ContactCenterInsightsQaQuestion_PredefinedQuestionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qa_question_data_options: std::option::Option<std::vec::Vec<ContactCenterInsightsQaQuestion_QaQuestionDataOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContactCenterInsightsQaQuestion_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tuning_metadata: std::option::Option<std::vec::Vec<ContactCenterInsightsQaQuestion_TuningMetadata>>,
    dynamic: ContactCenterInsightsQaQuestionDynamic,
}

struct ContactCenterInsightsQaQuestion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContactCenterInsightsQaQuestionData_>,
}

#[derive(Clone)]
pub struct ContactCenterInsightsQaQuestion(std::rc::Rc<ContactCenterInsightsQaQuestion_>);

impl ContactCenterInsightsQaQuestion {
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
        "Set the field `abbreviation`.\nShort, descriptive string, used in the UI where it's not practical\nto display the full question body. E.g., \"Greeting\"."]
    pub fn set_abbreviation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().abbreviation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `answer_instructions`.\nInstructions describing how to determine the answer."]
    pub fn set_answer_instructions(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().answer_instructions = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `order`.\nDefines the order of the question within its parent scorecard revision."]
    pub fn set_order(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `question_body`.\nQuestion text. E.g., \"Did the agent greet the customer?\""]
    pub fn set_question_body(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().question_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `question_type`.\nThe type of question.\nPossible values:\nCUSTOMIZABLE\nPREDEFINED"]
    pub fn set_question_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().question_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nQuestions are tagged for categorization and scoring. Tags can either be:\n- Default Tags: These are predefined categories. They are identified by\ntheir string value (e.g., \"BUSINESS\", \"COMPLIANCE\", and \"CUSTOMER\").\n- Custom Tags: These are user-defined categories. They are identified by\ntheir full resource name (e.g.,\nprojects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}).\nBoth default and custom tags are used to group questions and to influence\nthe scoring of each question."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `answer_choices`.\n"]
    pub fn set_answer_choices(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsQaQuestion_AnswerChoices>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().answer_choices = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.answer_choices = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsQaQuestion_Metrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predefined_question_config`.\n"]
    pub fn set_predefined_question_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContactCenterInsightsQaQuestion_PredefinedQuestionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().predefined_question_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.predefined_question_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `qa_question_data_options`.\n"]
    pub fn set_qa_question_data_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsQaQuestion_QaQuestionDataOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().qa_question_data_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.qa_question_data_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContactCenterInsightsQaQuestion_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tuning_metadata`.\n"]
    pub fn set_tuning_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsQaQuestion_TuningMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tuning_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tuning_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `abbreviation` after provisioning.\nShort, descriptive string, used in the UI where it's not practical\nto display the full question body. E.g., \"Greeting\"."]
    pub fn abbreviation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abbreviation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `answer_instructions` after provisioning.\nInstructions describing how to determine the answer."]
    pub fn answer_instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.answer_instructions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this question was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the question.\nFormat:\nprojects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nDefines the order of the question within its parent scorecard revision."]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `qa_scorecard` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn qa_scorecard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qa_scorecard", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `question_body` after provisioning.\nQuestion text. E.g., \"Did the agent greet the customer?\""]
    pub fn question_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.question_body", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `question_type` after provisioning.\nThe type of question.\nPossible values:\nCUSTOMIZABLE\nPREDEFINED"]
    pub fn question_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.question_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nQuestions are tagged for categorization and scoring. Tags can either be:\n- Default Tags: These are predefined categories. They are identified by\ntheir string value (e.g., \"BUSINESS\", \"COMPLIANCE\", and \"CUSTOMER\").\n- Custom Tags: These are user-defined categories. They are identified by\ntheir full resource name (e.g.,\nprojects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}).\nBoth default and custom tags are used to group questions and to influence\nthe scoring of each question."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which the question was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `answer_choices` after provisioning.\n"]
    pub fn answer_choices(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_AnswerChoicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.answer_choices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predefined_question_config` after provisioning.\n"]
    pub fn predefined_question_config(
        &self,
    ) -> terrars::ListRef<ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_question_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qa_question_data_options` after provisioning.\n"]
    pub fn qa_question_data_options(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qa_question_data_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsQaQuestion_TimeoutsRef {
        ContactCenterInsightsQaQuestion_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tuning_metadata` after provisioning.\n"]
    pub fn tuning_metadata(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_TuningMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tuning_metadata", self.extract_ref()))
    }
}

impl terrars::Referable for ContactCenterInsightsQaQuestion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContactCenterInsightsQaQuestion { }

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion {
    type O = terrars::ListRef<ContactCenterInsightsQaQuestionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContactCenterInsightsQaQuestion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_contact_center_insights_qa_question".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContactCenterInsightsQaQuestion {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub qa_scorecard: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub revision: terrars::PrimField<std::string::String>,
}

impl BuildContactCenterInsightsQaQuestion {
    pub fn build(self, stack: &mut terrars::Stack) -> ContactCenterInsightsQaQuestion {
        let out = ContactCenterInsightsQaQuestion(std::rc::Rc::new(ContactCenterInsightsQaQuestion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContactCenterInsightsQaQuestionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                abbreviation: core::default::Default::default(),
                answer_instructions: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                order: core::default::Default::default(),
                project: core::default::Default::default(),
                qa_scorecard: self.qa_scorecard,
                question_body: core::default::Default::default(),
                question_type: core::default::Default::default(),
                revision: self.revision,
                tags: core::default::Default::default(),
                answer_choices: core::default::Default::default(),
                metrics: core::default::Default::default(),
                predefined_question_config: core::default::Default::default(),
                qa_question_data_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tuning_metadata: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContactCenterInsightsQaQuestionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContactCenterInsightsQaQuestionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `abbreviation` after provisioning.\nShort, descriptive string, used in the UI where it's not practical\nto display the full question body. E.g., \"Greeting\"."]
    pub fn abbreviation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.abbreviation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `answer_instructions` after provisioning.\nInstructions describing how to determine the answer."]
    pub fn answer_instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.answer_instructions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this question was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the question.\nFormat:\nprojects/{project}/locations/{location}/qaScorecards/{qa_scorecard}/revisions/{revision}/qaQuestions/{qa_question}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nDefines the order of the question within its parent scorecard revision."]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `qa_scorecard` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn qa_scorecard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qa_scorecard", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `question_body` after provisioning.\nQuestion text. E.g., \"Did the agent greet the customer?\""]
    pub fn question_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.question_body", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `question_type` after provisioning.\nThe type of question.\nPossible values:\nCUSTOMIZABLE\nPREDEFINED"]
    pub fn question_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.question_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nQuestions are tagged for categorization and scoring. Tags can either be:\n- Default Tags: These are predefined categories. They are identified by\ntheir string value (e.g., \"BUSINESS\", \"COMPLIANCE\", and \"CUSTOMER\").\n- Custom Tags: These are user-defined categories. They are identified by\ntheir full resource name (e.g.,\nprojects/{project}/locations/{location}/qaQuestionTags/{qa_question_tag}).\nBoth default and custom tags are used to group questions and to influence\nthe scoring of each question."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which the question was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `answer_choices` after provisioning.\n"]
    pub fn answer_choices(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_AnswerChoicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.answer_choices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `predefined_question_config` after provisioning.\n"]
    pub fn predefined_question_config(
        &self,
    ) -> terrars::ListRef<ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_question_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qa_question_data_options` after provisioning.\n"]
    pub fn qa_question_data_options(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qa_question_data_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsQaQuestion_TimeoutsRef {
        ContactCenterInsightsQaQuestion_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tuning_metadata` after provisioning.\n"]
    pub fn tuning_metadata(&self) -> terrars::ListRef<ContactCenterInsightsQaQuestion_TuningMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tuning_metadata", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ContactCenterInsightsQaQuestionDynamic {
    answer_choices: std::option::Option<terrars::DynamicBlock<ContactCenterInsightsQaQuestion_AnswerChoices>>,
    metrics: std::option::Option<terrars::DynamicBlock<ContactCenterInsightsQaQuestion_Metrics>>,
    predefined_question_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsQaQuestion_PredefinedQuestionConfig>>,
    qa_question_data_options: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsQaQuestion_QaQuestionDataOptions>>,
    tuning_metadata: std::option::Option<terrars::DynamicBlock<ContactCenterInsightsQaQuestion_TuningMetadata>>,
}

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_AnswerChoices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    na_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    score: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    str_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsQaQuestion_AnswerChoices {
    #[doc = "Set the field `bool_value`.\nBoolean value."]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nA short string used as an identifier."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `na_value`.\nA value of \"Not Applicable (N/A)\". If provided, this field may only\nbe set to 'true'. If a question receives this answer, it will be\nexcluded from any score calculations."]
    pub fn set_na_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.na_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_value`.\nNumerical value."]
    pub fn set_num_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `score`.\nNumerical score of the answer, used for generating the overall score of\na QaScorecardResult. If the answer uses na_value, this field is unused."]
    pub fn set_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.score = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `str_value`.\nString value."]
    pub fn set_str_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.str_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_AnswerChoices {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_AnswerChoices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_AnswerChoices {}

impl BuildContactCenterInsightsQaQuestion_AnswerChoices {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_AnswerChoices {
        ContactCenterInsightsQaQuestion_AnswerChoices {
            bool_value: core::default::Default::default(),
            key: core::default::Default::default(),
            na_value: core::default::Default::default(),
            num_value: core::default::Default::default(),
            score: core::default::Default::default(),
            str_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_AnswerChoicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_AnswerChoicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsQaQuestion_AnswerChoicesRef {
        ContactCenterInsightsQaQuestion_AnswerChoicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_AnswerChoicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\nBoolean value."]
    pub fn bool_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nA short string used as an identifier."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `na_value` after provisioning.\nA value of \"Not Applicable (N/A)\". If provided, this field may only\nbe set to 'true'. If a question receives this answer, it will be\nexcluded from any score calculations."]
    pub fn na_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.na_value", self.base))
    }

    #[doc = "Get a reference to the value of field `num_value` after provisioning.\nNumerical value."]
    pub fn num_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nNumerical score of the answer, used for generating the overall score of\na QaScorecardResult. If the answer uses na_value, this field is unused."]
    pub fn score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }

    #[doc = "Get a reference to the value of field `str_value` after provisioning.\nString value."]
    pub fn str_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.str_value", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_Metrics {}

impl ContactCenterInsightsQaQuestion_Metrics { }

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_Metrics {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_Metrics {}

impl BuildContactCenterInsightsQaQuestion_Metrics {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_Metrics {
        ContactCenterInsightsQaQuestion_Metrics {}
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_MetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContactCenterInsightsQaQuestion_MetricsRef {
        ContactCenterInsightsQaQuestion_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accuracy` after provisioning.\nAccuracy of the model. Measures the percentage of correct answers the\nmodel gave on the test set."]
    pub fn accuracy(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accuracy", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_PredefinedQuestionConfig {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsQaQuestion_PredefinedQuestionConfig {
    #[doc =
        "Set the field `type_`.\nThe type of the predefined question.\nPossible values:\nCONVERSATION_OUTCOME\nCONVERSATION_OUTCOME_ESCALATION_INITIATOR_ROLE"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_PredefinedQuestionConfig {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_PredefinedQuestionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_PredefinedQuestionConfig {}

impl BuildContactCenterInsightsQaQuestion_PredefinedQuestionConfig {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_PredefinedQuestionConfig {
        ContactCenterInsightsQaQuestion_PredefinedQuestionConfig { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef {
        ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_PredefinedQuestionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the predefined question.\nPossible values:\nCONVERSATION_OUTCOME\nCONVERSATION_OUTCOME_ESCALATION_INITIATOR_ROLE"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_dialogflow_interaction_data: std::option::Option<terrars::PrimField<bool>>,
}

impl ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {
    #[doc =
        "Set the field `include_dialogflow_interaction_data`.\nWhether to include the per turn Dialogflow interaction data in conversation\ntranscript."]
    pub fn set_include_dialogflow_interaction_data(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.include_dialogflow_interaction_data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {}

impl BuildContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {
        ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions {
            include_dialogflow_interaction_data: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptionsRef {
        ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `include_dialogflow_interaction_data` after provisioning.\nWhether to include the per turn Dialogflow interaction data in conversation\ntranscript."]
    pub fn include_dialogflow_interaction_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_dialogflow_interaction_data", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContactCenterInsightsQaQuestion_QaQuestionDataOptionsDynamic {
    conversation_data_options: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions>>,
}

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_QaQuestionDataOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_data_options: std
    ::option
    ::Option<std::vec::Vec<ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions>>,
    dynamic: ContactCenterInsightsQaQuestion_QaQuestionDataOptionsDynamic,
}

impl ContactCenterInsightsQaQuestion_QaQuestionDataOptions {
    #[doc = "Set the field `conversation_data_options`.\n"]
    pub fn set_conversation_data_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_data_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_data_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_QaQuestionDataOptions {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_QaQuestionDataOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_QaQuestionDataOptions {}

impl BuildContactCenterInsightsQaQuestion_QaQuestionDataOptions {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_QaQuestionDataOptions {
        ContactCenterInsightsQaQuestion_QaQuestionDataOptions {
            conversation_data_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef {
        ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_QaQuestionDataOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conversation_data_options` after provisioning.\n"]
    pub fn conversation_data_options(
        &self,
    ) -> terrars::ListRef<ContactCenterInsightsQaQuestion_QaQuestionDataOptions_ConversationDataOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_data_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsQaQuestion_Timeouts {
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

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_Timeouts {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_Timeouts {}

impl BuildContactCenterInsightsQaQuestion_Timeouts {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_Timeouts {
        ContactCenterInsightsQaQuestion_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContactCenterInsightsQaQuestion_TimeoutsRef {
        ContactCenterInsightsQaQuestion_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ContactCenterInsightsQaQuestion_TuningMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_validation_warnings: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_valid_label_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tuning_error: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsQaQuestion_TuningMetadata {
    #[doc =
        "Set the field `dataset_validation_warnings`.\nA list of any applicable data validation warnings about the question's\nfeedback labels."]
    pub fn set_dataset_validation_warnings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dataset_validation_warnings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_valid_label_count`.\nTotal number of valid labels provided for the question at the time of\ntuining."]
    pub fn set_total_valid_label_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_valid_label_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tuning_error`.\nError status of the tuning operation for the question. Will only be set\nif the tuning operation failed."]
    pub fn set_tuning_error(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tuning_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsQaQuestion_TuningMetadata {
    type O = terrars::BlockAssignable<ContactCenterInsightsQaQuestion_TuningMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsQaQuestion_TuningMetadata {}

impl BuildContactCenterInsightsQaQuestion_TuningMetadata {
    pub fn build(self) -> ContactCenterInsightsQaQuestion_TuningMetadata {
        ContactCenterInsightsQaQuestion_TuningMetadata {
            dataset_validation_warnings: core::default::Default::default(),
            total_valid_label_count: core::default::Default::default(),
            tuning_error: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsQaQuestion_TuningMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsQaQuestion_TuningMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsQaQuestion_TuningMetadataRef {
        ContactCenterInsightsQaQuestion_TuningMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsQaQuestion_TuningMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_validation_warnings` after provisioning.\nA list of any applicable data validation warnings about the question's\nfeedback labels."]
    pub fn dataset_validation_warnings(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_validation_warnings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_valid_label_count` after provisioning.\nTotal number of valid labels provided for the question at the time of\ntuining."]
    pub fn total_valid_label_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_valid_label_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tuning_error` after provisioning.\nError status of the tuning operation for the question. Will only be set\nif the tuning operation failed."]
    pub fn tuning_error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tuning_error", self.base))
    }
}
