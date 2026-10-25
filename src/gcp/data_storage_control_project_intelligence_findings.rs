#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageControlProjectIntelligenceFindingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataStorageControlProjectIntelligenceFindings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageControlProjectIntelligenceFindingsData_>,
}

#[derive(Clone)]
pub struct DataStorageControlProjectIntelligenceFindings(
    std::rc::Rc<DataStorageControlProjectIntelligenceFindings_>,
);

impl DataStorageControlProjectIntelligenceFindings {
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

    #[doc =
        "Set the field `filter`.\nThe filter expression to apply. Supports filtering by type and associated_resources."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe location of the intelligence findings. Currently default value is global and users cannot use for input for now."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `page_size`.\nThe maximum number of IntelligenceFinding resources to return. The maximum value is 100; values greater than 100 become 100. The default value is 100."]
    pub fn set_page_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().page_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filter expression to apply. Supports filtering by type and associated_resources."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `findings` after provisioning.\nThe list of intelligence findings."]
    pub fn findings(&self) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindings_FindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.findings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the intelligence findings. Currently default value is global and users cannot use for input for now."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_size` after provisioning.\nThe maximum number of IntelligenceFinding resources to return. The maximum value is 100; values greater than 100 become 100. The default value is 100."]
    pub fn page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataStorageControlProjectIntelligenceFindings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStorageControlProjectIntelligenceFindings { }

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindings {
    type O = terrars::ListRef<DataStorageControlProjectIntelligenceFindingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStorageControlProjectIntelligenceFindings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_storage_control_project_intelligence_findings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings {
    pub tf_id: std::string::String,
}

impl BuildDataStorageControlProjectIntelligenceFindings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStorageControlProjectIntelligenceFindings {
        let out =
            DataStorageControlProjectIntelligenceFindings(
                std::rc::Rc::new(DataStorageControlProjectIntelligenceFindings_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataStorageControlProjectIntelligenceFindingsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        filter: core::default::Default::default(),
                        id: core::default::Default::default(),
                        location: core::default::Default::default(),
                        page_size: core::default::Default::default(),
                        project: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageControlProjectIntelligenceFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filter expression to apply. Supports filtering by type and associated_resources."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `findings` after provisioning.\nThe list of intelligence findings."]
    pub fn findings(&self) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindings_FindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.findings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the intelligence findings. Currently default value is global and users cannot use for input for now."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `page_size` after provisioning.\nThe maximum number of IntelligenceFinding resources to return. The maximum value is 100; values greater than 100 become 100. The default value is 100."]
    pub fn page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
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

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
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
::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
            percentage_increase: core::default::Default::default(),
            prefix: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
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
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_prefixes: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
        >,
    >,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {
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
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
                        >,
                    >,
    ) -> Self {
        self.top_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution {
            top_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ContributionRef {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ContributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `top_prefixes` after provisioning.\n"]
    pub fn top_prefixes(
        &self,
    ) -> terrars
    ::ListRef<
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {
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

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ErrorRef {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ErrorRef {
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
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contribution: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error>,
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

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {
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
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Contribution,
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
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_Error,
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

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets {
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
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef {
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
    ::ListRef<
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ContributionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contribution", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets_ErrorRef> {
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
pub struct DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_operations_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets,
                        >,
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
::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {
        DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpikeRef {
        DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpikeRef {
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
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_operations_count` after provisioning.\n"]
    pub fn total_operations_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_operations_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets,
                        >,
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpikeRef {
        DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpikeRef {
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
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_egress_bytes` after provisioning.\n"]
    pub fn total_egress_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {
        DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriodRef {
        DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriodRef {
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
pub struct DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_growth_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets,
                        >,
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {
    type O =
        terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {
        DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_storage_growth_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrendRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrendRef {
        DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrendRef {
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
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_growth_bytes` after provisioning.\n"]
    pub fn total_storage_growth_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_growth_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets>,
    >,
}

impl DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBuckets,
                        >,
                    >,
    ) -> Self {
        self.top_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {
        DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike {
            percentage_increase: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpikeRef {
        DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpikeRef {
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
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindings_Findings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coldline_and_archival_storage_operations_spike: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_egress_spike: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observation_period: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_growth_above_trend: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests_spike: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindings_Findings {
    #[doc = "Set the field `associated_resources`.\n"]
    pub fn set_associated_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.associated_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coldline_and_archival_storage_operations_spike`.\n"]
    pub fn set_coldline_and_archival_storage_operations_spike(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpike,
                        >,
                    >,
    ) -> Self {
        self.coldline_and_archival_storage_operations_spike = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cross_region_egress_spike`.\n"]
    pub fn set_cross_region_egress_spike(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpike>,
                    >,
    ) -> Self {
        self.cross_region_egress_spike = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `observation_period`.\n"]
    pub fn set_observation_period(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriod>,
                    >,
    ) -> Self {
        self.observation_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_growth_above_trend`.\n"]
    pub fn set_storage_growth_above_trend(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrend>,
                    >,
    ) -> Self {
        self.storage_growth_above_trend = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_resource`.\n"]
    pub fn set_target_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throttled_requests_spike`.\n"]
    pub fn set_throttled_requests_spike(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpike>,
                    >,
    ) -> Self {
        self.throttled_requests_spike = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindings_Findings {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindings_Findings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindings_Findings {}

impl BuildDataStorageControlProjectIntelligenceFindings_Findings {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindings_Findings {
        DataStorageControlProjectIntelligenceFindings_Findings {
            associated_resources: core::default::Default::default(),
            category: core::default::Default::default(),
            coldline_and_archival_storage_operations_spike: core::default::Default::default(),
            create_time: core::default::Default::default(),
            cross_region_egress_spike: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            observation_period: core::default::Default::default(),
            severity: core::default::Default::default(),
            storage_growth_above_trend: core::default::Default::default(),
            target_resource: core::default::Default::default(),
            throttled_requests_spike: core::default::Default::default(),
            type_: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindings_FindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindings_FindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindings_FindingsRef {
        DataStorageControlProjectIntelligenceFindings_FindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindings_FindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `associated_resources` after provisioning.\n"]
    pub fn associated_resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `coldline_and_archival_storage_operations_spike` after provisioning.\n"]
    pub fn coldline_and_archival_storage_operations_spike(
        &self,
    ) -> terrars
    ::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_ColdlineAndArchivalStorageOperationsSpikeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.coldline_and_archival_storage_operations_spike", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_region_egress_spike` after provisioning.\n"]
    pub fn cross_region_egress_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_CrossRegionEgressSpikeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_region_egress_spike", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `observation_period` after provisioning.\n"]
    pub fn observation_period(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_ObservationPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.observation_period", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_growth_above_trend` after provisioning.\n"]
    pub fn storage_growth_above_trend(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_StorageGrowthAboveTrendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_growth_above_trend", self.base))
    }

    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\n"]
    pub fn target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `throttled_requests_spike` after provisioning.\n"]
    pub fn throttled_requests_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindings_Findings_ThrottledRequestsSpikeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.throttled_requests_spike", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
