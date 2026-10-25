#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudQuotasQuotaInfoData_ {
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
    quota_id: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

struct DataCloudQuotasQuotaInfo_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudQuotasQuotaInfoData_>,
}

#[derive(Clone)]
pub struct DataCloudQuotasQuotaInfo(std::rc::Rc<DataCloudQuotasQuotaInfo_>);

impl DataCloudQuotasQuotaInfo {
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

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dimensions_infos` after provisioning.\n"]
    pub fn dimensions_infos(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfo_DimensionsInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions_infos", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_concurrent` after provisioning.\n"]
    pub fn is_concurrent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_concurrent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_fixed` after provisioning.\n"]
    pub fn is_fixed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_fixed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_precise` after provisioning.\n"]
    pub fn is_precise(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_precise", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_display_name` after provisioning.\n"]
    pub fn metric_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_display_name` after provisioning.\n"]
    pub fn quota_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\n"]
    pub fn quota_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_increase_eligibility` after provisioning.\n"]
    pub fn quota_increase_eligibility(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_increase_eligibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\n"]
    pub fn refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_request_quota_uri` after provisioning.\n"]
    pub fn service_request_quota_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_request_quota_uri", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudQuotasQuotaInfo {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudQuotasQuotaInfo { }

impl terrars::ToListMappable for DataCloudQuotasQuotaInfo {
    type O = terrars::ListRef<DataCloudQuotasQuotaInfoRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudQuotasQuotaInfo_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_quotas_quota_info".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudQuotasQuotaInfo {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub quota_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudQuotasQuotaInfo {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudQuotasQuotaInfo {
        let out = DataCloudQuotasQuotaInfo(std::rc::Rc::new(DataCloudQuotasQuotaInfo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudQuotasQuotaInfoData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                parent: self.parent,
                quota_id: self.quota_id,
                service: self.service,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudQuotasQuotaInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudQuotasQuotaInfoRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `container_type` after provisioning.\n"]
    pub fn container_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dimensions_infos` after provisioning.\n"]
    pub fn dimensions_infos(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfo_DimensionsInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions_infos", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_concurrent` after provisioning.\n"]
    pub fn is_concurrent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_concurrent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_fixed` after provisioning.\n"]
    pub fn is_fixed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_fixed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_precise` after provisioning.\n"]
    pub fn is_precise(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_precise", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_display_name` after provisioning.\n"]
    pub fn metric_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_unit` after provisioning.\n"]
    pub fn metric_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_display_name` after provisioning.\n"]
    pub fn quota_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_id` after provisioning.\n"]
    pub fn quota_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_increase_eligibility` after provisioning.\n"]
    pub fn quota_increase_eligibility(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_increase_eligibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `refresh_interval` after provisioning.\n"]
    pub fn refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_request_quota_uri` after provisioning.\n"]
    pub fn service_request_quota_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_request_quota_uri", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfo_DimensionsInfos_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudQuotasQuotaInfo_DimensionsInfos_Details {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudQuotasQuotaInfo_DimensionsInfos_Details {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfo_DimensionsInfos_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfo_DimensionsInfos_Details {}

impl BuildDataCloudQuotasQuotaInfo_DimensionsInfos_Details {
    pub fn build(self) -> DataCloudQuotasQuotaInfo_DimensionsInfos_Details {
        DataCloudQuotasQuotaInfo_DimensionsInfos_Details { value: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfo_DimensionsInfos_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfo_DimensionsInfos_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudQuotasQuotaInfo_DimensionsInfos_DetailsRef {
        DataCloudQuotasQuotaInfo_DimensionsInfos_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfo_DimensionsInfos_DetailsRef {
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
pub struct DataCloudQuotasQuotaInfo_DimensionsInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    applicable_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<DataCloudQuotasQuotaInfo_DimensionsInfos_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudQuotasQuotaInfo_DimensionsInfos {
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
        v: impl std::convert::Into<terrars::ListField<DataCloudQuotasQuotaInfo_DimensionsInfos_Details>>,
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

impl terrars::ToListMappable for DataCloudQuotasQuotaInfo_DimensionsInfos {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfo_DimensionsInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfo_DimensionsInfos {}

impl BuildDataCloudQuotasQuotaInfo_DimensionsInfos {
    pub fn build(self) -> DataCloudQuotasQuotaInfo_DimensionsInfos {
        DataCloudQuotasQuotaInfo_DimensionsInfos {
            applicable_locations: core::default::Default::default(),
            details: core::default::Default::default(),
            dimensions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfo_DimensionsInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfo_DimensionsInfosRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudQuotasQuotaInfo_DimensionsInfosRef {
        DataCloudQuotasQuotaInfo_DimensionsInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfo_DimensionsInfosRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `applicable_locations` after provisioning.\n"]
    pub fn applicable_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applicable_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DataCloudQuotasQuotaInfo_DimensionsInfos_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ineligibility_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_eligible: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {
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

impl terrars::ToListMappable for DataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {
    type O = terrars::BlockAssignable<DataCloudQuotasQuotaInfo_QuotaIncreaseEligibility>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {}

impl BuildDataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {
    pub fn build(self) -> DataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {
        DataCloudQuotasQuotaInfo_QuotaIncreaseEligibility {
            ineligibility_reason: core::default::Default::default(),
            is_eligible: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef {
        DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudQuotasQuotaInfo_QuotaIncreaseEligibilityRef {
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
