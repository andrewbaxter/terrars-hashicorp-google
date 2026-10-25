#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LoggingSavedQueryData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    visibility: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_query: std::option::Option<std::vec::Vec<LoggingSavedQuery_LoggingQuery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ops_analytics_query: std::option::Option<std::vec::Vec<LoggingSavedQuery_OpsAnalyticsQuery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LoggingSavedQuery_Timeouts>,
    dynamic: LoggingSavedQueryDynamic,
}

struct LoggingSavedQuery_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoggingSavedQueryData_>,
}

#[derive(Clone)]
pub struct LoggingSavedQuery(std::rc::Rc<LoggingSavedQuery_>);

impl LoggingSavedQuery {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the saved query."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_query`.\n"]
    pub fn set_logging_query(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingSavedQuery_LoggingQuery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ops_analytics_query`.\n"]
    pub fn set_ops_analytics_query(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingSavedQuery_OpsAnalyticsQuery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ops_analytics_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ops_analytics_query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LoggingSavedQuery_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of the saved query."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the saved query."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-visible display name of the saved query."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource see\n[supported regions](https://docs.cloud.google.com/logging/docs/region-support#bucket-regions)."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the saved query. For example: 'my-saved-query'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of the saved query."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the saved query. Possible values: [\"SHARED\", \"PRIVATE\"]"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_query` after provisioning.\n"]
    pub fn logging_query(&self) -> terrars::ListRef<LoggingSavedQuery_LoggingQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_analytics_query` after provisioning.\n"]
    pub fn ops_analytics_query(&self) -> terrars::ListRef<LoggingSavedQuery_OpsAnalyticsQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ops_analytics_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingSavedQuery_TimeoutsRef {
        LoggingSavedQuery_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LoggingSavedQuery {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LoggingSavedQuery { }

impl terrars::ToListMappable for LoggingSavedQuery {
    type O = terrars::ListRef<LoggingSavedQueryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LoggingSavedQuery_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_logging_saved_query".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoggingSavedQuery {
    pub tf_id: std::string::String,
    #[doc = "The user-visible display name of the saved query."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The location of the resource see\n[supported regions](https://docs.cloud.google.com/logging/docs/region-support#bucket-regions)."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the saved query. For example: 'my-saved-query'"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The parent of the resource."]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = "The visibility of the saved query. Possible values: [\"SHARED\", \"PRIVATE\"]"]
    pub visibility: terrars::PrimField<std::string::String>,
}

impl BuildLoggingSavedQuery {
    pub fn build(self, stack: &mut terrars::Stack) -> LoggingSavedQuery {
        let out = LoggingSavedQuery(std::rc::Rc::new(LoggingSavedQuery_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoggingSavedQueryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                parent: self.parent,
                visibility: self.visibility,
                logging_query: core::default::Default::default(),
                ops_analytics_query: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoggingSavedQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingSavedQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoggingSavedQueryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of the saved query."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the saved query."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-visible display name of the saved query."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource see\n[supported regions](https://docs.cloud.google.com/logging/docs/region-support#bucket-regions)."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the saved query. For example: 'my-saved-query'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of the saved query."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nThe visibility of the saved query. Possible values: [\"SHARED\", \"PRIVATE\"]"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_query` after provisioning.\n"]
    pub fn logging_query(&self) -> terrars::ListRef<LoggingSavedQuery_LoggingQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_analytics_query` after provisioning.\n"]
    pub fn ops_analytics_query(&self) -> terrars::ListRef<LoggingSavedQuery_OpsAnalyticsQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ops_analytics_query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingSavedQuery_TimeoutsRef {
        LoggingSavedQuery_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LoggingSavedQueryDynamic {
    logging_query: std::option::Option<terrars::DynamicBlock<LoggingSavedQuery_LoggingQuery>>,
    ops_analytics_query: std::option::Option<terrars::DynamicBlock<LoggingSavedQuery_OpsAnalyticsQuery>>,
}

#[derive(Serialize)]
pub struct LoggingSavedQuery_LoggingQuery_SummaryFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoggingSavedQuery_LoggingQuery_SummaryFields {
    #[doc = "Set the field `field`.\nThe field from the LogEntry to include in the summary line."]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoggingSavedQuery_LoggingQuery_SummaryFields {
    type O = terrars::BlockAssignable<LoggingSavedQuery_LoggingQuery_SummaryFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingSavedQuery_LoggingQuery_SummaryFields {}

impl BuildLoggingSavedQuery_LoggingQuery_SummaryFields {
    pub fn build(self) -> LoggingSavedQuery_LoggingQuery_SummaryFields {
        LoggingSavedQuery_LoggingQuery_SummaryFields { field: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LoggingSavedQuery_LoggingQuery_SummaryFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingSavedQuery_LoggingQuery_SummaryFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoggingSavedQuery_LoggingQuery_SummaryFieldsRef {
        LoggingSavedQuery_LoggingQuery_SummaryFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingSavedQuery_LoggingQuery_SummaryFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nThe field from the LogEntry to include in the summary line."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoggingSavedQuery_LoggingQueryDynamic {
    summary_fields: std::option::Option<terrars::DynamicBlock<LoggingSavedQuery_LoggingQuery_SummaryFields>>,
}

#[derive(Serialize)]
pub struct LoggingSavedQuery_LoggingQuery {
    filter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary_field_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary_field_start: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summary_fields: std::option::Option<std::vec::Vec<LoggingSavedQuery_LoggingQuery_SummaryFields>>,
    dynamic: LoggingSavedQuery_LoggingQueryDynamic,
}

impl LoggingSavedQuery_LoggingQuery {
    #[doc = "Set the field `summary_field_end`.\nCharacters will be counted from the end of the string."]
    pub fn set_summary_field_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.summary_field_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summary_field_start`.\nCharacters will be counted from the start of the string."]
    pub fn set_summary_field_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.summary_field_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summary_fields`.\n"]
    pub fn set_summary_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingSavedQuery_LoggingQuery_SummaryFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.summary_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.summary_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LoggingSavedQuery_LoggingQuery {
    type O = terrars::BlockAssignable<LoggingSavedQuery_LoggingQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingSavedQuery_LoggingQuery {
    #[doc =
        "An [advanced logs filter](https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildLoggingSavedQuery_LoggingQuery {
    pub fn build(self) -> LoggingSavedQuery_LoggingQuery {
        LoggingSavedQuery_LoggingQuery {
            filter: self.filter,
            summary_field_end: core::default::Default::default(),
            summary_field_start: core::default::Default::default(),
            summary_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoggingSavedQuery_LoggingQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingSavedQuery_LoggingQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingSavedQuery_LoggingQueryRef {
        LoggingSavedQuery_LoggingQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingSavedQuery_LoggingQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nAn [advanced logs filter](https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `summary_field_end` after provisioning.\nCharacters will be counted from the end of the string."]
    pub fn summary_field_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary_field_end", self.base))
    }

    #[doc =
        "Get a reference to the value of field `summary_field_start` after provisioning.\nCharacters will be counted from the start of the string."]
    pub fn summary_field_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.summary_field_start", self.base))
    }

    #[doc = "Get a reference to the value of field `summary_fields` after provisioning.\n"]
    pub fn summary_fields(&self) -> terrars::ListRef<LoggingSavedQuery_LoggingQuery_SummaryFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summary_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingSavedQuery_OpsAnalyticsQuery {
    sql_query_text: terrars::PrimField<std::string::String>,
}

impl LoggingSavedQuery_OpsAnalyticsQuery { }

impl terrars::ToListMappable for LoggingSavedQuery_OpsAnalyticsQuery {
    type O = terrars::BlockAssignable<LoggingSavedQuery_OpsAnalyticsQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingSavedQuery_OpsAnalyticsQuery {
    #[doc = "A logs analytics SQL query, which generally follows BigQuery format."]
    pub sql_query_text: terrars::PrimField<std::string::String>,
}

impl BuildLoggingSavedQuery_OpsAnalyticsQuery {
    pub fn build(self) -> LoggingSavedQuery_OpsAnalyticsQuery {
        LoggingSavedQuery_OpsAnalyticsQuery { sql_query_text: self.sql_query_text }
    }
}

#[allow(dead_code)]
pub struct LoggingSavedQuery_OpsAnalyticsQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingSavedQuery_OpsAnalyticsQueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingSavedQuery_OpsAnalyticsQueryRef {
        LoggingSavedQuery_OpsAnalyticsQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingSavedQuery_OpsAnalyticsQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sql_query_text` after provisioning.\nA logs analytics SQL query, which generally follows BigQuery format."]
    pub fn sql_query_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_query_text", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingSavedQuery_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoggingSavedQuery_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoggingSavedQuery_Timeouts {
    type O = terrars::BlockAssignable<LoggingSavedQuery_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingSavedQuery_Timeouts {}

impl BuildLoggingSavedQuery_Timeouts {
    pub fn build(self) -> LoggingSavedQuery_Timeouts {
        LoggingSavedQuery_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoggingSavedQuery_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingSavedQuery_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingSavedQuery_TimeoutsRef {
        LoggingSavedQuery_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingSavedQuery_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
