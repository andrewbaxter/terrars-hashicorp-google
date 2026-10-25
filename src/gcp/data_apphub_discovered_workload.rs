#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataApphubDiscoveredWorkloadData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    workload_uri: terrars::PrimField<std::string::String>,
}

struct DataApphubDiscoveredWorkload_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataApphubDiscoveredWorkloadData_>,
}

#[derive(Clone)]
pub struct DataApphubDiscoveredWorkload(std::rc::Rc<DataApphubDiscoveredWorkload_>);

impl DataApphubDiscoveredWorkload {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_properties` after provisioning.\n"]
    pub fn workload_properties(&self) -> terrars::ListRef<DataApphubDiscoveredWorkload_WorkloadPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_reference` after provisioning.\n"]
    pub fn workload_reference(&self) -> terrars::ListRef<DataApphubDiscoveredWorkload_WorkloadReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_uri` after provisioning.\n"]
    pub fn workload_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_uri", self.extract_ref()))
    }
}

impl terrars::Referable for DataApphubDiscoveredWorkload {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApphubDiscoveredWorkload { }

impl terrars::ToListMappable for DataApphubDiscoveredWorkload {
    type O = terrars::ListRef<DataApphubDiscoveredWorkloadRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApphubDiscoveredWorkload_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_apphub_discovered_workload".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApphubDiscoveredWorkload {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workload_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataApphubDiscoveredWorkload {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApphubDiscoveredWorkload {
        let out = DataApphubDiscoveredWorkload(std::rc::Rc::new(DataApphubDiscoveredWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataApphubDiscoveredWorkloadData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                workload_uri: self.workload_uri,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApphubDiscoveredWorkloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApphubDiscoveredWorkloadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApphubDiscoveredWorkloadRef {
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

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_properties` after provisioning.\n"]
    pub fn workload_properties(&self) -> terrars::ListRef<DataApphubDiscoveredWorkload_WorkloadPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_reference` after provisioning.\n"]
    pub fn workload_reference(&self) -> terrars::ListRef<DataApphubDiscoveredWorkload_WorkloadReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_uri` after provisioning.\n"]
    pub fn workload_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_uri", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataApphubDiscoveredWorkload_WorkloadProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApphubDiscoveredWorkload_WorkloadProperties {
    #[doc = "Set the field `gcp_project`.\n"]
    pub fn set_gcp_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcp_project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApphubDiscoveredWorkload_WorkloadProperties {
    type O = terrars::BlockAssignable<DataApphubDiscoveredWorkload_WorkloadProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApphubDiscoveredWorkload_WorkloadProperties {}

impl BuildDataApphubDiscoveredWorkload_WorkloadProperties {
    pub fn build(self) -> DataApphubDiscoveredWorkload_WorkloadProperties {
        DataApphubDiscoveredWorkload_WorkloadProperties {
            gcp_project: core::default::Default::default(),
            location: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApphubDiscoveredWorkload_WorkloadPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApphubDiscoveredWorkload_WorkloadPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApphubDiscoveredWorkload_WorkloadPropertiesRef {
        DataApphubDiscoveredWorkload_WorkloadPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApphubDiscoveredWorkload_WorkloadPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_project` after provisioning.\n"]
    pub fn gcp_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_project", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApphubDiscoveredWorkload_WorkloadReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApphubDiscoveredWorkload_WorkloadReference {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApphubDiscoveredWorkload_WorkloadReference {
    type O = terrars::BlockAssignable<DataApphubDiscoveredWorkload_WorkloadReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApphubDiscoveredWorkload_WorkloadReference {}

impl BuildDataApphubDiscoveredWorkload_WorkloadReference {
    pub fn build(self) -> DataApphubDiscoveredWorkload_WorkloadReference {
        DataApphubDiscoveredWorkload_WorkloadReference { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApphubDiscoveredWorkload_WorkloadReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApphubDiscoveredWorkload_WorkloadReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApphubDiscoveredWorkload_WorkloadReferenceRef {
        DataApphubDiscoveredWorkload_WorkloadReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApphubDiscoveredWorkload_WorkloadReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
