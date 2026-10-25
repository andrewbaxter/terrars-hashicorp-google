#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageControlProjectIntelligenceFindingData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    finding_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataStorageControlProjectIntelligenceFinding_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageControlProjectIntelligenceFindingData_>,
}

#[derive(Clone)]
pub struct DataStorageControlProjectIntelligenceFinding(std::rc::Rc<DataStorageControlProjectIntelligenceFinding_>);

impl DataStorageControlProjectIntelligenceFinding {
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
        "Set the field `location`.\nThe location of the intelligence finding. Currently default value is global and users cannot use for input for now."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `associated_resources` after provisioning.\nGoogle Cloud resource names that are relevant to the IntelligenceFinding. This list also includes the targetResource."]
    pub fn associated_resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the finding."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `coldline_and_archival_storage_operations_spike` after provisioning.\nA finding about a spike in Class A or Class B operations on Coldline or Archive Cloud Storage objects."]
    pub fn coldline_and_archival_storage_operations_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.coldline_and_archival_storage_operations_spike", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the finding was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cross_region_egress_spike` after provisioning.\nA finding about a spike in cross-region egress from Cloud Storage."]
    pub fn cross_region_egress_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_egress_spike", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA short description of the finding."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_id` after provisioning.\nThe ID of the intelligence finding."]
    pub fn finding_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the intelligence finding. Currently default value is global and users cannot use for input for now."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the finding."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observation_period` after provisioning.\nThe time interval from which the underlying data generated this IntelligenceFinding was observed."]
    pub fn observation_period(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.observation_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe severity of the finding."]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_growth_above_trend` after provisioning.\nA finding about a spike in storage growth (bytes or object count) that is outside the normal historical trend."]
    pub fn storage_growth_above_trend(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_growth_above_trend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_resource` after provisioning.\nThe fully qualified resource name of the resource that this IntelligenceFinding applies to."]
    pub fn target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `throttled_requests_spike` after provisioning.\nA finding about a spike in throttled requests (429 errors) within a project."]
    pub fn throttled_requests_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttled_requests_spike", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of this finding."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the finding was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataStorageControlProjectIntelligenceFinding {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStorageControlProjectIntelligenceFinding { }

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding {
    type O = terrars::ListRef<DataStorageControlProjectIntelligenceFindingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStorageControlProjectIntelligenceFinding_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_storage_control_project_intelligence_finding".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding {
    pub tf_id: std::string::String,
    #[doc = "The ID of the intelligence finding."]
    pub finding_id: terrars::PrimField<std::string::String>,
}

impl BuildDataStorageControlProjectIntelligenceFinding {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStorageControlProjectIntelligenceFinding {
        let out =
            DataStorageControlProjectIntelligenceFinding(
                std::rc::Rc::new(DataStorageControlProjectIntelligenceFinding_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataStorageControlProjectIntelligenceFindingData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        finding_id: self.finding_id,
                        id: core::default::Default::default(),
                        location: core::default::Default::default(),
                        project: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageControlProjectIntelligenceFindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `associated_resources` after provisioning.\nGoogle Cloud resource names that are relevant to the IntelligenceFinding. This list also includes the targetResource."]
    pub fn associated_resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\nThe category of the finding."]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `coldline_and_archival_storage_operations_spike` after provisioning.\nA finding about a spike in Class A or Class B operations on Coldline or Archive Cloud Storage objects."]
    pub fn coldline_and_archival_storage_operations_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.coldline_and_archival_storage_operations_spike", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the finding was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cross_region_egress_spike` after provisioning.\nA finding about a spike in cross-region egress from Cloud Storage."]
    pub fn cross_region_egress_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_egress_spike", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA short description of the finding."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `finding_id` after provisioning.\nThe ID of the intelligence finding."]
    pub fn finding_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finding_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the intelligence finding. Currently default value is global and users cannot use for input for now."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the finding."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `observation_period` after provisioning.\nThe time interval from which the underlying data generated this IntelligenceFinding was observed."]
    pub fn observation_period(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.observation_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\nThe severity of the finding."]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_growth_above_trend` after provisioning.\nA finding about a spike in storage growth (bytes or object count) that is outside the normal historical trend."]
    pub fn storage_growth_above_trend(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_growth_above_trend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_resource` after provisioning.\nThe fully qualified resource name of the resource that this IntelligenceFinding applies to."]
    pub fn target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `throttled_requests_spike` after provisioning.\nA finding about a spike in throttled requests (429 errors) within a project."]
    pub fn throttled_requests_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttled_requests_spike", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of this finding."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the finding was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_operations_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    #[doc = "Set the field `percentage_increase`.\n"]
    pub fn set_percentage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttled_requests`.\n"]
    pub fn set_throttled_requests(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.throttled_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_egress_bytes`.\n"]
    pub fn set_total_egress_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_egress_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_operations_count`.\n"]
    pub fn set_total_operations_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_operations_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {}

impl BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
            percentage_increase: core::default::Default::default(),
            prefix: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percentage_increase` after provisioning.\n"]
    pub fn percentage_increase(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage_increase", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `throttled_requests` after provisioning.\n"]
    pub fn throttled_requests(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttled_requests", self.base))
    }

    #[doc = "Get a reference to the value of field `total_egress_bytes` after provisioning.\n"]
    pub fn total_egress_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_operations_count` after provisioning.\n"]
    pub fn total_operations_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_operations_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_prefixes: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
        >,
    >,
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {
    #[doc = "Set the field `top_prefixes`.\n"]
    pub fn set_top_prefixes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
                        >,
                    >,
    ) -> Self {
        self.top_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {}

impl BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution {
            top_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ContributionRef {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ContributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `top_prefixes` after provisioning.\n"]
    pub fn top_prefixes(
        &self,
    ) -> terrars
    ::ListRef<
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {}

impl BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ErrorRef {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contribution: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_operations_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_growth_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contribution`.\n"]
    pub fn set_contribution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Contribution,
                        >,
                    >,
    ) -> Self {
        self.contribution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_Error,
                        >,
                    >,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percentage_increase`.\n"]
    pub fn set_percentage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttled_requests`.\n"]
    pub fn set_throttled_requests(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.throttled_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_egress_bytes`.\n"]
    pub fn set_total_egress_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_egress_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_operations_count`.\n"]
    pub fn set_total_operations_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_operations_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_storage_growth_bytes`.\n"]
    pub fn set_total_storage_growth_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_storage_growth_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {
    type O =
        terrars::BlockAssignable<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {}

impl BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets {
            bucket: core::default::Default::default(),
            contribution: core::default::Default::default(),
            error: core::default::Default::default(),
            percentage_increase: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
            total_storage_growth_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `contribution` after provisioning.\n"]
    pub fn contribution(
        &self,
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ContributionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contribution", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `percentage_increase` after provisioning.\n"]
    pub fn percentage_increase(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage_increase", self.base))
    }

    #[doc = "Get a reference to the value of field `throttled_requests` after provisioning.\n"]
    pub fn throttled_requests(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttled_requests", self.base))
    }

    #[doc = "Get a reference to the value of field `total_egress_bytes` after provisioning.\n"]
    pub fn total_egress_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_operations_count` after provisioning.\n"]
    pub fn total_operations_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_operations_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_growth_bytes` after provisioning.\n"]
    pub fn total_storage_growth_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_growth_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_operations_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {
    #[doc = "Set the field `percentage_increase`.\n"]
    pub fn set_percentage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_buckets`.\n"]
    pub fn set_top_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>,
                    >,
    ) -> Self {
        self.top_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_operations_count`.\n"]
    pub fn set_total_operations_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_operations_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {}

impl BuildDataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {
        DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpike {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef {
        DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_ColdlineAndArchivalStorageOperationsSpikeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percentage_increase` after provisioning.\n"]
    pub fn percentage_increase(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage_increase", self.base))
    }

    #[doc = "Get a reference to the value of field `top_buckets` after provisioning.\n"]
    pub fn top_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_operations_count` after provisioning.\n"]
    pub fn total_operations_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_operations_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {
    #[doc = "Set the field `percentage_increase`.\n"]
    pub fn set_percentage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_buckets`.\n"]
    pub fn set_top_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>,
                    >,
    ) -> Self {
        self.top_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_egress_bytes`.\n"]
    pub fn set_total_egress_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_egress_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {}

impl BuildDataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef {
        DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpikeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percentage_increase` after provisioning.\n"]
    pub fn percentage_increase(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage_increase", self.base))
    }

    #[doc = "Get a reference to the value of field `top_buckets` after provisioning.\n"]
    pub fn top_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_egress_bytes` after provisioning.\n"]
    pub fn total_egress_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_ObservationPeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_ObservationPeriod {
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding_ObservationPeriod {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFinding_ObservationPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_ObservationPeriod {}

impl BuildDataStorageControlProjectIntelligenceFinding_ObservationPeriod {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_ObservationPeriod {
        DataStorageControlProjectIntelligenceFinding_ObservationPeriod {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef {
        DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_ObservationPeriodRef {
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
pub struct DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_growth_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {
    #[doc = "Set the field `percentage_increase`.\n"]
    pub fn set_percentage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_buckets`.\n"]
    pub fn set_top_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>,
                    >,
    ) -> Self {
        self.top_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_storage_growth_bytes`.\n"]
    pub fn set_total_storage_growth_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_storage_growth_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {}

impl BuildDataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {
        DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrend {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_storage_growth_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef {
        DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_StorageGrowthAboveTrendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percentage_increase` after provisioning.\n"]
    pub fn percentage_increase(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage_increase", self.base))
    }

    #[doc = "Get a reference to the value of field `top_buckets` after provisioning.\n"]
    pub fn top_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_growth_bytes` after provisioning.\n"]
    pub fn total_storage_growth_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_growth_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>>,
}

impl DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {
    #[doc = "Set the field `percentage_increase`.\n"]
    pub fn set_percentage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttled_requests`.\n"]
    pub fn set_throttled_requests(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.throttled_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_buckets`.\n"]
    pub fn set_top_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBuckets>,
                    >,
    ) -> Self {
        self.top_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {}

impl BuildDataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {
        DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpike {
            percentage_increase: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef {
        DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFinding_ThrottledRequestsSpikeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percentage_increase` after provisioning.\n"]
    pub fn percentage_increase(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage_increase", self.base))
    }

    #[doc = "Get a reference to the value of field `throttled_requests` after provisioning.\n"]
    pub fn throttled_requests(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throttled_requests", self.base))
    }

    #[doc = "Get a reference to the value of field `top_buckets` after provisioning.\n"]
    pub fn top_buckets(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFinding_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }
}
