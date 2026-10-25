#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDataplexDataQualityRulesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    data_scan_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDataplexDataQualityRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDataplexDataQualityRulesData_>,
}

#[derive(Clone)]
pub struct DataDataplexDataQualityRules(std::rc::Rc<DataDataplexDataQualityRules_>);

impl DataDataplexDataQualityRules {
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

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `data_scan_id` after provisioning.\n"]
    pub fn data_scan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_scan_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataDataplexDataQualityRules_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

impl terrars::Referable for DataDataplexDataQualityRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDataplexDataQualityRules { }

impl terrars::ToListMappable for DataDataplexDataQualityRules {
    type O = terrars::ListRef<DataDataplexDataQualityRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDataplexDataQualityRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_dataplex_data_quality_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDataplexDataQualityRules {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_scan_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDataplexDataQualityRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDataplexDataQualityRules {
        let out = DataDataplexDataQualityRules(std::rc::Rc::new(DataDataplexDataQualityRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDataplexDataQualityRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                data_scan_id: self.data_scan_id,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDataplexDataQualityRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDataplexDataQualityRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `data_scan_id` after provisioning.\n"]
    pub fn data_scan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_scan_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataDataplexDataQualityRules_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_NonNullExpectation {}

impl DataDataplexDataQualityRules_Rules_NonNullExpectation { }

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_NonNullExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_NonNullExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_NonNullExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_NonNullExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_NonNullExpectation {
        DataDataplexDataQualityRules_Rules_NonNullExpectation {}
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_NonNullExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_NonNullExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_NonNullExpectationRef {
        DataDataplexDataQualityRules_Rules_NonNullExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_NonNullExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_RangeExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_max_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_min_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDataplexDataQualityRules_Rules_RangeExpectation {
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

    #[doc = "Set the field `strict_max_enabled`.\n"]
    pub fn set_strict_max_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_max_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strict_min_enabled`.\n"]
    pub fn set_strict_min_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_min_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_RangeExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_RangeExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_RangeExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_RangeExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_RangeExpectation {
        DataDataplexDataQualityRules_Rules_RangeExpectation {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_RangeExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_RangeExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_RangeExpectationRef {
        DataDataplexDataQualityRules_Rules_RangeExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_RangeExpectationRef {
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

    #[doc = "Get a reference to the value of field `strict_max_enabled` after provisioning.\n"]
    pub fn strict_max_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_max_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_min_enabled` after provisioning.\n"]
    pub fn strict_min_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_min_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_RegexExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataplexDataQualityRules_Rules_RegexExpectation {
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_RegexExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_RegexExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_RegexExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_RegexExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_RegexExpectation {
        DataDataplexDataQualityRules_Rules_RegexExpectation { regex: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_RegexExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_RegexExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_RegexExpectationRef {
        DataDataplexDataQualityRules_Rules_RegexExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_RegexExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_RowConditionExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataplexDataQualityRules_Rules_RowConditionExpectation {
    #[doc = "Set the field `sql_expression`.\n"]
    pub fn set_sql_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_RowConditionExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_RowConditionExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_RowConditionExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_RowConditionExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_RowConditionExpectation {
        DataDataplexDataQualityRules_Rules_RowConditionExpectation {
            sql_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_RowConditionExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_RowConditionExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_RowConditionExpectationRef {
        DataDataplexDataQualityRules_Rules_RowConditionExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_RowConditionExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\n"]
    pub fn sql_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_SetExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataDataplexDataQualityRules_Rules_SetExpectation {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_SetExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_SetExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_SetExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_SetExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_SetExpectation {
        DataDataplexDataQualityRules_Rules_SetExpectation { values: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_SetExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_SetExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_SetExpectationRef {
        DataDataplexDataQualityRules_Rules_SetExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_SetExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_SqlAssertion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_statement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataplexDataQualityRules_Rules_SqlAssertion {
    #[doc = "Set the field `sql_statement`.\n"]
    pub fn set_sql_statement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_statement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_SqlAssertion {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_SqlAssertion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_SqlAssertion {}

impl BuildDataDataplexDataQualityRules_Rules_SqlAssertion {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_SqlAssertion {
        DataDataplexDataQualityRules_Rules_SqlAssertion { sql_statement: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_SqlAssertionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_SqlAssertionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_SqlAssertionRef {
        DataDataplexDataQualityRules_Rules_SqlAssertionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_SqlAssertionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sql_statement` after provisioning.\n"]
    pub fn sql_statement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_statement", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_StatisticRangeExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_max_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strict_min_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDataplexDataQualityRules_Rules_StatisticRangeExpectation {
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

    #[doc = "Set the field `statistic`.\n"]
    pub fn set_statistic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.statistic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strict_max_enabled`.\n"]
    pub fn set_strict_max_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_max_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strict_min_enabled`.\n"]
    pub fn set_strict_min_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strict_min_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_StatisticRangeExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_StatisticRangeExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_StatisticRangeExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_StatisticRangeExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_StatisticRangeExpectation {
        DataDataplexDataQualityRules_Rules_StatisticRangeExpectation {
            max_value: core::default::Default::default(),
            min_value: core::default::Default::default(),
            statistic: core::default::Default::default(),
            strict_max_enabled: core::default::Default::default(),
            strict_min_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_StatisticRangeExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_StatisticRangeExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_StatisticRangeExpectationRef {
        DataDataplexDataQualityRules_Rules_StatisticRangeExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_StatisticRangeExpectationRef {
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

    #[doc = "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_max_enabled` after provisioning.\n"]
    pub fn strict_max_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_max_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_min_enabled` after provisioning.\n"]
    pub fn strict_min_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strict_min_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_TableConditionExpectation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataplexDataQualityRules_Rules_TableConditionExpectation {
    #[doc = "Set the field `sql_expression`.\n"]
    pub fn set_sql_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_TableConditionExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_TableConditionExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_TableConditionExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_TableConditionExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_TableConditionExpectation {
        DataDataplexDataQualityRules_Rules_TableConditionExpectation {
            sql_expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_TableConditionExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_TableConditionExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_TableConditionExpectationRef {
        DataDataplexDataQualityRules_Rules_TableConditionExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_TableConditionExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sql_expression` after provisioning.\n"]
    pub fn sql_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules_UniquenessExpectation {}

impl DataDataplexDataQualityRules_Rules_UniquenessExpectation { }

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules_UniquenessExpectation {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules_UniquenessExpectation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules_UniquenessExpectation {}

impl BuildDataDataplexDataQualityRules_Rules_UniquenessExpectation {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules_UniquenessExpectation {
        DataDataplexDataQualityRules_Rules_UniquenessExpectation {}
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_Rules_UniquenessExpectationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_Rules_UniquenessExpectationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataplexDataQualityRules_Rules_UniquenessExpectationRef {
        DataDataplexDataQualityRules_Rules_UniquenessExpectationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_Rules_UniquenessExpectationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataDataplexDataQualityRules_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_null: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_null_expectation: std::option::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_NonNullExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_expectation: std::option::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_RangeExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_expectation: std::option::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_RegexExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_condition_expectation: std
    ::option
    ::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_RowConditionExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    set_expectation: std::option::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_SetExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_assertion: std::option::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_SqlAssertion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statistic_range_expectation: std
    ::option
    ::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_StatisticRangeExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_condition_expectation: std
    ::option
    ::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_TableConditionExpectation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uniqueness_expectation: std
    ::option
    ::Option<terrars::ListField<DataDataplexDataQualityRules_Rules_UniquenessExpectation>>,
}

impl DataDataplexDataQualityRules_Rules {
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_null`.\n"]
    pub fn set_ignore_null(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_null = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_null_expectation`.\n"]
    pub fn set_non_null_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_NonNullExpectation>>,
    ) -> Self {
        self.non_null_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_expectation`.\n"]
    pub fn set_range_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_RangeExpectation>>,
    ) -> Self {
        self.range_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex_expectation`.\n"]
    pub fn set_regex_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_RegexExpectation>>,
    ) -> Self {
        self.regex_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_condition_expectation`.\n"]
    pub fn set_row_condition_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_RowConditionExpectation>>,
    ) -> Self {
        self.row_condition_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `set_expectation`.\n"]
    pub fn set_set_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_SetExpectation>>,
    ) -> Self {
        self.set_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql_assertion`.\n"]
    pub fn set_sql_assertion(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_SqlAssertion>>,
    ) -> Self {
        self.sql_assertion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statistic_range_expectation`.\n"]
    pub fn set_statistic_range_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_StatisticRangeExpectation>>,
    ) -> Self {
        self.statistic_range_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspended`.\n"]
    pub fn set_suspended(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_condition_expectation`.\n"]
    pub fn set_table_condition_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_TableConditionExpectation>>,
    ) -> Self {
        self.table_condition_expectation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uniqueness_expectation`.\n"]
    pub fn set_uniqueness_expectation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataplexDataQualityRules_Rules_UniquenessExpectation>>,
    ) -> Self {
        self.uniqueness_expectation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataplexDataQualityRules_Rules {
    type O = terrars::BlockAssignable<DataDataplexDataQualityRules_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataplexDataQualityRules_Rules {}

impl BuildDataDataplexDataQualityRules_Rules {
    pub fn build(self) -> DataDataplexDataQualityRules_Rules {
        DataDataplexDataQualityRules_Rules {
            column: core::default::Default::default(),
            description: core::default::Default::default(),
            dimension: core::default::Default::default(),
            ignore_null: core::default::Default::default(),
            name: core::default::Default::default(),
            non_null_expectation: core::default::Default::default(),
            range_expectation: core::default::Default::default(),
            regex_expectation: core::default::Default::default(),
            row_condition_expectation: core::default::Default::default(),
            set_expectation: core::default::Default::default(),
            sql_assertion: core::default::Default::default(),
            statistic_range_expectation: core::default::Default::default(),
            suspended: core::default::Default::default(),
            table_condition_expectation: core::default::Default::default(),
            threshold: core::default::Default::default(),
            uniqueness_expectation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataplexDataQualityRules_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataplexDataQualityRules_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDataplexDataQualityRules_RulesRef {
        DataDataplexDataQualityRules_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataplexDataQualityRules_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_null` after provisioning.\n"]
    pub fn ignore_null(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_null", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `non_null_expectation` after provisioning.\n"]
    pub fn non_null_expectation(&self) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_NonNullExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.non_null_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `range_expectation` after provisioning.\n"]
    pub fn range_expectation(&self) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_RangeExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_expectation` after provisioning.\n"]
    pub fn regex_expectation(&self) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_RegexExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `row_condition_expectation` after provisioning.\n"]
    pub fn row_condition_expectation(
        &self,
    ) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_RowConditionExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_condition_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `set_expectation` after provisioning.\n"]
    pub fn set_expectation(&self) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_SetExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.set_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_assertion` after provisioning.\n"]
    pub fn sql_assertion(&self) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_SqlAssertionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_assertion", self.base))
    }

    #[doc = "Get a reference to the value of field `statistic_range_expectation` after provisioning.\n"]
    pub fn statistic_range_expectation(
        &self,
    ) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_StatisticRangeExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statistic_range_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `suspended` after provisioning.\n"]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.base))
    }

    #[doc = "Get a reference to the value of field `table_condition_expectation` after provisioning.\n"]
    pub fn table_condition_expectation(
        &self,
    ) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_TableConditionExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_condition_expectation", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `uniqueness_expectation` after provisioning.\n"]
    pub fn uniqueness_expectation(
        &self,
    ) -> terrars::ListRef<DataDataplexDataQualityRules_Rules_UniquenessExpectationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uniqueness_expectation", self.base))
    }
}
