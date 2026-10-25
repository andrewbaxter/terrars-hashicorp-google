#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MonitoringSnoozeData_ {
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
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criteria: std::option::Option<std::vec::Vec<MonitoringSnooze_Criteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<std::vec::Vec<MonitoringSnooze_Interval>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MonitoringSnooze_Timeouts>,
    dynamic: MonitoringSnoozeDynamic,
}

struct MonitoringSnooze_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MonitoringSnoozeData_>,
}

#[derive(Clone)]
pub struct MonitoringSnooze(std::rc::Rc<MonitoringSnooze_>);

impl MonitoringSnooze {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `criteria`.\n"]
    pub fn set_criteria(self, v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSnooze_Criteria>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(self, v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSnooze_Interval>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interval = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interval = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MonitoringSnooze_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA display name for the Snooze. This can be, at most, 512 unicode characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Snooze. The format is:\nprojects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID]\nThe ID of the Snooze will be generated by the system."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> terrars::ListRef<MonitoringSnooze_CriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::ListRef<MonitoringSnooze_IntervalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringSnooze_TimeoutsRef {
        MonitoringSnooze_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MonitoringSnooze {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MonitoringSnooze { }

impl terrars::ToListMappable for MonitoringSnooze {
    type O = terrars::ListRef<MonitoringSnoozeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MonitoringSnooze_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_monitoring_snooze".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMonitoringSnooze {
    pub tf_id: std::string::String,
    #[doc = "A display name for the Snooze. This can be, at most, 512 unicode characters."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSnooze {
    pub fn build(self, stack: &mut terrars::Stack) -> MonitoringSnooze {
        let out = MonitoringSnooze(std::rc::Rc::new(MonitoringSnooze_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MonitoringSnoozeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                criteria: core::default::Default::default(),
                interval: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MonitoringSnoozeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSnoozeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MonitoringSnoozeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA display name for the Snooze. This can be, at most, 512 unicode characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Snooze. The format is:\nprojects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID]\nThe ID of the Snooze will be generated by the system."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> terrars::ListRef<MonitoringSnooze_CriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::ListRef<MonitoringSnooze_IntervalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringSnooze_TimeoutsRef {
        MonitoringSnooze_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSnoozeDynamic {
    criteria: std::option::Option<terrars::DynamicBlock<MonitoringSnooze_Criteria>>,
    interval: std::option::Option<terrars::DynamicBlock<MonitoringSnooze_Interval>>,
}

#[derive(Serialize)]
pub struct MonitoringSnooze_Criteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MonitoringSnooze_Criteria {
    #[doc =
        "Set the field `filter`.\nWhen you define a snooze, you can also define a filter for that snooze.\nThe filter is a string containing one or more key-value pairs. The string\nuses the standard https://google.aip.dev/160 filter syntax. If you define\na filter for a snooze, then the snooze can only apply to one alert policy.\nWhen the snooze is active, incidents won't be created when the incident\nwould have key-value pairs (labels) that match those specified by the\nfilter in the snooze.\n\nSnooze filters support resource, metric, and metadata labels. If multiple\nlabels are used, then they must be connected with an AND operator. For\nexample, the following filter applies the snooze to incidents that have a\nresource label with an instance ID of 1234567890, a metric label with an\ninstance name of test_group, a metadata user label with a key of foo and a\nvalue of bar, and a metadata system label with a key of region and a value\nof us-central1:\n\n\"filter\": \"resource.labels.instance_id=\\\"1234567890\\\" AND metric.labels.instance_name=\\\"test_group\\\" AND metadata.user_labels.foo=\\\"bar\\\" AND metadata.system_labels.region=\\\"us-central1\\\"\""]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policies`.\nThe specific AlertPolicy names for the alert that should be snoozed.\nThe format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]\nThere is a limit of 16 policies per snooze. This limit is checked during\nsnooze creation. Exactly 1 alert policy is required if filter is specified\nat the same time."]
    pub fn set_policies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.policies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSnooze_Criteria {
    type O = terrars::BlockAssignable<MonitoringSnooze_Criteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSnooze_Criteria {}

impl BuildMonitoringSnooze_Criteria {
    pub fn build(self) -> MonitoringSnooze_Criteria {
        MonitoringSnooze_Criteria {
            filter: core::default::Default::default(),
            policies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSnooze_CriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSnooze_CriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSnooze_CriteriaRef {
        MonitoringSnooze_CriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSnooze_CriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nWhen you define a snooze, you can also define a filter for that snooze.\nThe filter is a string containing one or more key-value pairs. The string\nuses the standard https://google.aip.dev/160 filter syntax. If you define\na filter for a snooze, then the snooze can only apply to one alert policy.\nWhen the snooze is active, incidents won't be created when the incident\nwould have key-value pairs (labels) that match those specified by the\nfilter in the snooze.\n\nSnooze filters support resource, metric, and metadata labels. If multiple\nlabels are used, then they must be connected with an AND operator. For\nexample, the following filter applies the snooze to incidents that have a\nresource label with an instance ID of 1234567890, a metric label with an\ninstance name of test_group, a metadata user label with a key of foo and a\nvalue of bar, and a metadata system label with a key of region and a value\nof us-central1:\n\n\"filter\": \"resource.labels.instance_id=\\\"1234567890\\\" AND metric.labels.instance_name=\\\"test_group\\\" AND metadata.user_labels.foo=\\\"bar\\\" AND metadata.system_labels.region=\\\"us-central1\\\"\""]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nThe specific AlertPolicy names for the alert that should be snoozed.\nThe format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]\nThere is a limit of 16 policies per snooze. This limit is checked during\nsnooze creation. Exactly 1 alert policy is required if filter is specified\nat the same time."]
    pub fn policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSnooze_Interval {
    end_time: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringSnooze_Interval {
    #[doc =
        "Set the field `start_time`.\nThe beginning of the time interval. The default value for the start time\nis the end time. The start time must not be later than the end time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSnooze_Interval {
    type O = terrars::BlockAssignable<MonitoringSnooze_Interval>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSnooze_Interval {
    #[doc =
        "The end of the time interval.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub end_time: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSnooze_Interval {
    pub fn build(self) -> MonitoringSnooze_Interval {
        MonitoringSnooze_Interval {
            end_time: self.end_time,
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSnooze_IntervalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSnooze_IntervalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSnooze_IntervalRef {
        MonitoringSnooze_IntervalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSnooze_IntervalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nThe end of the time interval.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe beginning of the time interval. The default value for the start time\nis the end time. The start time must not be later than the end time.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and\nup to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSnooze_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringSnooze_Timeouts {
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

impl terrars::ToListMappable for MonitoringSnooze_Timeouts {
    type O = terrars::BlockAssignable<MonitoringSnooze_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSnooze_Timeouts {}

impl BuildMonitoringSnooze_Timeouts {
    pub fn build(self) -> MonitoringSnooze_Timeouts {
        MonitoringSnooze_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSnooze_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSnooze_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSnooze_TimeoutsRef {
        MonitoringSnooze_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSnooze_TimeoutsRef {
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
