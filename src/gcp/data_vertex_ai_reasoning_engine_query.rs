#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVertexAiReasoningEngineQueryData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    class_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    reasoning_engine_id: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
}

struct DataVertexAiReasoningEngineQuery_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVertexAiReasoningEngineQueryData_>,
}

#[derive(Clone)]
pub struct DataVertexAiReasoningEngineQuery(std::rc::Rc<DataVertexAiReasoningEngineQuery_>);

impl DataVertexAiReasoningEngineQuery {
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
        "Set the field `class_method`.\nClass method to be used for the query. It is optional and defaults to \"query\" if unspecified."]
    pub fn set_class_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().class_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `input`.\nInput content provided by users in JSON object format. Examples include text query, function calling parameters, media bytes, etc.."]
    pub fn set_input(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().input = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project of the resource. If not provided, the provider default project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `class_method` after provisioning.\nClass method to be used for the query. It is optional and defaults to \"query\" if unspecified."]
    pub fn class_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `input` after provisioning.\nInput content provided by users in JSON object format. Examples include text query, function calling parameters, media bytes, etc.."]
    pub fn input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\nThe output of the query."]
    pub fn output(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project of the resource. If not provided, the provider default project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reasoning_engine_id` after provisioning.\nThe id of the Vertex Agent Engine to query."]
    pub fn reasoning_engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reasoning_engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe location of the resource."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataVertexAiReasoningEngineQuery {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVertexAiReasoningEngineQuery { }

impl terrars::ToListMappable for DataVertexAiReasoningEngineQuery {
    type O = terrars::ListRef<DataVertexAiReasoningEngineQueryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVertexAiReasoningEngineQuery_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vertex_ai_reasoning_engine_query".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVertexAiReasoningEngineQuery {
    pub tf_id: std::string::String,
    #[doc = "The id of the Vertex Agent Engine to query."]
    pub reasoning_engine_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildDataVertexAiReasoningEngineQuery {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVertexAiReasoningEngineQuery {
        let out = DataVertexAiReasoningEngineQuery(std::rc::Rc::new(DataVertexAiReasoningEngineQuery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVertexAiReasoningEngineQueryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                class_method: core::default::Default::default(),
                id: core::default::Default::default(),
                input: core::default::Default::default(),
                project: core::default::Default::default(),
                reasoning_engine_id: self.reasoning_engine_id,
                region: self.region,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVertexAiReasoningEngineQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiReasoningEngineQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVertexAiReasoningEngineQueryRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `class_method` after provisioning.\nClass method to be used for the query. It is optional and defaults to \"query\" if unspecified."]
    pub fn class_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.class_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `input` after provisioning.\nInput content provided by users in JSON object format. Examples include text query, function calling parameters, media bytes, etc.."]
    pub fn input(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output` after provisioning.\nThe output of the query."]
    pub fn output(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project of the resource. If not provided, the provider default project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reasoning_engine_id` after provisioning.\nThe id of the Vertex Agent Engine to query."]
    pub fn reasoning_engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reasoning_engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe location of the resource."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}
