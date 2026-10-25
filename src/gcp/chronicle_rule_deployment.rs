#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleRuleDeploymentData_ {
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
    alerting: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archived: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    rule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_customizations: std::option::Option<std::vec::Vec<ChronicleRuleDeployment_ScheduleCustomizations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleRuleDeployment_Timeouts>,
    dynamic: ChronicleRuleDeploymentDynamic,
}

struct ChronicleRuleDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleRuleDeploymentData_>,
}

#[derive(Clone)]
pub struct ChronicleRuleDeployment(std::rc::Rc<ChronicleRuleDeployment_>);

impl ChronicleRuleDeployment {
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
        "Set the field `alerting`.\nWhether detections resulting from this deployment should be considered\nalerts."]
    pub fn set_alerting(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().alerting = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `archived`.\nThe archive state of the rule deployment.\nCannot be set to true unless enabled is set to false i.e.\narchiving requires a two-step process: first, disable the rule by\nsetting 'enabled' to false, then set 'archive' to true.\nIf set to true, alerting will automatically be set to false.\nIf currently set to true, enabled, alerting, and run_frequency cannot be\nupdated."]
    pub fn set_archived(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().archived = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the rule is currently deployed continuously against incoming data."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
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
        "Set the field `run_frequency`.\nThe run frequency of the rule deployment.\nPossible values:\nLIVE\nHOURLY\nDAILY\nLIVE_CUSTOMIZABLE\nHOURLY_CUSTOMIZABLE\n\nNote: Certain legacy run frequencies are deprecated. For multi-event rules, use LIVE_CUSTOMIZABLE or HOURLY_CUSTOMIZABLE (for match windows <=2d), or DAILY (for match windows >2d).\nLegacy values LIVE and HOURLY are mapped to their customizable counterparts on the backend. DAILY for <=2d match window multi-event rules will be happed to HOURLY_CUSTOMIZABLE.\nFor single-event rules, HOURLY and DAILY are deprecated and mapped to LIVE. If you continue to use deprecated values in your Terraform configuration, Terraform will silently\nsuppress the diff and ignore the changes to prevent infinite update loops."]
    pub fn set_run_frequency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().run_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_customizations`.\n"]
    pub fn set_schedule_customizations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleRuleDeployment_ScheduleCustomizations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_customizations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_customizations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleRuleDeployment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `alerting` after provisioning.\nWhether detections resulting from this deployment should be considered\nalerts."]
    pub fn alerting(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alerting", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `archive_time` after provisioning.\nOutput only. The timestamp when the rule deployment archive state was last set to true. If the rule deployment's current archive state is not set to true, the field will be empty."]
    pub fn archive_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `archived` after provisioning.\nThe archive state of the rule deployment.\nCannot be set to true unless enabled is set to false i.e.\narchiving requires a two-step process: first, disable the rule by\nsetting 'enabled' to false, then set 'archive' to true.\nIf set to true, alerting will automatically be set to false.\nIf currently set to true, enabled, alerting, and run_frequency cannot be\nupdated."]
    pub fn archived(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_rules` after provisioning.\nOutput only. The names of the associated/chained consumer rules. Rules are considered\nconsumers of this rule if their rule text explicitly filters on this rule's ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn consumer_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumer_rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is currently deployed continuously against incoming data."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_state` after provisioning.\nThe execution state of the rule deployment.\nPossible values:\nDEFAULT\nLIMITED\nPAUSED"]
    pub fn execution_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_alert_status_change_time` after provisioning.\nOutput only. The timestamp when the rule deployment alert state was lastly changed. This is filled regardless of the current alert state.E.g. if the current alert status is false, this timestamp will be the timestamp when the alert status was changed to false."]
    pub fn last_alert_status_change_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_alert_status_change_time", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the rule deployment.\nNote that RuleDeployment is a child of the overall Rule, not any individual\nrevision, so the resource ID segment for the Rule resource must not\nreference a specific revision.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}/deployment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `producer_rules` after provisioning.\nOutput only. The names of the associated/chained producer rules. Rules are considered\nproducers for this rule if this rule explicitly filters on their ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn producer_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.producer_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe Rule ID of the rule."]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `run_frequency` after provisioning.\nThe run frequency of the rule deployment.\nPossible values:\nLIVE\nHOURLY\nDAILY\nLIVE_CUSTOMIZABLE\nHOURLY_CUSTOMIZABLE\n\nNote: Certain legacy run frequencies are deprecated. For multi-event rules, use LIVE_CUSTOMIZABLE or HOURLY_CUSTOMIZABLE (for match windows <=2d), or DAILY (for match windows >2d).\nLegacy values LIVE and HOURLY are mapped to their customizable counterparts on the backend. DAILY for <=2d match window multi-event rules will be happed to HOURLY_CUSTOMIZABLE.\nFor single-event rules, HOURLY and DAILY are deprecated and mapped to LIVE. If you continue to use deprecated values in your Terraform configuration, Terraform will silently\nsuppress the diff and ignore the changes to prevent infinite update loops."]
    pub fn run_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_customizations` after provisioning.\n"]
    pub fn schedule_customizations(&self) -> terrars::ListRef<ChronicleRuleDeployment_ScheduleCustomizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_customizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRuleDeployment_TimeoutsRef {
        ChronicleRuleDeployment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleRuleDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleRuleDeployment { }

impl terrars::ToListMappable for ChronicleRuleDeployment {
    type O = terrars::ListRef<ChronicleRuleDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleRuleDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_rule_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleRuleDeployment {
    pub tf_id: std::string::String,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The Rule ID of the rule."]
    pub rule: terrars::PrimField<std::string::String>,
}

impl BuildChronicleRuleDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleRuleDeployment {
        let out = ChronicleRuleDeployment(std::rc::Rc::new(ChronicleRuleDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleRuleDeploymentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alerting: core::default::Default::default(),
                archived: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                rule: self.rule,
                run_frequency: core::default::Default::default(),
                schedule_customizations: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleRuleDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRuleDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleRuleDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alerting` after provisioning.\nWhether detections resulting from this deployment should be considered\nalerts."]
    pub fn alerting(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alerting", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `archive_time` after provisioning.\nOutput only. The timestamp when the rule deployment archive state was last set to true. If the rule deployment's current archive state is not set to true, the field will be empty."]
    pub fn archive_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `archived` after provisioning.\nThe archive state of the rule deployment.\nCannot be set to true unless enabled is set to false i.e.\narchiving requires a two-step process: first, disable the rule by\nsetting 'enabled' to false, then set 'archive' to true.\nIf set to true, alerting will automatically be set to false.\nIf currently set to true, enabled, alerting, and run_frequency cannot be\nupdated."]
    pub fn archived(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archived", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_rules` after provisioning.\nOutput only. The names of the associated/chained consumer rules. Rules are considered\nconsumers of this rule if their rule text explicitly filters on this rule's ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn consumer_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumer_rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the rule is currently deployed continuously against incoming data."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_state` after provisioning.\nThe execution state of the rule deployment.\nPossible values:\nDEFAULT\nLIMITED\nPAUSED"]
    pub fn execution_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_alert_status_change_time` after provisioning.\nOutput only. The timestamp when the rule deployment alert state was lastly changed. This is filled regardless of the current alert state.E.g. if the current alert status is false, this timestamp will be the timestamp when the alert status was changed to false."]
    pub fn last_alert_status_change_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_alert_status_change_time", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the rule deployment.\nNote that RuleDeployment is a child of the overall Rule, not any individual\nrevision, so the resource ID segment for the Rule resource must not\nreference a specific revision.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}/deployment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `producer_rules` after provisioning.\nOutput only. The names of the associated/chained producer rules. Rules are considered\nproducers for this rule if this rule explicitly filters on their ruleid.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn producer_rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.producer_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\nThe Rule ID of the rule."]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `run_frequency` after provisioning.\nThe run frequency of the rule deployment.\nPossible values:\nLIVE\nHOURLY\nDAILY\nLIVE_CUSTOMIZABLE\nHOURLY_CUSTOMIZABLE\n\nNote: Certain legacy run frequencies are deprecated. For multi-event rules, use LIVE_CUSTOMIZABLE or HOURLY_CUSTOMIZABLE (for match windows <=2d), or DAILY (for match windows >2d).\nLegacy values LIVE and HOURLY are mapped to their customizable counterparts on the backend. DAILY for <=2d match window multi-event rules will be happed to HOURLY_CUSTOMIZABLE.\nFor single-event rules, HOURLY and DAILY are deprecated and mapped to LIVE. If you continue to use deprecated values in your Terraform configuration, Terraform will silently\nsuppress the diff and ignore the changes to prevent infinite update loops."]
    pub fn run_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_customizations` after provisioning.\n"]
    pub fn schedule_customizations(&self) -> terrars::ListRef<ChronicleRuleDeployment_ScheduleCustomizationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_customizations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRuleDeployment_TimeoutsRef {
        ChronicleRuleDeployment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleRuleDeploymentDynamic {
    schedule_customizations: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleRuleDeployment_ScheduleCustomizations>>,
}

#[derive(Serialize)]
pub struct ChronicleRuleDeployment_ScheduleCustomizations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ensure_enrichment_completeness: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    late_arriving_data_adjustment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleRuleDeployment_ScheduleCustomizations {
    #[doc =
        "Set the field `ensure_enrichment_completeness`.\nIndicates whether to add additional delays and runs to rules to ensure\nenrichment completeness, with the trade-off of more late-arriving detections."]
    pub fn set_ensure_enrichment_completeness(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ensure_enrichment_completeness = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `late_arriving_data_adjustment`.\nDelay the first rule execution run to account for late-arriving data."]
    pub fn set_late_arriving_data_adjustment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.late_arriving_data_adjustment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleRuleDeployment_ScheduleCustomizations {
    type O = terrars::BlockAssignable<ChronicleRuleDeployment_ScheduleCustomizations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleRuleDeployment_ScheduleCustomizations {}

impl BuildChronicleRuleDeployment_ScheduleCustomizations {
    pub fn build(self) -> ChronicleRuleDeployment_ScheduleCustomizations {
        ChronicleRuleDeployment_ScheduleCustomizations {
            ensure_enrichment_completeness: core::default::Default::default(),
            late_arriving_data_adjustment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleRuleDeployment_ScheduleCustomizationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRuleDeployment_ScheduleCustomizationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleRuleDeployment_ScheduleCustomizationsRef {
        ChronicleRuleDeployment_ScheduleCustomizationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleRuleDeployment_ScheduleCustomizationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ensure_enrichment_completeness` after provisioning.\nIndicates whether to add additional delays and runs to rules to ensure\nenrichment completeness, with the trade-off of more late-arriving detections."]
    pub fn ensure_enrichment_completeness(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ensure_enrichment_completeness", self.base))
    }

    #[doc =
        "Get a reference to the value of field `late_arriving_data_adjustment` after provisioning.\nDelay the first rule execution run to account for late-arriving data."]
    pub fn late_arriving_data_adjustment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.late_arriving_data_adjustment", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleRuleDeployment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleRuleDeployment_Timeouts {
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

impl terrars::ToListMappable for ChronicleRuleDeployment_Timeouts {
    type O = terrars::BlockAssignable<ChronicleRuleDeployment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleRuleDeployment_Timeouts {}

impl BuildChronicleRuleDeployment_Timeouts {
    pub fn build(self) -> ChronicleRuleDeployment_Timeouts {
        ChronicleRuleDeployment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleRuleDeployment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRuleDeployment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleRuleDeployment_TimeoutsRef {
        ChronicleRuleDeployment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleRuleDeployment_TimeoutsRef {
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
