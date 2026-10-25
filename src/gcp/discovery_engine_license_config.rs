#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineLicenseConfigData_ {
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
    auto_renew: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    free_trial: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_user_update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    license_config_id: terrars::PrimField<std::string::String>,
    license_count: terrars::PrimField<f64>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    subscription_term: terrars::PrimField<std::string::String>,
    subscription_tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<DiscoveryEngineLicenseConfig_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<DiscoveryEngineLicenseConfig_StartDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineLicenseConfig_Timeouts>,
    dynamic: DiscoveryEngineLicenseConfigDynamic,
}

struct DiscoveryEngineLicenseConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineLicenseConfigData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineLicenseConfig(std::rc::Rc<DiscoveryEngineLicenseConfig_>);

impl DiscoveryEngineLicenseConfig {
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
        "Set the field `auto_renew`.\nWhether the license config should be auto renewed when it reaches the end date."]
    pub fn set_auto_renew(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_renew = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `free_trial`.\nWhether the license config is for free trial."]
    pub fn set_free_trial(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().free_trial = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_user_update_time`.\nTimestamp of the most recent user-initiated update."]
    pub fn set_last_user_update_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().last_user_update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineLicenseConfig_EndDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineLicenseConfig_StartDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineLicenseConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `auto_renew` after provisioning.\nWhether the license config should be auto renewed when it reaches the end date."]
    pub fn auto_renew(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_renew", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `free_trial` after provisioning.\nWhether the license config is for free trial."]
    pub fn free_trial(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free_trial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_user_update_time` after provisioning.\nTimestamp of the most recent user-initiated update."]
    pub fn last_user_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_user_update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_config_id` after provisioning.\nThe unique id of the license config."]
    pub fn license_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_count` after provisioning.\nNumber of licenses purchased."]
    pub fn license_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the license config. Values are of the format\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_term` after provisioning.\nSubscription term. Possible values: [\"SUBSCRIPTION_TERM_UNSPECIFIED\", \"SUBSCRIPTION_TERM_ONE_MONTH\", \"SUBSCRIPTION_TERM_ONE_YEAR\", \"SUBSCRIPTION_TERM_THREE_YEARS\", \"SUBSCRIPTION_TERM_THREE_MONTHS\", \"SUBSCRIPTION_TERM_FOURTEEN_DAYS\", \"SUBSCRIPTION_TERM_CUSTOM\"]"]
    pub fn subscription_term(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_term", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_tier` after provisioning.\nSubscription tier information for the license config. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_NOTEBOOK_LM\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\"]"]
    pub fn subscription_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<DiscoveryEngineLicenseConfig_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<DiscoveryEngineLicenseConfig_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineLicenseConfig_TimeoutsRef {
        DiscoveryEngineLicenseConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DiscoveryEngineLicenseConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineLicenseConfig { }

impl terrars::ToListMappable for DiscoveryEngineLicenseConfig {
    type O = terrars::ListRef<DiscoveryEngineLicenseConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineLicenseConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_license_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineLicenseConfig {
    pub tf_id: std::string::String,
    #[doc = "The unique id of the license config."]
    pub license_config_id: terrars::PrimField<std::string::String>,
    #[doc = "Number of licenses purchased."]
    pub license_count: terrars::PrimField<f64>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Subscription term. Possible values: [\"SUBSCRIPTION_TERM_UNSPECIFIED\", \"SUBSCRIPTION_TERM_ONE_MONTH\", \"SUBSCRIPTION_TERM_ONE_YEAR\", \"SUBSCRIPTION_TERM_THREE_YEARS\", \"SUBSCRIPTION_TERM_THREE_MONTHS\", \"SUBSCRIPTION_TERM_FOURTEEN_DAYS\", \"SUBSCRIPTION_TERM_CUSTOM\"]"]
    pub subscription_term: terrars::PrimField<std::string::String>,
    #[doc =
        "Subscription tier information for the license config. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_NOTEBOOK_LM\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\"]"]
    pub subscription_tier: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineLicenseConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineLicenseConfig {
        let out = DiscoveryEngineLicenseConfig(std::rc::Rc::new(DiscoveryEngineLicenseConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineLicenseConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_renew: core::default::Default::default(),
                free_trial: core::default::Default::default(),
                id: core::default::Default::default(),
                last_user_update_time: core::default::Default::default(),
                license_config_id: self.license_config_id,
                license_count: self.license_count,
                location: self.location,
                project: core::default::Default::default(),
                subscription_term: self.subscription_term,
                subscription_tier: self.subscription_tier,
                end_date: core::default::Default::default(),
                start_date: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineLicenseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineLicenseConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineLicenseConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_renew` after provisioning.\nWhether the license config should be auto renewed when it reaches the end date."]
    pub fn auto_renew(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_renew", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `free_trial` after provisioning.\nWhether the license config is for free trial."]
    pub fn free_trial(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free_trial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_user_update_time` after provisioning.\nTimestamp of the most recent user-initiated update."]
    pub fn last_user_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_user_update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license_config_id` after provisioning.\nThe unique id of the license config."]
    pub fn license_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_count` after provisioning.\nNumber of licenses purchased."]
    pub fn license_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the license config. Values are of the format\n'projects/{project}/locations/{location}/licenseConfigs/{license_config}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_term` after provisioning.\nSubscription term. Possible values: [\"SUBSCRIPTION_TERM_UNSPECIFIED\", \"SUBSCRIPTION_TERM_ONE_MONTH\", \"SUBSCRIPTION_TERM_ONE_YEAR\", \"SUBSCRIPTION_TERM_THREE_YEARS\", \"SUBSCRIPTION_TERM_THREE_MONTHS\", \"SUBSCRIPTION_TERM_FOURTEEN_DAYS\", \"SUBSCRIPTION_TERM_CUSTOM\"]"]
    pub fn subscription_term(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_term", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_tier` after provisioning.\nSubscription tier information for the license config. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_NOTEBOOK_LM\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\"]"]
    pub fn subscription_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<DiscoveryEngineLicenseConfig_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<DiscoveryEngineLicenseConfig_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineLicenseConfig_TimeoutsRef {
        DiscoveryEngineLicenseConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineLicenseConfigDynamic {
    end_date: std::option::Option<terrars::DynamicBlock<DiscoveryEngineLicenseConfig_EndDate>>,
    start_date: std::option::Option<terrars::DynamicBlock<DiscoveryEngineLicenseConfig_StartDate>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineLicenseConfig_EndDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineLicenseConfig_EndDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineLicenseConfig_EndDate {
    type O = terrars::BlockAssignable<DiscoveryEngineLicenseConfig_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineLicenseConfig_EndDate {}

impl BuildDiscoveryEngineLicenseConfig_EndDate {
    pub fn build(self) -> DiscoveryEngineLicenseConfig_EndDate {
        DiscoveryEngineLicenseConfig_EndDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineLicenseConfig_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineLicenseConfig_EndDateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineLicenseConfig_EndDateRef {
        DiscoveryEngineLicenseConfig_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineLicenseConfig_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineLicenseConfig_StartDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineLicenseConfig_StartDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineLicenseConfig_StartDate {
    type O = terrars::BlockAssignable<DiscoveryEngineLicenseConfig_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineLicenseConfig_StartDate {}

impl BuildDiscoveryEngineLicenseConfig_StartDate {
    pub fn build(self) -> DiscoveryEngineLicenseConfig_StartDate {
        DiscoveryEngineLicenseConfig_StartDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineLicenseConfig_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineLicenseConfig_StartDateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineLicenseConfig_StartDateRef {
        DiscoveryEngineLicenseConfig_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineLicenseConfig_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineLicenseConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineLicenseConfig_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineLicenseConfig_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineLicenseConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineLicenseConfig_Timeouts {}

impl BuildDiscoveryEngineLicenseConfig_Timeouts {
    pub fn build(self) -> DiscoveryEngineLicenseConfig_Timeouts {
        DiscoveryEngineLicenseConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineLicenseConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineLicenseConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineLicenseConfig_TimeoutsRef {
        DiscoveryEngineLicenseConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineLicenseConfig_TimeoutsRef {
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
