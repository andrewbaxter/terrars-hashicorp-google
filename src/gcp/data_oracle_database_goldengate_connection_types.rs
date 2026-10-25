#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseGoldengateConnectionTypesData_ {
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
}

struct DataOracleDatabaseGoldengateConnectionTypes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseGoldengateConnectionTypesData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseGoldengateConnectionTypes(std::rc::Rc<DataOracleDatabaseGoldengateConnectionTypes_>);

impl DataOracleDatabaseGoldengateConnectionTypes {
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
        "Set the field `project`.\nThe ID of the project in which the resource is located. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `goldengate_connection_types` after provisioning.\n"]
    pub fn goldengate_connection_types(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.goldengate_connection_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataOracleDatabaseGoldengateConnectionTypes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseGoldengateConnectionTypes { }

impl terrars::ToListMappable for DataOracleDatabaseGoldengateConnectionTypes {
    type O = terrars::ListRef<DataOracleDatabaseGoldengateConnectionTypesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseGoldengateConnectionTypes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_goldengate_connection_types".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseGoldengateConnectionTypes {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseGoldengateConnectionTypes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseGoldengateConnectionTypes {
        let out =
            DataOracleDatabaseGoldengateConnectionTypes(
                std::rc::Rc::new(DataOracleDatabaseGoldengateConnectionTypes_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataOracleDatabaseGoldengateConnectionTypesData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        id: core::default::Default::default(),
                        location: self.location,
                        project: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseGoldengateConnectionTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateConnectionTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseGoldengateConnectionTypesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `goldengate_connection_types` after provisioning.\n"]
    pub fn goldengate_connection_types(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.goldengate_connection_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {
    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_types`.\n"]
    pub fn set_technology_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.technology_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {
    type O = terrars::BlockAssignable<DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {}

impl BuildDataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {
    pub fn build(self) -> DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {
        DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypes {
            connection_type: core::default::Default::default(),
            name: core::default::Default::default(),
            technology_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef {
        DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseGoldengateConnectionTypes_GoldengateConnectionTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `technology_types` after provisioning.\n"]
    pub fn technology_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.technology_types", self.base))
    }
}
