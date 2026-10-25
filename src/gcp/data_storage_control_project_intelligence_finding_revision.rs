#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageControlProjectIntelligenceFindingRevisionData_ {
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
    revision_id: terrars::PrimField<std::string::String>,
}

struct DataStorageControlProjectIntelligenceFindingRevision_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStorageControlProjectIntelligenceFindingRevisionData_>,
}

#[derive(Clone)]
pub struct DataStorageControlProjectIntelligenceFindingRevision(
    std::rc::Rc<DataStorageControlProjectIntelligenceFindingRevision_>,
);

impl DataStorageControlProjectIntelligenceFindingRevision {
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the finding revision was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the finding revision."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nThe ID of the finding revision."]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snapshot` after provisioning.\nThe snapshot of the finding at revision creation time."]
    pub fn snapshot(&self) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot", self.extract_ref()))
    }
}

impl terrars::Referable for DataStorageControlProjectIntelligenceFindingRevision {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStorageControlProjectIntelligenceFindingRevision { }

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision {
    type O = terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevisionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStorageControlProjectIntelligenceFindingRevision_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_storage_control_project_intelligence_finding_revision".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision {
    pub tf_id: std::string::String,
    #[doc = "The ID of the intelligence finding."]
    pub finding_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the finding revision."]
    pub revision_id: terrars::PrimField<std::string::String>,
}

impl BuildDataStorageControlProjectIntelligenceFindingRevision {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStorageControlProjectIntelligenceFindingRevision {
        let out =
            DataStorageControlProjectIntelligenceFindingRevision(
                std::rc::Rc::new(DataStorageControlProjectIntelligenceFindingRevision_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataStorageControlProjectIntelligenceFindingRevisionData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        finding_id: self.finding_id,
                        id: core::default::Default::default(),
                        location: core::default::Default::default(),
                        project: core::default::Default::default(),
                        revision_id: self.revision_id,
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageControlProjectIntelligenceFindingRevisionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRevisionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevisionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the finding revision was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the finding revision."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nThe ID of the finding revision."]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snapshot` after provisioning.\nThe snapshot of the finding at revision creation time."]
    pub fn snapshot(&self) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
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

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
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
::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes {
            percentage_increase: core::default::Default::default(),
            prefix: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef {
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
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_prefixes: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
        >,
    >,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixes,
                        >,
                    >,
    ) -> Self {
        self.top_prefixes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution {
            top_prefixes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ContributionRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ContributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ContributionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `top_prefixes` after provisioning.\n"]
    pub fn top_prefixes(
        &self,
    ) -> terrars
    ::ListRef<
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution_TopPrefixesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {
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
::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ErrorRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ErrorRef {
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
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contribution: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error,
        >,
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

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Contribution,
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_Error,
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
::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets {
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
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef {
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
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ContributionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contribution", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> terrars
    ::ListRef<
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets_ErrorRef,
    > {
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
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_operations_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets,
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
::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {
    type O =
        terrars
        ::BlockAssignable<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {
    pub fn build(
        self,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_operations_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpikeRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpikeRef {
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
    ::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_operations_count` after provisioning.\n"]
    pub fn total_operations_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_operations_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets,
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_egress_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpikeRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpikeRef {
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
    ::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_egress_bytes` after provisioning.\n"]
    pub fn total_egress_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {
    type O =
        terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriodRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriodRef {
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
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_growth_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets,
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

impl terrars
::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend {
            percentage_increase: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
            total_storage_growth_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrendRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrendRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrendRef {
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
    ::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_growth_bytes` after provisioning.\n"]
    pub fn total_storage_growth_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_growth_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage_increase: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_buckets: std
    ::option
    ::Option<
        terrars
        ::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets>,
    >,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBuckets,
                        >,
                    >,
    ) -> Self {
        self.top_buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {
    type O =
        terrars
        ::BlockAssignable<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike {
            percentage_increase: core::default::Default::default(),
            throttled_requests: core::default::Default::default(),
            top_buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpikeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpikeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpikeRef {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpikeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpikeRef {
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
    ::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike_TopBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.top_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageControlProjectIntelligenceFindingRevision_Snapshot {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coldline_and_archival_storage_operations_spike: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_egress_spike: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observation_period: std
    ::option
    ::Option<terrars::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_growth_above_trend: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throttled_requests_spike: std
    ::option
    ::Option<
        terrars::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike>,
    >,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataStorageControlProjectIntelligenceFindingRevision_Snapshot {
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
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpike,
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpike,
                        >,
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
                        terrars
                        ::ListField<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriod>,
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrend,
                        >,
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
                        ::ListField<
                            DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpike,
                        >,
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

impl terrars::ToListMappable for DataStorageControlProjectIntelligenceFindingRevision_Snapshot {
    type O = terrars::BlockAssignable<DataStorageControlProjectIntelligenceFindingRevision_Snapshot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot {}

impl BuildDataStorageControlProjectIntelligenceFindingRevision_Snapshot {
    pub fn build(self) -> DataStorageControlProjectIntelligenceFindingRevision_Snapshot {
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot {
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
pub struct DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef {
        DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageControlProjectIntelligenceFindingRevision_SnapshotRef {
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
    ::ListRef<
        DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ColdlineAndArchivalStorageOperationsSpikeRef,
    > {
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
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_CrossRegionEgressSpikeRef> {
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
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ObservationPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.observation_period", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_growth_above_trend` after provisioning.\n"]
    pub fn storage_growth_above_trend(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_StorageGrowthAboveTrendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_growth_above_trend", self.base))
    }

    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\n"]
    pub fn target_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `throttled_requests_spike` after provisioning.\n"]
    pub fn throttled_requests_spike(
        &self,
    ) -> terrars::ListRef<DataStorageControlProjectIntelligenceFindingRevision_Snapshot_ThrottledRequestsSpikeRef> {
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
