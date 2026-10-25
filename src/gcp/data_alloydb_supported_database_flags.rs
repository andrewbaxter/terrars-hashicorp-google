#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAlloydbSupportedDatabaseFlagsData_ {
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

struct DataAlloydbSupportedDatabaseFlags_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAlloydbSupportedDatabaseFlagsData_>,
}

#[derive(Clone)]
pub struct DataAlloydbSupportedDatabaseFlags(std::rc::Rc<DataAlloydbSupportedDatabaseFlags_>);

impl DataAlloydbSupportedDatabaseFlags {
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
        "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_database_flags` after provisioning.\n"]
    pub fn supported_database_flags(
        &self,
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_database_flags", self.extract_ref()))
    }
}

impl terrars::Referable for DataAlloydbSupportedDatabaseFlags {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAlloydbSupportedDatabaseFlags { }

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlags {
    type O = terrars::ListRef<DataAlloydbSupportedDatabaseFlagsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAlloydbSupportedDatabaseFlags_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_alloydb_supported_database_flags".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlags {
    pub tf_id: std::string::String,
    #[doc = "The canonical id for the location. For example: \"us-east1\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataAlloydbSupportedDatabaseFlags {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAlloydbSupportedDatabaseFlags {
        let out = DataAlloydbSupportedDatabaseFlags(std::rc::Rc::new(DataAlloydbSupportedDatabaseFlags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAlloydbSupportedDatabaseFlagsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAlloydbSupportedDatabaseFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAlloydbSupportedDatabaseFlagsRef {
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
        "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_database_flags` after provisioning.\n"]
    pub fn supported_database_flags(
        &self,
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_database_flags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {
    #[doc = "Set the field `max_value`.\n"]
    pub fn set_max_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_value`.\n"]
    pub fn set_min_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {
    type O = terrars::BlockAssignable<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {}

impl BuildDataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {
        DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictionsRef {
        DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_value` after provisioning.\n"]
    pub fn max_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_value", self.base))
    }

    #[doc = "Get a reference to the value of field `min_value` after provisioning.\n"]
    pub fn min_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {
    type O = terrars::BlockAssignable<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {}

impl BuildDataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {
        DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions {
            allowed_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictionsRef {
        DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepts_multiple_values: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flag_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_restrictions: std
    ::option
    ::Option<terrars::ListField<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requires_db_restart: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_restrictions: std
    ::option
    ::Option<terrars::ListField<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_db_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {
    #[doc = "Set the field `accepts_multiple_values`.\n"]
    pub fn set_accepts_multiple_values(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.accepts_multiple_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flag_name`.\n"]
    pub fn set_flag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flag_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integer_restrictions`.\n"]
    pub fn set_integer_restrictions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictions>,
                    >,
    ) -> Self {
        self.integer_restrictions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requires_db_restart`.\n"]
    pub fn set_requires_db_restart(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.requires_db_restart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_restrictions`.\n"]
    pub fn set_string_restrictions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictions>,
                    >,
    ) -> Self {
        self.string_restrictions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_db_versions`.\n"]
    pub fn set_supported_db_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_db_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_type`.\n"]
    pub fn set_value_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {
    type O = terrars::BlockAssignable<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {}

impl BuildDataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {
        DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags {
            accepts_multiple_values: core::default::Default::default(),
            flag_name: core::default::Default::default(),
            integer_restrictions: core::default::Default::default(),
            name: core::default::Default::default(),
            requires_db_restart: core::default::Default::default(),
            string_restrictions: core::default::Default::default(),
            supported_db_versions: core::default::Default::default(),
            value_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef {
        DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accepts_multiple_values` after provisioning.\n"]
    pub fn accepts_multiple_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accepts_multiple_values", self.base))
    }

    #[doc = "Get a reference to the value of field `flag_name` after provisioning.\n"]
    pub fn flag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flag_name", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_restrictions` after provisioning.\n"]
    pub fn integer_restrictions(
        &self,
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_IntegerRestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `requires_db_restart` after provisioning.\n"]
    pub fn requires_db_restart(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requires_db_restart", self.base))
    }

    #[doc = "Get a reference to the value of field `string_restrictions` after provisioning.\n"]
    pub fn string_restrictions(
        &self,
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlags_SupportedDatabaseFlags_StringRestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_db_versions` after provisioning.\n"]
    pub fn supported_db_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_db_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}
