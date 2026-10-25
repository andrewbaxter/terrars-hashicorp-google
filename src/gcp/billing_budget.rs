#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BillingBudgetData_ {
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
    billing_account: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ownership_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_updates_rule: std::option::Option<std::vec::Vec<BillingBudget_AllUpdatesRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<std::vec::Vec<BillingBudget_Amount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    budget_filter: std::option::Option<std::vec::Vec<BillingBudget_BudgetFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_rules: std::option::Option<std::vec::Vec<BillingBudget_ThresholdRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BillingBudget_Timeouts>,
    dynamic: BillingBudgetDynamic,
}

struct BillingBudget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BillingBudgetData_>,
}

#[derive(Clone)]
pub struct BillingBudget(std::rc::Rc<BillingBudget_>);

impl BillingBudget {
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

    #[doc = "Set the field `display_name`.\nUser data for display name in UI. Must be <= 60 chars."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ownership_scope`.\nThe ownership scope of the budget. The ownership scope and users'\nIAM permissions determine who has full access to the budget's data. Possible values: [\"OWNERSHIP_SCOPE_UNSPECIFIED\", \"ALL_USERS\", \"BILLING_ACCOUNT\"]"]
    pub fn set_ownership_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ownership_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all_updates_rule`.\n"]
    pub fn set_all_updates_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_AllUpdatesRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().all_updates_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.all_updates_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(self, v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_Amount>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().amount = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.amount = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `budget_filter`.\n"]
    pub fn set_budget_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_BudgetFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().budget_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.budget_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threshold_rules`.\n"]
    pub fn set_threshold_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_ThresholdRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threshold_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threshold_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BillingBudget_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `billing_account` after provisioning.\nID of the billing account to set a budget on."]
    pub fn billing_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser data for display name in UI. Must be <= 60 chars."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the budget. The resource name\nimplies the scope of a budget. Values are of the form\nbillingAccounts/{billingAccountId}/budgets/{budgetId}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_scope` after provisioning.\nThe ownership scope of the budget. The ownership scope and users'\nIAM permissions determine who has full access to the budget's data. Possible values: [\"OWNERSHIP_SCOPE_UNSPECIFIED\", \"ALL_USERS\", \"BILLING_ACCOUNT\"]"]
    pub fn ownership_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `all_updates_rule` after provisioning.\n"]
    pub fn all_updates_rule(&self) -> terrars::ListRef<BillingBudget_AllUpdatesRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_updates_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::ListRef<BillingBudget_AmountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_filter` after provisioning.\n"]
    pub fn budget_filter(&self) -> terrars::ListRef<BillingBudget_BudgetFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.budget_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_rules` after provisioning.\n"]
    pub fn threshold_rules(&self) -> terrars::ListRef<BillingBudget_ThresholdRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingBudget_TimeoutsRef {
        BillingBudget_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BillingBudget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BillingBudget { }

impl terrars::ToListMappable for BillingBudget {
    type O = terrars::ListRef<BillingBudgetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BillingBudget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_billing_budget".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBillingBudget {
    pub tf_id: std::string::String,
    #[doc = "ID of the billing account to set a budget on."]
    pub billing_account: terrars::PrimField<std::string::String>,
}

impl BuildBillingBudget {
    pub fn build(self, stack: &mut terrars::Stack) -> BillingBudget {
        let out = BillingBudget(std::rc::Rc::new(BillingBudget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BillingBudgetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                billing_account: self.billing_account,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                ownership_scope: core::default::Default::default(),
                all_updates_rule: core::default::Default::default(),
                amount: core::default::Default::default(),
                budget_filter: core::default::Default::default(),
                threshold_rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BillingBudgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudgetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BillingBudgetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `billing_account` after provisioning.\nID of the billing account to set a budget on."]
    pub fn billing_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser data for display name in UI. Must be <= 60 chars."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the budget. The resource name\nimplies the scope of a budget. Values are of the form\nbillingAccounts/{billingAccountId}/budgets/{budgetId}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_scope` after provisioning.\nThe ownership scope of the budget. The ownership scope and users'\nIAM permissions determine who has full access to the budget's data. Possible values: [\"OWNERSHIP_SCOPE_UNSPECIFIED\", \"ALL_USERS\", \"BILLING_ACCOUNT\"]"]
    pub fn ownership_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `all_updates_rule` after provisioning.\n"]
    pub fn all_updates_rule(&self) -> terrars::ListRef<BillingBudget_AllUpdatesRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_updates_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::ListRef<BillingBudget_AmountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_filter` after provisioning.\n"]
    pub fn budget_filter(&self) -> terrars::ListRef<BillingBudget_BudgetFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.budget_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_rules` after provisioning.\n"]
    pub fn threshold_rules(&self) -> terrars::ListRef<BillingBudget_ThresholdRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingBudget_TimeoutsRef {
        BillingBudget_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BillingBudgetDynamic {
    all_updates_rule: std::option::Option<terrars::DynamicBlock<BillingBudget_AllUpdatesRule>>,
    amount: std::option::Option<terrars::DynamicBlock<BillingBudget_Amount>>,
    budget_filter: std::option::Option<terrars::DynamicBlock<BillingBudget_BudgetFilter>>,
    threshold_rules: std::option::Option<terrars::DynamicBlock<BillingBudget_ThresholdRules>>,
}

#[derive(Serialize)]
pub struct BillingBudget_AllUpdatesRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_default_iam_recipients: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_project_level_recipients: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_notification_channels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BillingBudget_AllUpdatesRule {
    #[doc =
        "Set the field `disable_default_iam_recipients`.\nBoolean. When set to true, disables default notifications sent\nwhen a threshold is exceeded. Default recipients are\nthose with Billing Account Administrators and Billing\nAccount Users IAM roles for the target account."]
    pub fn set_disable_default_iam_recipients(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_default_iam_recipients = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_project_level_recipients`.\nWhen set to true, and when the budget has a single project configured,\nnotifications will be sent to project level recipients of that project.\nThis field will be ignored if the budget has multiple or no project configured.\n\nCurrently, project level recipients are the users with Owner role on a cloud project."]
    pub fn set_enable_project_level_recipients(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_project_level_recipients = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `monitoring_notification_channels`.\nThe full resource name of a monitoring notification\nchannel in the form\nprojects/{project_id}/notificationChannels/{channel_id}.\nA maximum of 5 channels are allowed."]
    pub fn set_monitoring_notification_channels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.monitoring_notification_channels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pubsub_topic`.\nThe name of the Cloud Pub/Sub topic where budget related\nmessages will be published, in the form\nprojects/{project_id}/topics/{topic_id}. Updates are sent\nat regular intervals to the topic."]
    pub fn set_pubsub_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pubsub_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema_version`.\nThe schema version of the notification. Only \"1.0\" is\naccepted. It represents the JSON schema as defined in\nhttps://cloud.google.com/billing/docs/how-to/budgets#notification_format."]
    pub fn set_schema_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BillingBudget_AllUpdatesRule {
    type O = terrars::BlockAssignable<BillingBudget_AllUpdatesRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_AllUpdatesRule {}

impl BuildBillingBudget_AllUpdatesRule {
    pub fn build(self) -> BillingBudget_AllUpdatesRule {
        BillingBudget_AllUpdatesRule {
            disable_default_iam_recipients: core::default::Default::default(),
            enable_project_level_recipients: core::default::Default::default(),
            monitoring_notification_channels: core::default::Default::default(),
            pubsub_topic: core::default::Default::default(),
            schema_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_AllUpdatesRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_AllUpdatesRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_AllUpdatesRuleRef {
        BillingBudget_AllUpdatesRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_AllUpdatesRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_default_iam_recipients` after provisioning.\nBoolean. When set to true, disables default notifications sent\nwhen a threshold is exceeded. Default recipients are\nthose with Billing Account Administrators and Billing\nAccount Users IAM roles for the target account."]
    pub fn disable_default_iam_recipients(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_default_iam_recipients", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_project_level_recipients` after provisioning.\nWhen set to true, and when the budget has a single project configured,\nnotifications will be sent to project level recipients of that project.\nThis field will be ignored if the budget has multiple or no project configured.\n\nCurrently, project level recipients are the users with Owner role on a cloud project."]
    pub fn enable_project_level_recipients(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_project_level_recipients", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_notification_channels` after provisioning.\nThe full resource name of a monitoring notification\nchannel in the form\nprojects/{project_id}/notificationChannels/{channel_id}.\nA maximum of 5 channels are allowed."]
    pub fn monitoring_notification_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_notification_channels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe name of the Cloud Pub/Sub topic where budget related\nmessages will be published, in the form\nprojects/{project_id}/topics/{topic_id}. Updates are sent\nat regular intervals to the topic."]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_version` after provisioning.\nThe schema version of the notification. Only \"1.0\" is\naccepted. It represents the JSON schema as defined in\nhttps://cloud.google.com/billing/docs/how-to/budgets#notification_format."]
    pub fn schema_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingBudget_Amount_SpecifiedAmount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BillingBudget_Amount_SpecifiedAmount {
    #[doc = "Set the field `currency_code`.\nThe 3-letter currency code defined in ISO 4217."]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nanos`.\nNumber of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999\ninclusive. If units is positive, nanos must be positive or\nzero. If units is zero, nanos can be positive, zero, or\nnegative. If units is negative, nanos must be negative or\nzero. For example $-1.75 is represented as units=-1 and\nnanos=-750,000,000."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `units`.\nThe whole units of the amount. For example if currencyCode\nis \"USD\", then 1 unit is one US dollar."]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BillingBudget_Amount_SpecifiedAmount {
    type O = terrars::BlockAssignable<BillingBudget_Amount_SpecifiedAmount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_Amount_SpecifiedAmount {}

impl BuildBillingBudget_Amount_SpecifiedAmount {
    pub fn build(self) -> BillingBudget_Amount_SpecifiedAmount {
        BillingBudget_Amount_SpecifiedAmount {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_Amount_SpecifiedAmountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_Amount_SpecifiedAmountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_Amount_SpecifiedAmountRef {
        BillingBudget_Amount_SpecifiedAmountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_Amount_SpecifiedAmountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `currency_code` after provisioning.\nThe 3-letter currency code defined in ISO 4217."]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nNumber of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999\ninclusive. If units is positive, nanos must be positive or\nzero. If units is zero, nanos can be positive, zero, or\nnegative. If units is negative, nanos must be negative or\nzero. For example $-1.75 is represented as units=-1 and\nnanos=-750,000,000."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `units` after provisioning.\nThe whole units of the amount. For example if currencyCode\nis \"USD\", then 1 unit is one US dollar."]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize, Default)]
struct BillingBudget_AmountDynamic {
    specified_amount: std::option::Option<terrars::DynamicBlock<BillingBudget_Amount_SpecifiedAmount>>,
}

#[derive(Serialize)]
pub struct BillingBudget_Amount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_period_amount: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specified_amount: std::option::Option<std::vec::Vec<BillingBudget_Amount_SpecifiedAmount>>,
    dynamic: BillingBudget_AmountDynamic,
}

impl BillingBudget_Amount {
    #[doc =
        "Set the field `last_period_amount`.\nConfigures a budget amount that is automatically set to 100% of\nlast period's spend.\nBoolean. Set value to true to use. Do not set to false, instead\nuse the 'specified_amount' block."]
    pub fn set_last_period_amount(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.last_period_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `specified_amount`.\n"]
    pub fn set_specified_amount(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_Amount_SpecifiedAmount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.specified_amount = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.specified_amount = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BillingBudget_Amount {
    type O = terrars::BlockAssignable<BillingBudget_Amount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_Amount {}

impl BuildBillingBudget_Amount {
    pub fn build(self) -> BillingBudget_Amount {
        BillingBudget_Amount {
            last_period_amount: core::default::Default::default(),
            specified_amount: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_AmountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_AmountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_AmountRef {
        BillingBudget_AmountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_AmountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `last_period_amount` after provisioning.\nConfigures a budget amount that is automatically set to 100% of\nlast period's spend.\nBoolean. Set value to true to use. Do not set to false, instead\nuse the 'specified_amount' block."]
    pub fn last_period_amount(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_period_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `specified_amount` after provisioning.\n"]
    pub fn specified_amount(&self) -> terrars::ListRef<BillingBudget_Amount_SpecifiedAmountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specified_amount", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingBudget_BudgetFilter_CustomPeriod_EndDate {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl BillingBudget_BudgetFilter_CustomPeriod_EndDate { }

impl terrars::ToListMappable for BillingBudget_BudgetFilter_CustomPeriod_EndDate {
    type O = terrars::BlockAssignable<BillingBudget_BudgetFilter_CustomPeriod_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_BudgetFilter_CustomPeriod_EndDate {
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month."]
    pub day: terrars::PrimField<f64>,
    #[doc = "Month of a year. Must be from 1 to 12."]
    pub month: terrars::PrimField<f64>,
    #[doc = "Year of the date. Must be from 1 to 9999."]
    pub year: terrars::PrimField<f64>,
}

impl BuildBillingBudget_BudgetFilter_CustomPeriod_EndDate {
    pub fn build(self) -> BillingBudget_BudgetFilter_CustomPeriod_EndDate {
        BillingBudget_BudgetFilter_CustomPeriod_EndDate {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_BudgetFilter_CustomPeriod_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_BudgetFilter_CustomPeriod_EndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BillingBudget_BudgetFilter_CustomPeriod_EndDateRef {
        BillingBudget_BudgetFilter_CustomPeriod_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_BudgetFilter_CustomPeriod_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingBudget_BudgetFilter_CustomPeriod_StartDate {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl BillingBudget_BudgetFilter_CustomPeriod_StartDate { }

impl terrars::ToListMappable for BillingBudget_BudgetFilter_CustomPeriod_StartDate {
    type O = terrars::BlockAssignable<BillingBudget_BudgetFilter_CustomPeriod_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_BudgetFilter_CustomPeriod_StartDate {
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month."]
    pub day: terrars::PrimField<f64>,
    #[doc = "Month of a year. Must be from 1 to 12."]
    pub month: terrars::PrimField<f64>,
    #[doc = "Year of the date. Must be from 1 to 9999."]
    pub year: terrars::PrimField<f64>,
}

impl BuildBillingBudget_BudgetFilter_CustomPeriod_StartDate {
    pub fn build(self) -> BillingBudget_BudgetFilter_CustomPeriod_StartDate {
        BillingBudget_BudgetFilter_CustomPeriod_StartDate {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_BudgetFilter_CustomPeriod_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_BudgetFilter_CustomPeriod_StartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BillingBudget_BudgetFilter_CustomPeriod_StartDateRef {
        BillingBudget_BudgetFilter_CustomPeriod_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_BudgetFilter_CustomPeriod_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize, Default)]
struct BillingBudget_BudgetFilter_CustomPeriodDynamic {
    end_date: std::option::Option<terrars::DynamicBlock<BillingBudget_BudgetFilter_CustomPeriod_EndDate>>,
    start_date: std::option::Option<terrars::DynamicBlock<BillingBudget_BudgetFilter_CustomPeriod_StartDate>>,
}

#[derive(Serialize)]
pub struct BillingBudget_BudgetFilter_CustomPeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<BillingBudget_BudgetFilter_CustomPeriod_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<BillingBudget_BudgetFilter_CustomPeriod_StartDate>>,
    dynamic: BillingBudget_BudgetFilter_CustomPeriodDynamic,
}

impl BillingBudget_BudgetFilter_CustomPeriod {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_BudgetFilter_CustomPeriod_EndDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_BudgetFilter_CustomPeriod_StartDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BillingBudget_BudgetFilter_CustomPeriod {
    type O = terrars::BlockAssignable<BillingBudget_BudgetFilter_CustomPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_BudgetFilter_CustomPeriod {}

impl BuildBillingBudget_BudgetFilter_CustomPeriod {
    pub fn build(self) -> BillingBudget_BudgetFilter_CustomPeriod {
        BillingBudget_BudgetFilter_CustomPeriod {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_BudgetFilter_CustomPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_BudgetFilter_CustomPeriodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_BudgetFilter_CustomPeriodRef {
        BillingBudget_BudgetFilter_CustomPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_BudgetFilter_CustomPeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<BillingBudget_BudgetFilter_CustomPeriod_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<BillingBudget_BudgetFilter_CustomPeriod_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
}

#[derive(Serialize, Default)]
struct BillingBudget_BudgetFilterDynamic {
    custom_period: std::option::Option<terrars::DynamicBlock<BillingBudget_BudgetFilter_CustomPeriod>>,
}

#[derive(Serialize)]
pub struct BillingBudget_BudgetFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    calendar_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credit_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credit_types_treatment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projects: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_ancestors: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subaccounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_period: std::option::Option<std::vec::Vec<BillingBudget_BudgetFilter_CustomPeriod>>,
    dynamic: BillingBudget_BudgetFilterDynamic,
}

impl BillingBudget_BudgetFilter {
    #[doc =
        "Set the field `calendar_period`.\nA CalendarPeriod represents the abstract concept of a recurring time period that has a\ncanonical start. Grammatically, \"the start of the current CalendarPeriod\".\nAll calendar times begin at 12 AM US and Canadian Pacific Time (UTC-8).\n\nExactly one of 'calendar_period', 'custom_period' must be provided. Possible values: [\"MONTH\", \"QUARTER\", \"YEAR\", \"CALENDAR_PERIOD_UNSPECIFIED\"]"]
    pub fn set_calendar_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.calendar_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `credit_types`.\nOptional. If creditTypesTreatment is INCLUDE_SPECIFIED_CREDITS,\nthis is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. See a list of acceptable credit type values.\nIf creditTypesTreatment is not INCLUDE_SPECIFIED_CREDITS, this field must be empty."]
    pub fn set_credit_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.credit_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `credit_types_treatment`.\nSpecifies how credits should be treated when determining spend\nfor threshold calculations. Default value: \"INCLUDE_ALL_CREDITS\" Possible values: [\"INCLUDE_ALL_CREDITS\", \"EXCLUDE_ALL_CREDITS\", \"INCLUDE_SPECIFIED_CREDITS\"]"]
    pub fn set_credit_types_treatment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.credit_types_treatment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA single label and value pair specifying that usage from only\nthis set of labeled resources should be included in the budget."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `projects`.\nA set of projects of the form projects/{project_number},\nspecifying that usage from only this set of projects should be\nincluded in the budget. If omitted, the report will include\nall usage for the billing account, regardless of which project\nthe usage occurred on."]
    pub fn set_projects(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.projects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_ancestors`.\nA set of folder and organization names of the form folders/{folderId} or organizations/{organizationId},\nspecifying that usage from only this set of folders and organizations should be included in the budget.\nIf omitted, the budget includes all usage that the billing account pays for. If the folder or organization\ncontains projects that are paid for by a different Cloud Billing account, the budget doesn't apply to those projects."]
    pub fn set_resource_ancestors(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_ancestors = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `services`.\nA set of services of the form services/{service_id},\nspecifying that usage from only this set of services should be\nincluded in the budget. If omitted, the report will include\nusage for all the services. The service names are available\nthrough the Catalog API:\nhttps://cloud.google.com/billing/v1/how-tos/catalog-api."]
    pub fn set_services(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subaccounts`.\nA set of subaccounts of the form billingAccounts/{account_id},\nspecifying that usage from only this set of subaccounts should\nbe included in the budget. If a subaccount is set to the name of\nthe parent account, usage from the parent account will be included.\nIf the field is omitted, the report will include usage from the parent\naccount and all subaccounts, if they exist."]
    pub fn set_subaccounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subaccounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_period`.\n"]
    pub fn set_custom_period(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingBudget_BudgetFilter_CustomPeriod>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_period = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BillingBudget_BudgetFilter {
    type O = terrars::BlockAssignable<BillingBudget_BudgetFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_BudgetFilter {}

impl BuildBillingBudget_BudgetFilter {
    pub fn build(self) -> BillingBudget_BudgetFilter {
        BillingBudget_BudgetFilter {
            calendar_period: core::default::Default::default(),
            credit_types: core::default::Default::default(),
            credit_types_treatment: core::default::Default::default(),
            labels: core::default::Default::default(),
            projects: core::default::Default::default(),
            resource_ancestors: core::default::Default::default(),
            services: core::default::Default::default(),
            subaccounts: core::default::Default::default(),
            custom_period: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_BudgetFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_BudgetFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_BudgetFilterRef {
        BillingBudget_BudgetFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_BudgetFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `calendar_period` after provisioning.\nA CalendarPeriod represents the abstract concept of a recurring time period that has a\ncanonical start. Grammatically, \"the start of the current CalendarPeriod\".\nAll calendar times begin at 12 AM US and Canadian Pacific Time (UTC-8).\n\nExactly one of 'calendar_period', 'custom_period' must be provided. Possible values: [\"MONTH\", \"QUARTER\", \"YEAR\", \"CALENDAR_PERIOD_UNSPECIFIED\"]"]
    pub fn calendar_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.calendar_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `credit_types` after provisioning.\nOptional. If creditTypesTreatment is INCLUDE_SPECIFIED_CREDITS,\nthis is a list of credit types to be subtracted from gross cost to determine the spend for threshold calculations. See a list of acceptable credit type values.\nIf creditTypesTreatment is not INCLUDE_SPECIFIED_CREDITS, this field must be empty."]
    pub fn credit_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `credit_types_treatment` after provisioning.\nSpecifies how credits should be treated when determining spend\nfor threshold calculations. Default value: \"INCLUDE_ALL_CREDITS\" Possible values: [\"INCLUDE_ALL_CREDITS\", \"EXCLUDE_ALL_CREDITS\", \"INCLUDE_SPECIFIED_CREDITS\"]"]
    pub fn credit_types_treatment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credit_types_treatment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA single label and value pair specifying that usage from only\nthis set of labeled resources should be included in the budget."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `projects` after provisioning.\nA set of projects of the form projects/{project_number},\nspecifying that usage from only this set of projects should be\nincluded in the budget. If omitted, the report will include\nall usage for the billing account, regardless of which project\nthe usage occurred on."]
    pub fn projects(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_ancestors` after provisioning.\nA set of folder and organization names of the form folders/{folderId} or organizations/{organizationId},\nspecifying that usage from only this set of folders and organizations should be included in the budget.\nIf omitted, the budget includes all usage that the billing account pays for. If the folder or organization\ncontains projects that are paid for by a different Cloud Billing account, the budget doesn't apply to those projects."]
    pub fn resource_ancestors(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_ancestors", self.base))
    }

    #[doc =
        "Get a reference to the value of field `services` after provisioning.\nA set of services of the form services/{service_id},\nspecifying that usage from only this set of services should be\nincluded in the budget. If omitted, the report will include\nusage for all the services. The service names are available\nthrough the Catalog API:\nhttps://cloud.google.com/billing/v1/how-tos/catalog-api."]
    pub fn services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subaccounts` after provisioning.\nA set of subaccounts of the form billingAccounts/{account_id},\nspecifying that usage from only this set of subaccounts should\nbe included in the budget. If a subaccount is set to the name of\nthe parent account, usage from the parent account will be included.\nIf the field is omitted, the report will include usage from the parent\naccount and all subaccounts, if they exist."]
    pub fn subaccounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subaccounts", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_period` after provisioning.\n"]
    pub fn custom_period(&self) -> terrars::ListRef<BillingBudget_BudgetFilter_CustomPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_period", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingBudget_ThresholdRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spend_basis: std::option::Option<terrars::PrimField<std::string::String>>,
    threshold_percent: terrars::PrimField<f64>,
}

impl BillingBudget_ThresholdRules {
    #[doc =
        "Set the field `spend_basis`.\nThe type of basis used to determine if spend has passed\nthe threshold. Default value: \"CURRENT_SPEND\" Possible values: [\"CURRENT_SPEND\", \"FORECASTED_SPEND\"]"]
    pub fn set_spend_basis(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spend_basis = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BillingBudget_ThresholdRules {
    type O = terrars::BlockAssignable<BillingBudget_ThresholdRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_ThresholdRules {
    #[doc =
        "Send an alert when this threshold is exceeded. This is a\n1.0-based percentage, so 0.5 = 50%. Must be >= 0."]
    pub threshold_percent: terrars::PrimField<f64>,
}

impl BuildBillingBudget_ThresholdRules {
    pub fn build(self) -> BillingBudget_ThresholdRules {
        BillingBudget_ThresholdRules {
            spend_basis: core::default::Default::default(),
            threshold_percent: self.threshold_percent,
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_ThresholdRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_ThresholdRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_ThresholdRulesRef {
        BillingBudget_ThresholdRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_ThresholdRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `spend_basis` after provisioning.\nThe type of basis used to determine if spend has passed\nthe threshold. Default value: \"CURRENT_SPEND\" Possible values: [\"CURRENT_SPEND\", \"FORECASTED_SPEND\"]"]
    pub fn spend_basis(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spend_basis", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threshold_percent` after provisioning.\nSend an alert when this threshold is exceeded. This is a\n1.0-based percentage, so 0.5 = 50%. Must be >= 0."]
    pub fn threshold_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingBudget_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BillingBudget_Timeouts {
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

impl terrars::ToListMappable for BillingBudget_Timeouts {
    type O = terrars::BlockAssignable<BillingBudget_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingBudget_Timeouts {}

impl BuildBillingBudget_Timeouts {
    pub fn build(self) -> BillingBudget_Timeouts {
        BillingBudget_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingBudget_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingBudget_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingBudget_TimeoutsRef {
        BillingBudget_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingBudget_TimeoutsRef {
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
