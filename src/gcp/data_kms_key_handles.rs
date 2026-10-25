#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataKmsKeyHandlesData_ {
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
    resource_type_selector: terrars::PrimField<std::string::String>,
}

struct DataKmsKeyHandles_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKmsKeyHandlesData_>,
}

#[derive(Clone)]
pub struct DataKmsKeyHandles(std::rc::Rc<DataKmsKeyHandles_>);

impl DataKmsKeyHandles {
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

    #[doc = "Set the field `project`.\nProject ID of the project."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_handles` after provisioning.\nA list of all the retrieved key handles"]
    pub fn key_handles(&self) -> terrars::ListRef<DataKmsKeyHandles_KeyHandlesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_handles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type_selector` after provisioning.\n\n\t\t\t\t\tThe resource_type_selector argument is used to add a filter query parameter that limits which key handles are retrieved by the data source: ?filter=resource_type_selector=\"{{resource_type_selector}}\".\n\t\t\t\t\tExample values:\n\t\t\t\t\t* resource_type_selector=\"{SERVICE}.googleapis.com/{TYPE}\".\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyHandles/list)\n\t\t\t\t"]
    pub fn resource_type_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type_selector", self.extract_ref()))
    }
}

impl terrars::Referable for DataKmsKeyHandles {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKmsKeyHandles { }

impl terrars::ToListMappable for DataKmsKeyHandles {
    type O = terrars::ListRef<DataKmsKeyHandlesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKmsKeyHandles_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_kms_key_handles".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsKeyHandles {
    pub tf_id: std::string::String,
    #[doc = "The canonical id for the location. For example: \"us-east1\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "\n\t\t\t\t\tThe resource_type_selector argument is used to add a filter query parameter that limits which key handles are retrieved by the data source: ?filter=resource_type_selector=\"{{resource_type_selector}}\".\n\t\t\t\t\tExample values:\n\t\t\t\t\t* resource_type_selector=\"{SERVICE}.googleapis.com/{TYPE}\".\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyHandles/list)\n\t\t\t\t"]
    pub resource_type_selector: terrars::PrimField<std::string::String>,
}

impl BuildDataKmsKeyHandles {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKmsKeyHandles {
        let out = DataKmsKeyHandles(std::rc::Rc::new(DataKmsKeyHandles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKmsKeyHandlesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                resource_type_selector: self.resource_type_selector,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsKeyHandlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsKeyHandlesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKmsKeyHandlesRef {
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
        "Get a reference to the value of field `key_handles` after provisioning.\nA list of all the retrieved key handles"]
    pub fn key_handles(&self) -> terrars::ListRef<DataKmsKeyHandles_KeyHandlesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_handles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type_selector` after provisioning.\n\n\t\t\t\t\tThe resource_type_selector argument is used to add a filter query parameter that limits which key handles are retrieved by the data source: ?filter=resource_type_selector=\"{{resource_type_selector}}\".\n\t\t\t\t\tExample values:\n\t\t\t\t\t* resource_type_selector=\"{SERVICE}.googleapis.com/{TYPE}\".\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyHandles/list)\n\t\t\t\t"]
    pub fn resource_type_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type_selector", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKmsKeyHandles_KeyHandles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type_selector: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKmsKeyHandles_KeyHandles {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type_selector`.\n"]
    pub fn set_resource_type_selector(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_type_selector = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKmsKeyHandles_KeyHandles {
    type O = terrars::BlockAssignable<DataKmsKeyHandles_KeyHandles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKmsKeyHandles_KeyHandles {}

impl BuildDataKmsKeyHandles_KeyHandles {
    pub fn build(self) -> DataKmsKeyHandles_KeyHandles {
        DataKmsKeyHandles_KeyHandles {
            kms_key: core::default::Default::default(),
            name: core::default::Default::default(),
            resource_type_selector: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataKmsKeyHandles_KeyHandlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsKeyHandles_KeyHandlesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKmsKeyHandles_KeyHandlesRef {
        DataKmsKeyHandles_KeyHandlesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKmsKeyHandles_KeyHandlesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type_selector` after provisioning.\n"]
    pub fn resource_type_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type_selector", self.base))
    }
}
