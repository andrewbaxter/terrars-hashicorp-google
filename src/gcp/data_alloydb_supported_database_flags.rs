use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAlloydbSupportedDatabaseFlagsData {
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
    data: std::cell::RefCell<DataAlloydbSupportedDatabaseFlagsData>,
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
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef> {
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
            data: std::cell::RefCell::new(DataAlloydbSupportedDatabaseFlagsData {
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
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_database_flags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
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

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
    type O =
        terrars::BlockAssignable<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {}

impl BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
        }
    }
}

pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef {
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
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    type O = terrars::BlockAssignable<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {}

impl BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl {
            allowed_values: core::default::Default::default(),
        }
    }
}

pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_values` after provisioning.\n"]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepts_multiple_values: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flag_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_restrictions: std
    ::option
    ::Option<terrars::ListField<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requires_db_restart: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_restrictions: std
    ::option
    ::Option<terrars::ListField<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_db_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
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
                        ::ListField<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsEl>,
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
                        ::ListField<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsEl>,
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

impl terrars::ToListMappable for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    type O = terrars::BlockAssignable<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {}

impl BuildDataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
    pub fn build(self) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsEl {
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

pub struct DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
        DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElRef {
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
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElIntegerRestrictionsElRef> {
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
    ) -> terrars::ListRef<DataAlloydbSupportedDatabaseFlagsSupportedDatabaseFlagsElStringRestrictionsElRef> {
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
