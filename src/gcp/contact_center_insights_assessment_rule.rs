#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContactCenterInsightsAssessmentRuleData_ {
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
    assessment_rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
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
    sample_rule: std::option::Option<std::vec::Vec<ContactCenterInsightsAssessmentRule_SampleRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_info: std::option::Option<std::vec::Vec<ContactCenterInsightsAssessmentRule_ScheduleInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContactCenterInsightsAssessmentRule_Timeouts>,
    dynamic: ContactCenterInsightsAssessmentRuleDynamic,
}

struct ContactCenterInsightsAssessmentRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContactCenterInsightsAssessmentRuleData_>,
}

#[derive(Clone)]
pub struct ContactCenterInsightsAssessmentRule(std::rc::Rc<ContactCenterInsightsAssessmentRule_>);

impl ContactCenterInsightsAssessmentRule {
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

    #[doc = "Set the field `active`.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive."]
    pub fn set_active(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().active = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `assessment_rule_id`.\nA unique ID for the new AssessmentRule. This ID will become the final\ncomponent of the AssessmentRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn set_assessment_rule_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().assessment_rule_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay Name of the assessment rule."]
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

    #[doc = "Set the field `sample_rule`.\n"]
    pub fn set_sample_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsAssessmentRule_SampleRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sample_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sample_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule_info`.\n"]
    pub fn set_schedule_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContactCenterInsightsAssessmentRule_ScheduleInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContactCenterInsightsAssessmentRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive."]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `assessment_rule_id` after provisioning.\nA unique ID for the new AssessmentRule. This ID will become the final\ncomponent of the AssessmentRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn assessment_rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assessment_rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this assessment rule was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the assessment rule."]
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the assessment rule.\nFormat:\nprojects/{project}/locations/{location}/assessmentRules/{assessment_rule}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which this assessment rule was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sample_rule` after provisioning.\n"]
    pub fn sample_rule(&self) -> terrars::ListRef<ContactCenterInsightsAssessmentRule_SampleRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> terrars::ListRef<ContactCenterInsightsAssessmentRule_ScheduleInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAssessmentRule_TimeoutsRef {
        ContactCenterInsightsAssessmentRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ContactCenterInsightsAssessmentRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContactCenterInsightsAssessmentRule { }

impl terrars::ToListMappable for ContactCenterInsightsAssessmentRule {
    type O = terrars::ListRef<ContactCenterInsightsAssessmentRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContactCenterInsightsAssessmentRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_contact_center_insights_assessment_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContactCenterInsightsAssessmentRule {
    pub tf_id: std::string::String,
    #[doc = "Location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildContactCenterInsightsAssessmentRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ContactCenterInsightsAssessmentRule {
        let out = ContactCenterInsightsAssessmentRule(std::rc::Rc::new(ContactCenterInsightsAssessmentRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContactCenterInsightsAssessmentRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                active: core::default::Default::default(),
                assessment_rule_id: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                sample_rule: core::default::Default::default(),
                schedule_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContactCenterInsightsAssessmentRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAssessmentRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContactCenterInsightsAssessmentRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `active` after provisioning.\nIf true, apply this rule to conversations. Otherwise, this rule is\ninactive."]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `assessment_rule_id` after provisioning.\nA unique ID for the new AssessmentRule. This ID will become the final\ncomponent of the AssessmentRule's resource name. If no ID is specified,\na server-generated ID will be used.\n\nThis value should be 4-64 characters and must match the regular\nexpression '^[A-Za-z0-9]{4,64}$'."]
    pub fn assessment_rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assessment_rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this assessment rule was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name of the assessment rule."]
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the assessment rule.\nFormat:\nprojects/{project}/locations/{location}/assessmentRules/{assessment_rule}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which this assessment rule was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sample_rule` after provisioning.\n"]
    pub fn sample_rule(&self) -> terrars::ListRef<ContactCenterInsightsAssessmentRule_SampleRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> terrars::ListRef<ContactCenterInsightsAssessmentRule_ScheduleInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContactCenterInsightsAssessmentRule_TimeoutsRef {
        ContactCenterInsightsAssessmentRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ContactCenterInsightsAssessmentRuleDynamic {
    sample_rule: std::option::Option<terrars::DynamicBlock<ContactCenterInsightsAssessmentRule_SampleRule>>,
    schedule_info: std::option::Option<terrars::DynamicBlock<ContactCenterInsightsAssessmentRule_ScheduleInfo>>,
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAssessmentRule_SampleRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_row: std::option::Option<terrars::PrimField<f64>>,
}

impl ContactCenterInsightsAssessmentRule_SampleRule {
    #[doc =
        "Set the field `conversation_filter`.\nTo specify the filter for the conversions that should apply this sample\nrule. An empty filter means this sample rule applies to all conversations."]
    pub fn set_conversation_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.conversation_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dimension`.\nGroup by dimension to sample the conversation. If no dimension is\nprovided, the sampling will be applied to the project level.\nCurrent supported dimensions is 'quality_metadata.agent_info.agent_id'."]
    pub fn set_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sample_percentage`.\nPercentage of conversations that we should sample  based on the dimension\nbetween [0, 100]."]
    pub fn set_sample_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_row`.\nNumber of the conversations that we should sample based on the dimension."]
    pub fn set_sample_row(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sample_row = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsAssessmentRule_SampleRule {
    type O = terrars::BlockAssignable<ContactCenterInsightsAssessmentRule_SampleRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAssessmentRule_SampleRule {}

impl BuildContactCenterInsightsAssessmentRule_SampleRule {
    pub fn build(self) -> ContactCenterInsightsAssessmentRule_SampleRule {
        ContactCenterInsightsAssessmentRule_SampleRule {
            conversation_filter: core::default::Default::default(),
            dimension: core::default::Default::default(),
            sample_percentage: core::default::Default::default(),
            sample_row: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAssessmentRule_SampleRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAssessmentRule_SampleRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAssessmentRule_SampleRuleRef {
        ContactCenterInsightsAssessmentRule_SampleRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAssessmentRule_SampleRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `conversation_filter` after provisioning.\nTo specify the filter for the conversions that should apply this sample\nrule. An empty filter means this sample rule applies to all conversations."]
    pub fn conversation_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conversation_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dimension` after provisioning.\nGroup by dimension to sample the conversation. If no dimension is\nprovided, the sampling will be applied to the project level.\nCurrent supported dimensions is 'quality_metadata.agent_info.agent_id'."]
    pub fn dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_percentage` after provisioning.\nPercentage of conversations that we should sample  based on the dimension\nbetween [0, 100]."]
    pub fn sample_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_percentage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_row` after provisioning.\nNumber of the conversations that we should sample based on the dimension."]
    pub fn sample_row(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_row", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAssessmentRule_ScheduleInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsAssessmentRule_ScheduleInfo {
    #[doc =
        "Set the field `end_time`.\nEnd time of the schedule. If not specified, will keep scheduling new\npipelines for execution until the schedule is no longer active or deleted.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nThe groc expression.\nFormat: 'every number [synchronized]'\nCron syntax is not supported.\nTime units can be: minutes, hours\nSynchronized is optional and indicates that the schedule should be\nsynchronized to the start of the interval: every 5 minutes synchronized\nmeans 00:00, 00:05 ...\nOtherwise the start time is random within the interval.\nExample: 'every 5 minutes'\ncould be  00:02, 00:07, 00:12, ..."]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time`.\nStart time of the schedule. If not specified, will start as soon as the\nschedule is created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nThe timezone to use for the groc expression.\nIf not specified, defaults to UTC."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContactCenterInsightsAssessmentRule_ScheduleInfo {
    type O = terrars::BlockAssignable<ContactCenterInsightsAssessmentRule_ScheduleInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAssessmentRule_ScheduleInfo {}

impl BuildContactCenterInsightsAssessmentRule_ScheduleInfo {
    pub fn build(self) -> ContactCenterInsightsAssessmentRule_ScheduleInfo {
        ContactCenterInsightsAssessmentRule_ScheduleInfo {
            end_time: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_time: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAssessmentRule_ScheduleInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAssessmentRule_ScheduleInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAssessmentRule_ScheduleInfoRef {
        ContactCenterInsightsAssessmentRule_ScheduleInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAssessmentRule_ScheduleInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nEnd time of the schedule. If not specified, will keep scheduling new\npipelines for execution until the schedule is no longer active or deleted.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nThe groc expression.\nFormat: 'every number [synchronized]'\nCron syntax is not supported.\nTime units can be: minutes, hours\nSynchronized is optional and indicates that the schedule should be\nsynchronized to the start of the interval: every 5 minutes synchronized\nmeans 00:00, 00:05 ...\nOtherwise the start time is random within the interval.\nExample: 'every 5 minutes'\ncould be  00:02, 00:07, 00:12, ..."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nStart time of the schedule. If not specified, will start as soon as the\nschedule is created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe timezone to use for the groc expression.\nIf not specified, defaults to UTC."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ContactCenterInsightsAssessmentRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContactCenterInsightsAssessmentRule_Timeouts {
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

impl terrars::ToListMappable for ContactCenterInsightsAssessmentRule_Timeouts {
    type O = terrars::BlockAssignable<ContactCenterInsightsAssessmentRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContactCenterInsightsAssessmentRule_Timeouts {}

impl BuildContactCenterInsightsAssessmentRule_Timeouts {
    pub fn build(self) -> ContactCenterInsightsAssessmentRule_Timeouts {
        ContactCenterInsightsAssessmentRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContactCenterInsightsAssessmentRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContactCenterInsightsAssessmentRule_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContactCenterInsightsAssessmentRule_TimeoutsRef {
        ContactCenterInsightsAssessmentRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContactCenterInsightsAssessmentRule_TimeoutsRef {
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
