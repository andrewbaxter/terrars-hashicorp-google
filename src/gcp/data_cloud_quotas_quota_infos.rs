#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudQuotasQuotaInfosData_ {
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
    parent: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

struct DataCloudQuotasQuotaInfos_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudQuotasQuotaInfosData_>,
}

#[derive(Clone)]
pub struct DataCloudQuotasQuotaInfos(std::rc::Rc<DataCloudQuotasQuotaInfos_>);

impl DataCloudQuotasQuotaInfos {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_infos` after provisioning.\n"]
    pub fn quota_infos(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfos_QuotaInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_infos", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudQuotasQuotaInfos {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudQuotasQuotaInfos { }

impl terrars::ToListMappable for DataCloudQuotasQuotaInfos {
    type O = terrars::ListRef<DataCloudQuotasQuotaInfosRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudQuotasQuotaInfos_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_quotas_quota_infos".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudQuotasQuotaInfos {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudQuotasQuotaInfos {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudQuotasQuotaInfos {
        let out = DataCloudQuotasQuotaInfos(std::rc::Rc::new(DataCloudQuotasQuotaInfos_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudQuotasQuotaInfosData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                parent: self.parent,
                service: self.service,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudQuotasQuotaInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfosRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudQuotasQuotaInfosRef {
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

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_infos` after provisioning.\n"]
    pub fn quota_infos(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfos_QuotaInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_infos", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details {}

impl BuildDataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details {
    pub fn build(self) -> DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details {
        DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details { value: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_DetailsRef {
        DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    applicable_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {
    #[doc = "Set the field `applicable_locations`.\n"]
    pub fn set_applicable_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.applicable_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_Details>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {}

impl BuildDataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {
    pub fn build(self) -> DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {
        DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos {
            applicable_locations: core::default::Default::default(),
            details: core::default::Default::default(),
            dimensions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfosRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfosRef {
        DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfosRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `applicable_locations` after provisioning.\n"]
    pub fn applicable_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applicable_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ineligibility_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_eligible: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {
    #[doc = "Set the field `ineligibility_reason`.\n"]
    pub fn set_ineligibility_reason(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ineligibility_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_eligible`.\n"]
    pub fn set_is_eligible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_eligible = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {}

impl BuildDataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {
    pub fn build(self) -> DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {
        DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility {
            ineligibility_reason: core::default::Default::default(),
            is_eligible: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibilityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibilityRef {
        DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ineligibility_reason` after provisioning.\n"]
    pub fn ineligibility_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ineligibility_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `is_eligible` after provisioning.\n"]
    pub fn is_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_eligible", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions_infos: std::option::Option<terrars::ListField<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_concurrent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_fixed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_precise: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_increase_eligibility: std
    ::option
    ::Option<terrars::ListField<DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_request_quota_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudQuotasQuotaInfos_QuotaInfos {
    #[doc = "Set the field `container_type`.\n"]
    pub fn set_container_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimensions_infos`.\n"]
    pub fn set_dimensions_infos(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfos>>,
    ) -> Self {
        self.dimensions_infos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_concurrent`.\n"]
    pub fn set_is_concurrent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_concurrent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_fixed`.\n"]
    pub fn set_is_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_precise`.\n"]
    pub fn set_is_precise(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_precise = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric`.\n"]
    pub fn set_metric(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_display_name`.\n"]
    pub fn set_metric_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metric_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_unit`.\n"]
    pub fn set_metric_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_display_name`.\n"]
    pub fn set_quota_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.quota_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_id`.\n"]
    pub fn set_quota_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quota_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_increase_eligibility`.\n"]
    pub fn set_quota_increase_eligibility(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibility>>,
    ) -> Self {
        self.quota_increase_eligibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_interval`.\n"]
    pub fn set_refresh_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_request_quota_uri`.\n"]
    pub fn set_service_request_quota_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_request_quota_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudQuotasQuotaInfos_QuotaInfos {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfos_QuotaInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfos_QuotaInfos {}

impl BuildDataCloudQuotasQuotaInfos_QuotaInfos {
    pub fn build(self) -> DataCloudQuotasQuotaInfos_QuotaInfos {
        DataCloudQuotasQuotaInfos_QuotaInfos {
            container_type: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            dimensions_infos: core::default::Default::default(),
            is_concurrent: core::default::Default::default(),
            is_fixed: core::default::Default::default(),
            is_precise: core::default::Default::default(),
            metric: core::default::Default::default(),
            metric_display_name: core::default::Default::default(),
            metric_unit: core::default::Default::default(),
            name: core::default::Default::default(),
            quota_display_name: core::default::Default::default(),
            quota_id: core::default::Default::default(),
            quota_increase_eligibility: core::default::Default::default(),
            refresh_interval: core::default::Default::default(),
            service: core::default::Default::default(),
            service_request_quota_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfos_QuotaInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfos_QuotaInfosRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudQuotasQuotaInfos_QuotaInfosRef {
        DataCloudQuotasQuotaInfos_QuotaInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfos_QuotaInfosRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions_infos` after provisioning.\n"]
    pub fn dimensions_infos(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfos_QuotaInfos_DimensionsInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions_infos", self.base))
    }

    #[doc = "Get a reference to the value of field `is_concurrent` after provisioning.\n"]
    pub fn is_concurrent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_concurrent", self.base))
    }

    #[doc = "Get a reference to the value of field `is_fixed` after provisioning.\n"]
    pub fn is_fixed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_fixed", self.base))
    }

    #[doc = "Get a reference to the value of field `is_precise` after provisioning.\n"]
    pub fn is_precise(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_precise", self.base))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_display_name` after provisioning.\n"]
    pub fn metric_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `quota_display_name` after provisioning.\n"]
    pub fn quota_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\n"]
    pub fn quota_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_id", self.base))
    }

    #[doc = "Get a reference to the value of field `quota_increase_eligibility` after provisioning.\n"]
    pub fn quota_increase_eligibility(
        &self,
    ) -> terrars::ListRef<DataCloudQuotasQuotaInfos_QuotaInfos_QuotaIncreaseEligibilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_increase_eligibility", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\n"]
    pub fn refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `service_request_quota_uri` after provisioning.\n"]
    pub fn service_request_quota_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_request_quota_uri", self.base))
    }
}
