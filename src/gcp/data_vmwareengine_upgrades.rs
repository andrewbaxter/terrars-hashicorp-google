#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareengineUpgradesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    parent: terrars::PrimField<std::string::String>,
}

struct DataVmwareengineUpgrades_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVmwareengineUpgradesData_>,
}

#[derive(Clone)]
pub struct DataVmwareengineUpgrades(std::rc::Rc<DataVmwareengineUpgrades_>);

impl DataVmwareengineUpgrades {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Set the field `name`.\nThe resource name of the specific Upgrade to retrieve. If provided, the 'upgrades' list will contain only this upgrade."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the specific Upgrade to retrieve. If provided, the 'upgrades' list will contain only this upgrade."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud for which upgrades will be listed.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upgrades` after provisioning.\nA list of VMware Engine upgrades. Contains one element if 'name' is specified in the arguments, otherwise all upgrades for the private cloud."]
    pub fn upgrades(&self) -> terrars::ListRef<DataVmwareengineUpgrades_UpgradesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrades", self.extract_ref()))
    }
}

impl terrars::Referable for DataVmwareengineUpgrades {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVmwareengineUpgrades { }

impl terrars::ToListMappable for DataVmwareengineUpgrades {
    type O = terrars::ListRef<DataVmwareengineUpgradesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVmwareengineUpgrades_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vmwareengine_upgrades".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareengineUpgrades {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of the private cloud for which upgrades will be listed.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataVmwareengineUpgrades {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVmwareengineUpgrades {
        let out = DataVmwareengineUpgrades(std::rc::Rc::new(DataVmwareengineUpgrades_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVmwareengineUpgradesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVmwareengineUpgradesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgradesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareengineUpgradesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the specific Upgrade to retrieve. If provided, the 'upgrades' list will contain only this upgrade."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the private cloud for which upgrades will be listed.\nResource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names.\nFor example: projects/my-project/locations/us-west1-a/privateClouds/my-cloud"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upgrades` after provisioning.\nA list of VMware Engine upgrades. Contains one element if 'name' is specified in the arguments, otherwise all upgrades for the private cloud."]
    pub fn upgrades(&self) -> terrars::ListRef<DataVmwareengineUpgrades_UpgradesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrades", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_ComponentUpgrades {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineUpgrades_Upgrades_ComponentUpgrades {
    #[doc = "Set the field `component_type`.\n"]
    pub fn set_component_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.component_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_ComponentUpgrades {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_ComponentUpgrades>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_ComponentUpgrades {}

impl BuildDataVmwareengineUpgrades_Upgrades_ComponentUpgrades {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_ComponentUpgrades {
        DataVmwareengineUpgrades_Upgrades_ComponentUpgrades {
            component_type: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_ComponentUpgradesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_ComponentUpgradesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_ComponentUpgradesRef {
        DataVmwareengineUpgrades_Upgrades_ComponentUpgradesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_ComponentUpgradesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_type` after provisioning.\n"]
    pub fn component_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_type", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {
    type O =
        terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTimeRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {
        DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_day: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {
    #[doc = "Set the field `end_day`.\n"]
    pub fn set_end_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTime,
                        >,
                    >,
    ) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_day`.\n"]
    pub fn set_start_day(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime>>,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals {
            end_day: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_day: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervalsRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_day` after provisioning.\n"]
    pub fn end_day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_day", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(
        &self,
    ) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals_EndTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_day` after provisioning.\n"]
    pub fn start_day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_day", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRangeRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_Constraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disallowed_intervals: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_hours_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_hours_week: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reschedule_date_range: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_Constraints {
    #[doc = "Set the field `disallowed_intervals`.\n"]
    pub fn set_disallowed_intervals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervals>,
                    >,
    ) -> Self {
        self.disallowed_intervals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_hours_day`.\n"]
    pub fn set_min_hours_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_hours_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_hours_week`.\n"]
    pub fn set_min_hours_week(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_hours_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reschedule_date_range`.\n"]
    pub fn set_reschedule_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRange>,
                    >,
    ) -> Self {
        self.reschedule_date_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_Constraints {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_Constraints {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_Constraints {
        DataVmwareengineUpgrades_Upgrades_Schedule_Constraints {
            disallowed_intervals: core::default::Default::default(),
            min_hours_day: core::default::Default::default(),
            min_hours_week: core::default::Default::default(),
            reschedule_date_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_ConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_ConstraintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_ConstraintsRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_ConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_ConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disallowed_intervals` after provisioning.\n"]
    pub fn disallowed_intervals(
        &self,
    ) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_DisallowedIntervalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disallowed_intervals", self.base))
    }

    #[doc = "Get a reference to the value of field `min_hours_day` after provisioning.\n"]
    pub fn min_hours_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_hours_day", self.base))
    }

    #[doc = "Get a reference to the value of field `min_hours_week` after provisioning.\n"]
    pub fn min_hours_week(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_hours_week", self.base))
    }

    #[doc = "Get a reference to the value of field `reschedule_date_range` after provisioning.\n"]
    pub fn reschedule_date_range(
        &self,
    ) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints_RescheduleDateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reschedule_date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {
        DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_EditWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_EditWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_EditWindowRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_EditWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_EditWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTime>>,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {
        DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows {
            day_of_week: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindowsRef {
        DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraints: std::option::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edit_window: std::option::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_editor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_windows: std::option::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows>>,
}

impl DataVmwareengineUpgrades_Upgrades_Schedule {
    #[doc = "Set the field `constraints`.\n"]
    pub fn set_constraints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_Constraints>>,
    ) -> Self {
        self.constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edit_window`.\n"]
    pub fn set_edit_window(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_EditWindow>>,
    ) -> Self {
        self.edit_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_editor`.\n"]
    pub fn set_last_editor(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_editor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_windows`.\n"]
    pub fn set_weekly_windows(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindows>>,
    ) -> Self {
        self.weekly_windows = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades_Schedule {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades_Schedule {}

impl BuildDataVmwareengineUpgrades_Upgrades_Schedule {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades_Schedule {
        DataVmwareengineUpgrades_Upgrades_Schedule {
            constraints: core::default::Default::default(),
            edit_window: core::default::Default::default(),
            last_editor: core::default::Default::default(),
            start_time: core::default::Default::default(),
            weekly_windows: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_Upgrades_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_Upgrades_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareengineUpgrades_Upgrades_ScheduleRef {
        DataVmwareengineUpgrades_Upgrades_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_Upgrades_ScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraints` after provisioning.\n"]
    pub fn constraints(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_ConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `edit_window` after provisioning.\n"]
    pub fn edit_window(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_EditWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edit_window", self.base))
    }

    #[doc = "Get a reference to the value of field `last_editor` after provisioning.\n"]
    pub fn last_editor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_editor", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_windows` after provisioning.\n"]
    pub fn weekly_windows(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_Schedule_WeeklyWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_windows", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineUpgrades_Upgrades {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_upgrades: std::option::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_ComponentUpgrades>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineUpgrades_Upgrades {
    #[doc = "Set the field `component_upgrades`.\n"]
    pub fn set_component_upgrades(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_ComponentUpgrades>>,
    ) -> Self {
        self.component_upgrades = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `estimated_duration`.\n"]
    pub fn set_estimated_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.estimated_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineUpgrades_Upgrades_Schedule>>,
    ) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_version`.\n"]
    pub fn set_start_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineUpgrades_Upgrades {
    type O = terrars::BlockAssignable<DataVmwareengineUpgrades_Upgrades>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineUpgrades_Upgrades {}

impl BuildDataVmwareengineUpgrades_Upgrades {
    pub fn build(self) -> DataVmwareengineUpgrades_Upgrades {
        DataVmwareengineUpgrades_Upgrades {
            component_upgrades: core::default::Default::default(),
            description: core::default::Default::default(),
            end_time: core::default::Default::default(),
            estimated_duration: core::default::Default::default(),
            name: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_version: core::default::Default::default(),
            state: core::default::Default::default(),
            target_version: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineUpgrades_UpgradesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineUpgrades_UpgradesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareengineUpgrades_UpgradesRef {
        DataVmwareengineUpgrades_UpgradesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineUpgrades_UpgradesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_upgrades` after provisioning.\n"]
    pub fn component_upgrades(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_ComponentUpgradesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component_upgrades", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `estimated_duration` after provisioning.\n"]
    pub fn estimated_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DataVmwareengineUpgrades_Upgrades_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `start_version` after provisioning.\n"]
    pub fn start_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_version", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
