#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MonitoringSloData_ {
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
    calendar_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    goal: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rolling_period_days: std::option::Option<terrars::PrimField<f64>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slo_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_sli: std::option::Option<std::vec::Vec<MonitoringSlo_BasicSli>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_based_sli: std::option::Option<std::vec::Vec<MonitoringSlo_RequestBasedSli>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MonitoringSlo_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_based_sli: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli>>,
    dynamic: MonitoringSloDynamic,
}

struct MonitoringSlo_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MonitoringSloData_>,
}

#[derive(Clone)]
pub struct MonitoringSlo(std::rc::Rc<MonitoringSlo_>);

impl MonitoringSlo {
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
        "Set the field `calendar_period`.\nA calendar period, semantically \"since the start of the current\n<calendarPeriod>\". Possible values: [\"DAY\", \"WEEK\", \"FORTNIGHT\", \"MONTH\"]"]
    pub fn set_calendar_period(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().calendar_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nName used for UI elements listing this SLO."]
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

    #[doc =
        "Set the field `rolling_period_days`.\nA rolling time period, semantically \"in the past X days\".\nMust be between 1 to 30 days, inclusive."]
    pub fn set_rolling_period_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().rolling_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `slo_id`.\nThe id to use for this ServiceLevelObjective. If omitted, an id will be generated instead."]
    pub fn set_slo_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().slo_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_labels`.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn set_user_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_sli`.\n"]
    pub fn set_basic_sli(self, v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_BasicSli>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic_sli = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic_sli = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_based_sli`.\n"]
    pub fn set_request_based_sli(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_RequestBasedSli>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().request_based_sli = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.request_based_sli = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MonitoringSlo_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `windows_based_sli`.\n"]
    pub fn set_windows_based_sli(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().windows_based_sli = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.windows_based_sli = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `calendar_period` after provisioning.\nA calendar period, semantically \"since the start of the current\n<calendarPeriod>\". Possible values: [\"DAY\", \"WEEK\", \"FORTNIGHT\", \"MONTH\"]"]
    pub fn calendar_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.calendar_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName used for UI elements listing this SLO."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goal` after provisioning.\nThe fraction of service that must be good in order for this objective\nto be met. 0 < goal <= 0.999"]
    pub fn goal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name for this service. The syntax is:\nprojects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rolling_period_days` after provisioning.\nA rolling time period, semantically \"in the past X days\".\nMust be between 1 to 30 days, inclusive."]
    pub fn rolling_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rolling_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nID of the service to which this SLO belongs."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `slo_id` after provisioning.\nThe id to use for this ServiceLevelObjective. If omitted, an id will be generated instead."]
    pub fn slo_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slo_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_sli` after provisioning.\n"]
    pub fn basic_sli(&self) -> terrars::ListRef<MonitoringSlo_BasicSliRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_sli", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_based_sli` after provisioning.\n"]
    pub fn request_based_sli(&self) -> terrars::ListRef<MonitoringSlo_RequestBasedSliRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_based_sli", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringSlo_TimeoutsRef {
        MonitoringSlo_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `windows_based_sli` after provisioning.\n"]
    pub fn windows_based_sli(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSliRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_based_sli", self.extract_ref()))
    }
}

impl terrars::Referable for MonitoringSlo {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MonitoringSlo { }

impl terrars::ToListMappable for MonitoringSlo {
    type O = terrars::ListRef<MonitoringSloRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MonitoringSlo_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_monitoring_slo".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMonitoringSlo {
    pub tf_id: std::string::String,
    #[doc = "The fraction of service that must be good in order for this objective\nto be met. 0 < goal <= 0.999"]
    pub goal: terrars::PrimField<f64>,
    #[doc = "ID of the service to which this SLO belongs."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSlo {
    pub fn build(self, stack: &mut terrars::Stack) -> MonitoringSlo {
        let out = MonitoringSlo(std::rc::Rc::new(MonitoringSlo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MonitoringSloData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                calendar_period: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                goal: self.goal,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                rolling_period_days: core::default::Default::default(),
                service: self.service,
                slo_id: core::default::Default::default(),
                user_labels: core::default::Default::default(),
                basic_sli: core::default::Default::default(),
                request_based_sli: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                windows_based_sli: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MonitoringSloRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSloRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MonitoringSloRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `calendar_period` after provisioning.\nA calendar period, semantically \"since the start of the current\n<calendarPeriod>\". Possible values: [\"DAY\", \"WEEK\", \"FORTNIGHT\", \"MONTH\"]"]
    pub fn calendar_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.calendar_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName used for UI elements listing this SLO."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goal` after provisioning.\nThe fraction of service that must be good in order for this objective\nto be met. 0 < goal <= 0.999"]
    pub fn goal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name for this service. The syntax is:\nprojects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rolling_period_days` after provisioning.\nA rolling time period, semantically \"in the past X days\".\nMust be between 1 to 30 days, inclusive."]
    pub fn rolling_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rolling_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nID of the service to which this SLO belongs."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `slo_id` after provisioning.\nThe id to use for this ServiceLevelObjective. If omitted, an id will be generated instead."]
    pub fn slo_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slo_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nThis field is intended to be used for organizing and identifying the AlertPolicy\nobjects.The field can contain up to 64 entries. Each key and value is limited\nto 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values\ncan contain only lowercase letters, numerals, underscores, and dashes. Keys\nmust begin with a letter."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_sli` after provisioning.\n"]
    pub fn basic_sli(&self) -> terrars::ListRef<MonitoringSlo_BasicSliRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_sli", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_based_sli` after provisioning.\n"]
    pub fn request_based_sli(&self) -> terrars::ListRef<MonitoringSlo_RequestBasedSliRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_based_sli", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringSlo_TimeoutsRef {
        MonitoringSlo_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `windows_based_sli` after provisioning.\n"]
    pub fn windows_based_sli(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSliRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_based_sli", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSloDynamic {
    basic_sli: std::option::Option<terrars::DynamicBlock<MonitoringSlo_BasicSli>>,
    request_based_sli: std::option::Option<terrars::DynamicBlock<MonitoringSlo_RequestBasedSli>>,
    windows_based_sli: std::option::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_BasicSli_Availability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl MonitoringSlo_BasicSli_Availability {
    #[doc =
        "Set the field `enabled`.\nWhether an availability SLI is enabled or not. Must be set to true. Defaults to 'true'."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_BasicSli_Availability {
    type O = terrars::BlockAssignable<MonitoringSlo_BasicSli_Availability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_BasicSli_Availability {}

impl BuildMonitoringSlo_BasicSli_Availability {
    pub fn build(self) -> MonitoringSlo_BasicSli_Availability {
        MonitoringSlo_BasicSli_Availability { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_BasicSli_AvailabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_BasicSli_AvailabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSlo_BasicSli_AvailabilityRef {
        MonitoringSlo_BasicSli_AvailabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_BasicSli_AvailabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether an availability SLI is enabled or not. Must be set to true. Defaults to 'true'."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSlo_BasicSli_Latency {
    threshold: terrars::PrimField<std::string::String>,
}

impl MonitoringSlo_BasicSli_Latency { }

impl terrars::ToListMappable for MonitoringSlo_BasicSli_Latency {
    type O = terrars::BlockAssignable<MonitoringSlo_BasicSli_Latency>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_BasicSli_Latency {
    #[doc =
        "A duration string, e.g. 10s.\nGood service is defined to be the count of requests made to\nthis service that return in no more than threshold."]
    pub threshold: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSlo_BasicSli_Latency {
    pub fn build(self) -> MonitoringSlo_BasicSli_Latency {
        MonitoringSlo_BasicSli_Latency { threshold: self.threshold }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_BasicSli_LatencyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_BasicSli_LatencyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSlo_BasicSli_LatencyRef {
        MonitoringSlo_BasicSli_LatencyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_BasicSli_LatencyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nA duration string, e.g. 10s.\nGood service is defined to be the count of requests made to\nthis service that return in no more than threshold."]
    pub fn threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_BasicSliDynamic {
    availability: std::option::Option<terrars::DynamicBlock<MonitoringSlo_BasicSli_Availability>>,
    latency: std::option::Option<terrars::DynamicBlock<MonitoringSlo_BasicSli_Latency>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_BasicSli {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability: std::option::Option<std::vec::Vec<MonitoringSlo_BasicSli_Availability>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latency: std::option::Option<std::vec::Vec<MonitoringSlo_BasicSli_Latency>>,
    dynamic: MonitoringSlo_BasicSliDynamic,
}

impl MonitoringSlo_BasicSli {
    #[doc =
        "Set the field `location`.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn set_method(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn set_version(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability`.\n"]
    pub fn set_availability(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_BasicSli_Availability>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.availability = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.availability = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `latency`.\n"]
    pub fn set_latency(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_BasicSli_Latency>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.latency = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.latency = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_BasicSli {
    type O = terrars::BlockAssignable<MonitoringSlo_BasicSli>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_BasicSli {}

impl BuildMonitoringSlo_BasicSli {
    pub fn build(self) -> MonitoringSlo_BasicSli {
        MonitoringSlo_BasicSli {
            location: core::default::Default::default(),
            method: core::default::Default::default(),
            version: core::default::Default::default(),
            availability: core::default::Default::default(),
            latency: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_BasicSliRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_BasicSliRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSlo_BasicSliRef {
        MonitoringSlo_BasicSliRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_BasicSliRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn location(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn method(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn version(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `availability` after provisioning.\n"]
    pub fn availability(&self) -> terrars::ListRef<MonitoringSlo_BasicSli_AvailabilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability", self.base))
    }

    #[doc = "Get a reference to the value of field `latency` after provisioning.\n"]
    pub fn latency(&self) -> terrars::ListRef<MonitoringSlo_BasicSli_LatencyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latency", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSlo_RequestBasedSli_DistributionCut_Range {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl MonitoringSlo_RequestBasedSli_DistributionCut_Range {
    #[doc = "Set the field `max`.\nmax value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\nMin value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_RequestBasedSli_DistributionCut_Range {
    type O = terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_DistributionCut_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_RequestBasedSli_DistributionCut_Range {}

impl BuildMonitoringSlo_RequestBasedSli_DistributionCut_Range {
    pub fn build(self) -> MonitoringSlo_RequestBasedSli_DistributionCut_Range {
        MonitoringSlo_RequestBasedSli_DistributionCut_Range {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_RequestBasedSli_DistributionCut_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_RequestBasedSli_DistributionCut_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_RequestBasedSli_DistributionCut_RangeRef {
        MonitoringSlo_RequestBasedSli_DistributionCut_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_RequestBasedSli_DistributionCut_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max` after provisioning.\nmax value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min` after provisioning.\nMin value for the range (inclusive). If not given,\nwill be set to 0"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_RequestBasedSli_DistributionCutDynamic {
    range: std::option::Option<terrars::DynamicBlock<MonitoringSlo_RequestBasedSli_DistributionCut_Range>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_RequestBasedSli_DistributionCut {
    distribution_filter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<MonitoringSlo_RequestBasedSli_DistributionCut_Range>>,
    dynamic: MonitoringSlo_RequestBasedSli_DistributionCutDynamic,
}

impl MonitoringSlo_RequestBasedSli_DistributionCut {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_DistributionCut_Range>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_RequestBasedSli_DistributionCut {
    type O = terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_DistributionCut>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_RequestBasedSli_DistributionCut {
    #[doc =
        "A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\naggregating values to quantify the good service provided.\n\nMust have ValueType = DISTRIBUTION and\nMetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub distribution_filter: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSlo_RequestBasedSli_DistributionCut {
    pub fn build(self) -> MonitoringSlo_RequestBasedSli_DistributionCut {
        MonitoringSlo_RequestBasedSli_DistributionCut {
            distribution_filter: self.distribution_filter,
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_RequestBasedSli_DistributionCutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_RequestBasedSli_DistributionCutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_RequestBasedSli_DistributionCutRef {
        MonitoringSlo_RequestBasedSli_DistributionCutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_RequestBasedSli_DistributionCutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `distribution_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\naggregating values to quantify the good service provided.\n\nMust have ValueType = DISTRIBUTION and\nMetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn distribution_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<MonitoringSlo_RequestBasedSli_DistributionCut_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSlo_RequestBasedSli_GoodTotalRatio {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bad_service_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    good_service_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_service_filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringSlo_RequestBasedSli_GoodTotalRatio {
    #[doc =
        "Set the field `bad_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn set_bad_service_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bad_service_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `good_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided.\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn set_good_service_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.good_service_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn set_total_service_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_service_filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_RequestBasedSli_GoodTotalRatio {
    type O = terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_GoodTotalRatio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_RequestBasedSli_GoodTotalRatio {}

impl BuildMonitoringSlo_RequestBasedSli_GoodTotalRatio {
    pub fn build(self) -> MonitoringSlo_RequestBasedSli_GoodTotalRatio {
        MonitoringSlo_RequestBasedSli_GoodTotalRatio {
            bad_service_filter: core::default::Default::default(),
            good_service_filter: core::default::Default::default(),
            total_service_filter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_RequestBasedSli_GoodTotalRatioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_RequestBasedSli_GoodTotalRatioRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_RequestBasedSli_GoodTotalRatioRef {
        MonitoringSlo_RequestBasedSli_GoodTotalRatioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_RequestBasedSli_GoodTotalRatioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bad_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn bad_service_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bad_service_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `good_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided.\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn good_service_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.good_service_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service.\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE.\n\nExactly two of 'good_service_filter','bad_service_filter','total_service_filter'\nmust be set (good + bad = total is assumed)."]
    pub fn total_service_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_service_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_RequestBasedSliDynamic {
    distribution_cut: std::option::Option<terrars::DynamicBlock<MonitoringSlo_RequestBasedSli_DistributionCut>>,
    good_total_ratio: std::option::Option<terrars::DynamicBlock<MonitoringSlo_RequestBasedSli_GoodTotalRatio>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_RequestBasedSli {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution_cut: std::option::Option<std::vec::Vec<MonitoringSlo_RequestBasedSli_DistributionCut>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    good_total_ratio: std::option::Option<std::vec::Vec<MonitoringSlo_RequestBasedSli_GoodTotalRatio>>,
    dynamic: MonitoringSlo_RequestBasedSliDynamic,
}

impl MonitoringSlo_RequestBasedSli {
    #[doc = "Set the field `distribution_cut`.\n"]
    pub fn set_distribution_cut(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_DistributionCut>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.distribution_cut = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.distribution_cut = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `good_total_ratio`.\n"]
    pub fn set_good_total_ratio(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_GoodTotalRatio>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.good_total_ratio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.good_total_ratio = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_RequestBasedSli {
    type O = terrars::BlockAssignable<MonitoringSlo_RequestBasedSli>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_RequestBasedSli {}

impl BuildMonitoringSlo_RequestBasedSli {
    pub fn build(self) -> MonitoringSlo_RequestBasedSli {
        MonitoringSlo_RequestBasedSli {
            distribution_cut: core::default::Default::default(),
            good_total_ratio: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_RequestBasedSliRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_RequestBasedSliRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSlo_RequestBasedSliRef {
        MonitoringSlo_RequestBasedSliRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_RequestBasedSliRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `distribution_cut` after provisioning.\n"]
    pub fn distribution_cut(&self) -> terrars::ListRef<MonitoringSlo_RequestBasedSli_DistributionCutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.distribution_cut", self.base))
    }

    #[doc = "Get a reference to the value of field `good_total_ratio` after provisioning.\n"]
    pub fn good_total_ratio(&self) -> terrars::ListRef<MonitoringSlo_RequestBasedSli_GoodTotalRatioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.good_total_ratio", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSlo_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringSlo_Timeouts {
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

impl terrars::ToListMappable for MonitoringSlo_Timeouts {
    type O = terrars::BlockAssignable<MonitoringSlo_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_Timeouts {}

impl BuildMonitoringSlo_Timeouts {
    pub fn build(self) -> MonitoringSlo_Timeouts {
        MonitoringSlo_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSlo_TimeoutsRef {
        MonitoringSlo_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_TimeoutsRef {
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
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {
    #[doc =
        "Set the field `enabled`.\nWhether an availability SLI is enabled or not. Must be set to 'true. Defaults to 'true'."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {
    type O =
        terrars
        ::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {}

impl BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_AvailabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_AvailabilityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_AvailabilityRef {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_AvailabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_AvailabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether an availability SLI is enabled or not. Must be set to 'true. Defaults to 'true'."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceDynamic {
    availability: std
    ::option
    ::Option<
        terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability>,
    >,
    latency: std::option::Option<terrars::DynamicBlock<MonitoringSlo_BasicSli_Latency>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability: std
    ::option
    ::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latency: std::option::Option<std::vec::Vec<MonitoringSlo_BasicSli_Latency>>,
    dynamic: MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceDynamic,
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {
    #[doc =
        "Set the field `location`.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn set_method(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn set_version(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability`.\n"]
    pub fn set_availability(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_Availability,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.availability = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.availability = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `latency`.\n"]
    pub fn set_latency(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_BasicSli_Latency>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.latency = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.latency = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {}

impl BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance {
            location: core::default::Default::default(),
            method: core::default::Default::default(),
            version: core::default::Default::default(),
            availability: core::default::Default::default(),
            latency: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceRef {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nAn optional set of locations to which this SLI is relevant.\nTelemetry from other locations will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nlocations in which the Service has activity. For service types\nthat don't support breaking down by location, setting this\nfield will result in an error."]
    pub fn location(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nAn optional set of RPCs to which this SLI is relevant.\nTelemetry from other methods will not be used to calculate\nperformance for this SLI. If omitted, this SLI applies to all\nthe Service's methods. For service types that don't support\nbreaking down by method, setting this field will result in an\nerror."]
    pub fn method(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe set of API versions to which this SLI is relevant.\nTelemetry from other API versions will not be used to\ncalculate performance for this SLI. If omitted,\nthis SLI applies to all API versions. For service types\nthat don't support breaking down by version, setting this\nfield will result in an error."]
    pub fn version(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `availability` after provisioning.\n"]
    pub fn availability(
        &self,
    ) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance_AvailabilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability", self.base))
    }

    #[doc = "Get a reference to the value of field `latency` after provisioning.\n"]
    pub fn latency(&self) -> terrars::ListRef<MonitoringSlo_BasicSli_LatencyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.latency", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bad_service_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    good_service_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_service_filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {
    #[doc =
        "Set the field `bad_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn set_bad_service_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bad_service_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `good_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn set_good_service_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.good_service_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_service_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn set_total_service_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_service_filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {
    type O =
        terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {}

impl BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio {
            bad_service_filter: core::default::Default::default(),
            good_service_filter: core::default::Default::default(),
            total_service_filter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatioRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatioRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatioRef {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatioRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatioRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bad_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying bad service provided, either demanded service that\nwas not provided or demanded service that was of inadequate\nquality. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn bad_service_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bad_service_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `good_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying good service provided. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn good_service_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.good_service_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_service_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nquantifying total demanded service. Exactly two of\ngood, bad, or total service filter must be defined (where\ngood + bad = total is assumed)\n\nMust have ValueType = DOUBLE or ValueType = INT64 and\nmust have MetricKind = DELTA or MetricKind = CUMULATIVE."]
    pub fn total_service_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_service_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceDynamic {
    distribution_cut: std::option::Option<terrars::DynamicBlock<MonitoringSlo_RequestBasedSli_DistributionCut>>,
    good_total_ratio: std
    ::option
    ::Option<
        terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio>,
    >,
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution_cut: std::option::Option<std::vec::Vec<MonitoringSlo_RequestBasedSli_DistributionCut>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    good_total_ratio: std
    ::option
    ::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio>>,
    dynamic: MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceDynamic,
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {
    #[doc = "Set the field `distribution_cut`.\n"]
    pub fn set_distribution_cut(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_RequestBasedSli_DistributionCut>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.distribution_cut = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.distribution_cut = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `good_total_ratio`.\n"]
    pub fn set_good_total_ratio(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatio,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.good_total_ratio = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.good_total_ratio = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {}

impl BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance {
            distribution_cut: core::default::Default::default(),
            good_total_ratio: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceRef {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `distribution_cut` after provisioning.\n"]
    pub fn distribution_cut(&self) -> terrars::ListRef<MonitoringSlo_RequestBasedSli_DistributionCutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.distribution_cut", self.base))
    }

    #[doc = "Get a reference to the value of field `good_total_ratio` after provisioning.\n"]
    pub fn good_total_ratio(
        &self,
    ) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance_GoodTotalRatioRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.good_total_ratio", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdDynamic {
    basic_sli_performance: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance>>,
    performance: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_sli_performance: std
    ::option
    ::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance>>,
    dynamic: MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdDynamic,
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {
    #[doc = "Set the field `threshold`.\nIf window performance >= threshold, the window is counted\nas good."]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_sli_performance`.\n"]
    pub fn set_basic_sli_performance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformance>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_sli_performance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_sli_performance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `performance`.\n"]
    pub fn set_performance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_Performance>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.performance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.performance = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {}

impl BuildMonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold {
            threshold: core::default::Default::default(),
            basic_sli_performance: core::default::Default::default(),
            performance: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdRef {
        MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nIf window performance >= threshold, the window is counted\nas good."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_sli_performance` after provisioning.\n"]
    pub fn basic_sli_performance(
        &self,
    ) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_BasicSliPerformanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_sli_performance", self.base))
    }

    #[doc = "Get a reference to the value of field `performance` after provisioning.\n"]
    pub fn performance(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold_PerformanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {
    #[doc =
        "Set the field `max`.\nmax value for the range (inclusive). If not given,\nwill be set to \"infinity\", defining an open range\n\">= range.min\""]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min`.\nMin value for the range (inclusive). If not given,\nwill be set to \"-infinity\", defining an open range\n\"< range.max\""]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {}

impl BuildMonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {
        MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef {
        MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max` after provisioning.\nmax value for the range (inclusive). If not given,\nwill be set to \"infinity\", defining an open range\n\">= range.min\""]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min` after provisioning.\nMin value for the range (inclusive). If not given,\nwill be set to \"-infinity\", defining an open range\n\"< range.max\""]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_WindowsBasedSli_MetricMeanInRangeDynamic {
    range: std::option::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_MetricMeanInRange {
    time_series: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>>,
    dynamic: MonitoringSlo_WindowsBasedSli_MetricMeanInRangeDynamic,
}

impl MonitoringSlo_WindowsBasedSli_MetricMeanInRange {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_MetricMeanInRange {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricMeanInRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_MetricMeanInRange {
    #[doc =
        "A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nThe provided TimeSeries must have ValueType = INT64 or\nValueType = DOUBLE and MetricKind = GAUGE. Mean value 'X'\nshould satisfy 'range.min <= X <= range.max'\nunder good service."]
    pub time_series: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSlo_WindowsBasedSli_MetricMeanInRange {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_MetricMeanInRange {
        MonitoringSlo_WindowsBasedSli_MetricMeanInRange {
            time_series: self.time_series,
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_MetricMeanInRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_MetricMeanInRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_MetricMeanInRangeRef {
        MonitoringSlo_WindowsBasedSli_MetricMeanInRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_MetricMeanInRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `time_series` after provisioning.\nA [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nThe provided TimeSeries must have ValueType = INT64 or\nValueType = DOUBLE and MetricKind = GAUGE. Mean value 'X'\nshould satisfy 'range.min <= X <= range.max'\nunder good service."]
    pub fn time_series(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_series", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_WindowsBasedSli_MetricSumInRangeDynamic {
    range: std::option::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli_MetricSumInRange {
    time_series: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>>,
    dynamic: MonitoringSlo_WindowsBasedSli_MetricSumInRangeDynamic,
}

impl MonitoringSlo_WindowsBasedSli_MetricSumInRange {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricSumInRange_Range>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli_MetricSumInRange {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricSumInRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli_MetricSumInRange {
    #[doc =
        "A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nquality. The provided TimeSeries must have\nValueType = INT64 or ValueType = DOUBLE and\nMetricKind = GAUGE.\n\nSummed value 'X' should satisfy\n'range.min <= X <= range.max' for a good window."]
    pub time_series: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringSlo_WindowsBasedSli_MetricSumInRange {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli_MetricSumInRange {
        MonitoringSlo_WindowsBasedSli_MetricSumInRange {
            time_series: self.time_series,
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSli_MetricSumInRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSli_MetricSumInRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringSlo_WindowsBasedSli_MetricSumInRangeRef {
        MonitoringSlo_WindowsBasedSli_MetricSumInRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSli_MetricSumInRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `time_series` after provisioning.\nA [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nspecifying the TimeSeries to use for evaluating window\nquality. The provided TimeSeries must have\nValueType = INT64 or ValueType = DOUBLE and\nMetricKind = GAUGE.\n\nSummed value 'X' should satisfy\n'range.min <= X <= range.max' for a good window."]
    pub fn time_series(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_series", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_MetricSumInRange_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringSlo_WindowsBasedSliDynamic {
    good_total_ratio_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold>>,
    metric_mean_in_range: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_MetricMeanInRange>>,
    metric_sum_in_range: std::option::Option<terrars::DynamicBlock<MonitoringSlo_WindowsBasedSli_MetricSumInRange>>,
}

#[derive(Serialize)]
pub struct MonitoringSlo_WindowsBasedSli {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    good_bad_metric_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    good_total_ratio_threshold: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_mean_in_range: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_MetricMeanInRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_sum_in_range: std::option::Option<std::vec::Vec<MonitoringSlo_WindowsBasedSli_MetricSumInRange>>,
    dynamic: MonitoringSlo_WindowsBasedSliDynamic,
}

impl MonitoringSlo_WindowsBasedSli {
    #[doc =
        "Set the field `good_bad_metric_filter`.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nwith ValueType = BOOL. The window is good if any true values\nappear in the window. One of 'good_bad_metric_filter',\n'good_total_ratio_threshold', 'metric_mean_in_range',\n'metric_sum_in_range' must be set for 'windows_based_sli'."]
    pub fn set_good_bad_metric_filter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.good_bad_metric_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `window_period`.\nDuration over which window quality is evaluated, given as a\nduration string \"{X}s\" representing X seconds. Must be an\ninteger fraction of a day and at least 60s."]
    pub fn set_window_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `good_total_ratio_threshold`.\n"]
    pub fn set_good_total_ratio_threshold(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThreshold>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.good_total_ratio_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.good_total_ratio_threshold = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_mean_in_range`.\n"]
    pub fn set_metric_mean_in_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricMeanInRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_mean_in_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_mean_in_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_sum_in_range`.\n"]
    pub fn set_metric_sum_in_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli_MetricSumInRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_sum_in_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_sum_in_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringSlo_WindowsBasedSli {
    type O = terrars::BlockAssignable<MonitoringSlo_WindowsBasedSli>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringSlo_WindowsBasedSli {}

impl BuildMonitoringSlo_WindowsBasedSli {
    pub fn build(self) -> MonitoringSlo_WindowsBasedSli {
        MonitoringSlo_WindowsBasedSli {
            good_bad_metric_filter: core::default::Default::default(),
            window_period: core::default::Default::default(),
            good_total_ratio_threshold: core::default::Default::default(),
            metric_mean_in_range: core::default::Default::default(),
            metric_sum_in_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringSlo_WindowsBasedSliRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringSlo_WindowsBasedSliRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringSlo_WindowsBasedSliRef {
        MonitoringSlo_WindowsBasedSliRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringSlo_WindowsBasedSliRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `good_bad_metric_filter` after provisioning.\nA TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)\nwith ValueType = BOOL. The window is good if any true values\nappear in the window. One of 'good_bad_metric_filter',\n'good_total_ratio_threshold', 'metric_mean_in_range',\n'metric_sum_in_range' must be set for 'windows_based_sli'."]
    pub fn good_bad_metric_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.good_bad_metric_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window_period` after provisioning.\nDuration over which window quality is evaluated, given as a\nduration string \"{X}s\" representing X seconds. Must be an\ninteger fraction of a day and at least 60s."]
    pub fn window_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_period", self.base))
    }

    #[doc = "Get a reference to the value of field `good_total_ratio_threshold` after provisioning.\n"]
    pub fn good_total_ratio_threshold(
        &self,
    ) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_GoodTotalRatioThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.good_total_ratio_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_mean_in_range` after provisioning.\n"]
    pub fn metric_mean_in_range(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_MetricMeanInRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_mean_in_range", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_sum_in_range` after provisioning.\n"]
    pub fn metric_sum_in_range(&self) -> terrars::ListRef<MonitoringSlo_WindowsBasedSli_MetricSumInRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_sum_in_range", self.base))
    }
}
