#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContactCenterInsightsAnalysisRuleData_ {
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
    active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotator_selector: std::option::Option<std::vec::Vec<ContactCenterInsightsAnalysisRule_AnnotatorSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContactCenterInsightsAnalysisRule_Timeouts>,
    dynamic: ContactCenterInsightsAnalysisRuleDynamic,
}

struct ContactCenterInsightsAnalysisRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContactCenterInsightsAnalysisRuleData_>,
}

#[derive(Clone)]
pub struct ContactCenterInsightsAnalysisRule(std::rc::Rc<ContactCenterInsightsAnalysisRule_>);

impl ContactCenterInsightsAnalysisRule {
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
        "Set the field `active`.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive and saved as a draft."]
    pub fn set_active(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `analysis_percentage`.\nPercentage of conversations that we should apply this analysis setting\nautomatically, between [0, 1]. For example, 0.1 means 10%. Conversations\nare sampled in a determenestic way. The original runtime_percentage &\nupload percentage will be replaced by defining filters on the conversation."]
    pub fn set_analysis_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().analysis_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `conversation_filter`.\nFilter for the conversations that should apply this analysis\nrule. An empty filter means this analysis rule applies to all\nconversations.\nRefer to https://cloud.google.com/contact-center/insights/docs/filtering\nfor details."]
    pub fn set_conversation_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().conversation_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay Name of the analysis rule."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `annotator_selector`.\n"]
    pub fn set_annotator_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().annotator_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.annotator_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContactCenterInsightsAnalysisRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive and saved as a draft."]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `analysis_percentage` after provisioning.\nPercentage of conversations that we should apply this analysis setting\nautomatically, between [0, 1]. For example, 0.1 means 10%. Conversations\nare sampled in a determenestic way. The original runtime_percentage &\nupload percentage will be replaced by defining filters on the conversation."]
    pub fn analysis_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analysis_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conversation_filter` after provisioning.\nFilter for the conversations that should apply this analysis\nrule. An empty filter means this analysis rule applies to all\nconversations.\nRefer to https://cloud.google.com/contact-center/insights/docs/filtering\nfor details."]
    pub fn conversation_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conversation_filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this analysis rule was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the analysis rule."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the analysis rule. Randomly generated by Insights."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The most recent time at which this analysis rule was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `annotator_selector` after provisioning.\n"]
    pub fn annotator_selector(&self) -> terrars::ListRef<ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.annotator_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAnalysisRule_TimeoutsRef {
        ContactCenterInsightsAnalysisRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ContactCenterInsightsAnalysisRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContactCenterInsightsAnalysisRule { }

impl terrars::ToListMappable for ContactCenterInsightsAnalysisRule {
    type O = terrars::ListRef<ContactCenterInsightsAnalysisRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContactCenterInsightsAnalysisRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_contact_center_insights_analysis_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContactCenterInsightsAnalysisRule {
    pub tf_id: std::string::String,
    #[doc = "Location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildContactCenterInsightsAnalysisRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ContactCenterInsightsAnalysisRule {
        let out = ContactCenterInsightsAnalysisRule(std::rc::Rc::new(ContactCenterInsightsAnalysisRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContactCenterInsightsAnalysisRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                active: core::default::Default::default(),
                analysis_percentage: core::default::Default::default(),
                conversation_filter: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                annotator_selector: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContactCenterInsightsAnalysisRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAnalysisRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContactCenterInsightsAnalysisRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive and saved as a draft."]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `analysis_percentage` after provisioning.\nPercentage of conversations that we should apply this analysis setting\nautomatically, between [0, 1]. For example, 0.1 means 10%. Conversations\nare sampled in a determenestic way. The original runtime_percentage &\nupload percentage will be replaced by defining filters on the conversation."]
    pub fn analysis_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analysis_percentage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conversation_filter` after provisioning.\nFilter for the conversations that should apply this analysis\nrule. An empty filter means this analysis rule applies to all\nconversations.\nRefer to https://cloud.google.com/contact-center/insights/docs/filtering\nfor details."]
    pub fn conversation_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conversation_filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which this analysis rule was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the analysis rule."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the analysis rule. Randomly generated by Insights."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The most recent time at which this analysis rule was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `annotator_selector` after provisioning.\n"]
    pub fn annotator_selector(&self) -> terrars::ListRef<ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.annotator_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAnalysisRule_TimeoutsRef {
        ContactCenterInsightsAnalysisRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ContactCenterInsightsAnalysisRuleDynamic {
    annotator_selector: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsAnalysisRule_AnnotatorSelector>>,
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qa_scorecard_revisions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {
    #[doc = "Set the field `qa_scorecard_revisions`.\nList of QaScorecardRevisions."]
    pub fn set_qa_scorecard_revisions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.qa_scorecard_revisions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {
    type O = terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {}

impl BuildContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {
    pub fn build(self) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList {
            qa_scorecard_revisions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardListRef {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `qa_scorecard_revisions` after provisioning.\nList of QaScorecardRevisions."]
    pub fn qa_scorecard_revisions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qa_scorecard_revisions", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigDynamic {
    scorecard_list: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList>>,
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scorecard_list: std
    ::option
    ::Option<std::vec::Vec<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList>>,
    dynamic: ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigDynamic,
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {
    #[doc = "Set the field `scorecard_list`.\n"]
    pub fn set_scorecard_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardList>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scorecard_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scorecard_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {
    type O = terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {}

impl BuildContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {
    pub fn build(self) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig {
            scorecard_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigRef {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scorecard_list` after provisioning.\n"]
    pub fn scorecard_list(
        &self,
    ) -> terrars::ListRef<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig_ScorecardListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scorecard_list", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_model: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {
    #[doc =
        "Set the field `conversation_profile`.\nResource name of the Dialogflow conversation profile.\nFormat:\nprojects/{project}/locations/{location}/conversationProfiles/{conversation_profile}"]
    pub fn set_conversation_profile(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.conversation_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `summarization_model`.\nDefault summarization model to be used.\nPossible values:\nSUMMARIZATION_MODEL_UNSPECIFIED\nBASELINE_MODEL\nBASELINE_MODEL_V2_0 Possible values: [\"BASELINE_MODEL\", \"BASELINE_MODEL_V2_0\"]"]
    pub fn set_summarization_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.summarization_model = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {
    type O = terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {}

impl BuildContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {
    pub fn build(self) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig {
            conversation_profile: core::default::Default::default(),
            summarization_model: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfigRef {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `conversation_profile` after provisioning.\nResource name of the Dialogflow conversation profile.\nFormat:\nprojects/{project}/locations/{location}/conversationProfiles/{conversation_profile}"]
    pub fn conversation_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conversation_profile", self.base))
    }

    #[doc =
        "Get a reference to the value of field `summarization_model` after provisioning.\nDefault summarization model to be used.\nPossible values:\nSUMMARIZATION_MODEL_UNSPECIFIED\nBASELINE_MODEL\nBASELINE_MODEL_V2_0 Possible values: [\"BASELINE_MODEL\", \"BASELINE_MODEL_V2_0\"]"]
    pub fn summarization_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summarization_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContactCenterInsightsAnalysisRule_AnnotatorSelectorDynamic {
    qa_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig>>,
    summarization_config: std
    ::option
    ::Option<terrars::DynamicBlock<ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig>>,
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issue_models: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phrase_matchers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_entity_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_intent_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_interruption_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_issue_model_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_phrase_matcher_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_qa_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_sentiment_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_silence_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_summarization_annotator: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qa_config: std::option::Option<std::vec::Vec<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_config: std
    ::option
    ::Option<std::vec::Vec<ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig>>,
    dynamic: ContactCenterInsightsAnalysisRule_AnnotatorSelectorDynamic,
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelector {
    #[doc =
        "Set the field `issue_models`.\nThe issue model to run. If not provided, the most recently deployed topic\nmodel will be used. The provided issue model will only be used for\ninference if the issue model is deployed and if run_issue_model_annotator\nis set to true. If more than one issue model is provided, only the first\nprovided issue model will be used for inference."]
    pub fn set_issue_models(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.issue_models = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `phrase_matchers`.\nThe list of phrase matchers to run. If not provided, all active phrase\nmatchers will be used. If inactive phrase matchers are provided, they will\nnot be used. Phrase matchers will be run only if\nrun_phrase_matcher_annotator is set to true. Format:\nprojects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}"]
    pub fn set_phrase_matchers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.phrase_matchers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_entity_annotator`.\nWhether to run the entity annotator."]
    pub fn set_run_entity_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_entity_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_intent_annotator`.\nWhether to run the intent annotator."]
    pub fn set_run_intent_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_intent_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_interruption_annotator`.\nWhether to run the interruption annotator."]
    pub fn set_run_interruption_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_interruption_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `run_issue_model_annotator`.\nWhether to run the issue model annotator. A model should have already been\ndeployed for this to take effect."]
    pub fn set_run_issue_model_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_issue_model_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_phrase_matcher_annotator`.\nWhether to run the active phrase matcher annotator(s)."]
    pub fn set_run_phrase_matcher_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_phrase_matcher_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_qa_annotator`.\nWhether to run the QA annotator."]
    pub fn set_run_qa_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_qa_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_sentiment_annotator`.\nWhether to run the sentiment annotator."]
    pub fn set_run_sentiment_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_sentiment_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_silence_annotator`.\nWhether to run the silence annotator."]
    pub fn set_run_silence_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_silence_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_summarization_annotator`.\nWhether to run the summarization annotator."]
    pub fn set_run_summarization_annotator(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.run_summarization_annotator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qa_config`.\n"]
    pub fn set_qa_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.qa_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.qa_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `summarization_config`.\n"]
    pub fn set_summarization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summarization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsAnalysisRule_AnnotatorSelector {
    type O = terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_AnnotatorSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAnalysisRule_AnnotatorSelector {}

impl BuildContactCenterInsightsAnalysisRule_AnnotatorSelector {
    pub fn build(self) -> ContactCenterInsightsAnalysisRule_AnnotatorSelector {
        ContactCenterInsightsAnalysisRule_AnnotatorSelector {
            issue_models: core::default::Default::default(),
            phrase_matchers: core::default::Default::default(),
            run_entity_annotator: core::default::Default::default(),
            run_intent_annotator: core::default::Default::default(),
            run_interruption_annotator: core::default::Default::default(),
            run_issue_model_annotator: core::default::Default::default(),
            run_phrase_matcher_annotator: core::default::Default::default(),
            run_qa_annotator: core::default::Default::default(),
            run_sentiment_annotator: core::default::Default::default(),
            run_silence_annotator: core::default::Default::default(),
            run_summarization_annotator: core::default::Default::default(),
            qa_config: core::default::Default::default(),
            summarization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef {
        ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAnalysisRule_AnnotatorSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `issue_models` after provisioning.\nThe issue model to run. If not provided, the most recently deployed topic\nmodel will be used. The provided issue model will only be used for\ninference if the issue model is deployed and if run_issue_model_annotator\nis set to true. If more than one issue model is provided, only the first\nprovided issue model will be used for inference."]
    pub fn issue_models(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issue_models", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phrase_matchers` after provisioning.\nThe list of phrase matchers to run. If not provided, all active phrase\nmatchers will be used. If inactive phrase matchers are provided, they will\nnot be used. Phrase matchers will be run only if\nrun_phrase_matcher_annotator is set to true. Format:\nprojects/{project}/locations/{location}/phraseMatchers/{phrase_matcher}"]
    pub fn phrase_matchers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phrase_matchers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_entity_annotator` after provisioning.\nWhether to run the entity annotator."]
    pub fn run_entity_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_entity_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_intent_annotator` after provisioning.\nWhether to run the intent annotator."]
    pub fn run_intent_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_intent_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_interruption_annotator` after provisioning.\nWhether to run the interruption annotator."]
    pub fn run_interruption_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_interruption_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_issue_model_annotator` after provisioning.\nWhether to run the issue model annotator. A model should have already been\ndeployed for this to take effect."]
    pub fn run_issue_model_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_issue_model_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_phrase_matcher_annotator` after provisioning.\nWhether to run the active phrase matcher annotator(s)."]
    pub fn run_phrase_matcher_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_phrase_matcher_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_qa_annotator` after provisioning.\nWhether to run the QA annotator."]
    pub fn run_qa_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_qa_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_sentiment_annotator` after provisioning.\nWhether to run the sentiment annotator."]
    pub fn run_sentiment_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_sentiment_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_silence_annotator` after provisioning.\nWhether to run the silence annotator."]
    pub fn run_silence_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_silence_annotator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_summarization_annotator` after provisioning.\nWhether to run the summarization annotator."]
    pub fn run_summarization_annotator(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_summarization_annotator", self.base))
    }

    #[doc = "Get a reference to the value of field `qa_config` after provisioning.\n"]
    pub fn qa_config(&self) -> terrars::ListRef<ContactCenterInsightsAnalysisRule_AnnotatorSelector_QaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qa_config", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization_config` after provisioning.\n"]
    pub fn summarization_config(
        &self,
    ) -> terrars::ListRef<ContactCenterInsightsAnalysisRule_AnnotatorSelector_SummarizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAnalysisRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsAnalysisRule_Timeouts {
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

impl terrars::ToListMappable for ContactCenterInsightsAnalysisRule_Timeouts {
    type O = terrars::BlockAssignable<ContactCenterInsightsAnalysisRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAnalysisRule_Timeouts {}

impl BuildContactCenterInsightsAnalysisRule_Timeouts {
    pub fn build(self) -> ContactCenterInsightsAnalysisRule_Timeouts {
        ContactCenterInsightsAnalysisRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAnalysisRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAnalysisRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContactCenterInsightsAnalysisRule_TimeoutsRef {
        ContactCenterInsightsAnalysisRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAnalysisRule_TimeoutsRef {
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
