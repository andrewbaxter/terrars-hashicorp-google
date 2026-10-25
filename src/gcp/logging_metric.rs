#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LoggingMetricData_ {
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
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    filter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_extractors: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_extractor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_options: std::option::Option<std::vec::Vec<LoggingMetric_BucketOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_descriptor: std::option::Option<std::vec::Vec<LoggingMetric_MetricDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LoggingMetric_Timeouts>,
    dynamic: LoggingMetricDynamic,
}

struct LoggingMetric_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoggingMetricData_>,
}

#[derive(Clone)]
pub struct LoggingMetric(std::rc::Rc<LoggingMetric_>);

impl LoggingMetric {
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
        "Set the field `bucket_name`.\nThe resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects\nare supported. The bucket has to be in the same project as the metric."]
    pub fn set_bucket_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nA description of this metric, which is used in documentation. The maximum length of the\ndescription is 8000 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nIf set to True, then this metric is disabled and it does not generate any points."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `label_extractors`.\nA map from a label key string to an extractor expression which is used to extract data from a log\nentry field and assign as the label value. Each label key specified in the LabelDescriptor must\nhave an associated extractor expression in this map. The syntax of the extractor expression is\nthe same as for the valueExtractor field."]
    pub fn set_label_extractors(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().label_extractors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value_extractor`.\nA valueExtractor is required when using a distribution logs-based metric to extract the values to\nrecord from a log entry. Two functions are supported for value extraction - EXTRACT(field) or\nREGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which\nthe value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax\n(https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified\nlog entry field. The value of the field is converted to a string before applying the regex. It is an\nerror to specify a regex that does not include exactly one capture group."]
    pub fn set_value_extractor(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().value_extractor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_options`.\n"]
    pub fn set_bucket_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingMetric_BucketOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bucket_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bucket_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_descriptor`.\n"]
    pub fn set_metric_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingMetric_MetricDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LoggingMetric_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects\nare supported. The bucket has to be in the same project as the metric."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of this metric, which is used in documentation. The maximum length of the\ndescription is 8000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this metric is disabled and it does not generate any points."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nAn advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_extractors` after provisioning.\nA map from a label key string to an extractor expression which is used to extract data from a log\nentry field and assign as the label value. Each label key specified in the LabelDescriptor must\nhave an associated extractor expression in this map. The syntax of the extractor expression is\nthe same as for the valueExtractor field."]
    pub fn label_extractors(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.label_extractors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe client-assigned metric identifier. Examples - \"error_count\", \"nginx/requests\".\nMetric identifiers are limited to 100 characters and can include only the following\ncharacters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash\ncharacter (/) denotes a hierarchy of name pieces, and it cannot be the first character\nof the name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `value_extractor` after provisioning.\nA valueExtractor is required when using a distribution logs-based metric to extract the values to\nrecord from a log entry. Two functions are supported for value extraction - EXTRACT(field) or\nREGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which\nthe value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax\n(https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified\nlog entry field. The value of the field is converted to a string before applying the regex. It is an\nerror to specify a regex that does not include exactly one capture group."]
    pub fn value_extractor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_extractor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_options` after provisioning.\n"]
    pub fn bucket_options(&self) -> terrars::ListRef<LoggingMetric_BucketOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_descriptor` after provisioning.\n"]
    pub fn metric_descriptor(&self) -> terrars::ListRef<LoggingMetric_MetricDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingMetric_TimeoutsRef {
        LoggingMetric_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LoggingMetric {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LoggingMetric { }

impl terrars::ToListMappable for LoggingMetric {
    type O = terrars::ListRef<LoggingMetricRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LoggingMetric_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_logging_metric".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoggingMetric {
    pub tf_id: std::string::String,
    #[doc =
        "An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub filter: terrars::PrimField<std::string::String>,
    #[doc =
        "The client-assigned metric identifier. Examples - \"error_count\", \"nginx/requests\".\nMetric identifiers are limited to 100 characters and can include only the following\ncharacters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash\ncharacter (/) denotes a hierarchy of name pieces, and it cannot be the first character\nof the name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLoggingMetric {
    pub fn build(self, stack: &mut terrars::Stack) -> LoggingMetric {
        let out = LoggingMetric(std::rc::Rc::new(LoggingMetric_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoggingMetricData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                filter: self.filter,
                id: core::default::Default::default(),
                label_extractors: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                value_extractor: core::default::Default::default(),
                bucket_options: core::default::Default::default(),
                metric_descriptor: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoggingMetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoggingMetricRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects\nare supported. The bucket has to be in the same project as the metric."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of this metric, which is used in documentation. The maximum length of the\ndescription is 8000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf set to True, then this metric is disabled and it does not generate any points."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nAn advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which\nis used to match log entries."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_extractors` after provisioning.\nA map from a label key string to an extractor expression which is used to extract data from a log\nentry field and assign as the label value. Each label key specified in the LabelDescriptor must\nhave an associated extractor expression in this map. The syntax of the extractor expression is\nthe same as for the valueExtractor field."]
    pub fn label_extractors(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.label_extractors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe client-assigned metric identifier. Examples - \"error_count\", \"nginx/requests\".\nMetric identifiers are limited to 100 characters and can include only the following\ncharacters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash\ncharacter (/) denotes a hierarchy of name pieces, and it cannot be the first character\nof the name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `value_extractor` after provisioning.\nA valueExtractor is required when using a distribution logs-based metric to extract the values to\nrecord from a log entry. Two functions are supported for value extraction - EXTRACT(field) or\nREGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which\nthe value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax\n(https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified\nlog entry field. The value of the field is converted to a string before applying the regex. It is an\nerror to specify a regex that does not include exactly one capture group."]
    pub fn value_extractor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_extractor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_options` after provisioning.\n"]
    pub fn bucket_options(&self) -> terrars::ListRef<LoggingMetric_BucketOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_descriptor` after provisioning.\n"]
    pub fn metric_descriptor(&self) -> terrars::ListRef<LoggingMetric_MetricDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LoggingMetric_TimeoutsRef {
        LoggingMetric_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LoggingMetricDynamic {
    bucket_options: std::option::Option<terrars::DynamicBlock<LoggingMetric_BucketOptions>>,
    metric_descriptor: std::option::Option<terrars::DynamicBlock<LoggingMetric_MetricDescriptor>>,
}

#[derive(Serialize)]
pub struct LoggingMetric_BucketOptions_ExplicitBuckets {
    bounds: terrars::ListField<terrars::PrimField<f64>>,
}

impl LoggingMetric_BucketOptions_ExplicitBuckets { }

impl terrars::ToListMappable for LoggingMetric_BucketOptions_ExplicitBuckets {
    type O = terrars::BlockAssignable<LoggingMetric_BucketOptions_ExplicitBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_BucketOptions_ExplicitBuckets {
    #[doc = "The values must be monotonically increasing."]
    pub bounds: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildLoggingMetric_BucketOptions_ExplicitBuckets {
    pub fn build(self) -> LoggingMetric_BucketOptions_ExplicitBuckets {
        LoggingMetric_BucketOptions_ExplicitBuckets { bounds: self.bounds }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_BucketOptions_ExplicitBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_BucketOptions_ExplicitBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoggingMetric_BucketOptions_ExplicitBucketsRef {
        LoggingMetric_BucketOptions_ExplicitBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_BucketOptions_ExplicitBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bounds` after provisioning.\nThe values must be monotonically increasing."]
    pub fn bounds(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bounds", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingMetric_BucketOptions_ExponentialBuckets {
    growth_factor: terrars::PrimField<f64>,
    num_finite_buckets: terrars::PrimField<f64>,
    scale: terrars::PrimField<f64>,
}

impl LoggingMetric_BucketOptions_ExponentialBuckets { }

impl terrars::ToListMappable for LoggingMetric_BucketOptions_ExponentialBuckets {
    type O = terrars::BlockAssignable<LoggingMetric_BucketOptions_ExponentialBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_BucketOptions_ExponentialBuckets {
    #[doc = "Must be greater than 1."]
    pub growth_factor: terrars::PrimField<f64>,
    #[doc = "Must be greater than 0."]
    pub num_finite_buckets: terrars::PrimField<f64>,
    #[doc = "Must be greater than 0."]
    pub scale: terrars::PrimField<f64>,
}

impl BuildLoggingMetric_BucketOptions_ExponentialBuckets {
    pub fn build(self) -> LoggingMetric_BucketOptions_ExponentialBuckets {
        LoggingMetric_BucketOptions_ExponentialBuckets {
            growth_factor: self.growth_factor,
            num_finite_buckets: self.num_finite_buckets,
            scale: self.scale,
        }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_BucketOptions_ExponentialBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_BucketOptions_ExponentialBucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LoggingMetric_BucketOptions_ExponentialBucketsRef {
        LoggingMetric_BucketOptions_ExponentialBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_BucketOptions_ExponentialBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `growth_factor` after provisioning.\nMust be greater than 1."]
    pub fn growth_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.growth_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `num_finite_buckets` after provisioning.\nMust be greater than 0."]
    pub fn num_finite_buckets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_finite_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nMust be greater than 0."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingMetric_BucketOptions_LinearBuckets {
    num_finite_buckets: terrars::PrimField<f64>,
    offset: terrars::PrimField<f64>,
    width: terrars::PrimField<f64>,
}

impl LoggingMetric_BucketOptions_LinearBuckets { }

impl terrars::ToListMappable for LoggingMetric_BucketOptions_LinearBuckets {
    type O = terrars::BlockAssignable<LoggingMetric_BucketOptions_LinearBuckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_BucketOptions_LinearBuckets {
    #[doc = "Must be greater than 0."]
    pub num_finite_buckets: terrars::PrimField<f64>,
    #[doc = "Lower bound of the first bucket."]
    pub offset: terrars::PrimField<f64>,
    #[doc = "Must be greater than 0."]
    pub width: terrars::PrimField<f64>,
}

impl BuildLoggingMetric_BucketOptions_LinearBuckets {
    pub fn build(self) -> LoggingMetric_BucketOptions_LinearBuckets {
        LoggingMetric_BucketOptions_LinearBuckets {
            num_finite_buckets: self.num_finite_buckets,
            offset: self.offset,
            width: self.width,
        }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_BucketOptions_LinearBucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_BucketOptions_LinearBucketsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingMetric_BucketOptions_LinearBucketsRef {
        LoggingMetric_BucketOptions_LinearBucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_BucketOptions_LinearBucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `num_finite_buckets` after provisioning.\nMust be greater than 0."]
    pub fn num_finite_buckets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_finite_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `offset` after provisioning.\nLower bound of the first bucket."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }

    #[doc = "Get a reference to the value of field `width` after provisioning.\nMust be greater than 0."]
    pub fn width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.width", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoggingMetric_BucketOptionsDynamic {
    explicit_buckets: std::option::Option<terrars::DynamicBlock<LoggingMetric_BucketOptions_ExplicitBuckets>>,
    exponential_buckets: std::option::Option<terrars::DynamicBlock<LoggingMetric_BucketOptions_ExponentialBuckets>>,
    linear_buckets: std::option::Option<terrars::DynamicBlock<LoggingMetric_BucketOptions_LinearBuckets>>,
}

#[derive(Serialize)]
pub struct LoggingMetric_BucketOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explicit_buckets: std::option::Option<std::vec::Vec<LoggingMetric_BucketOptions_ExplicitBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exponential_buckets: std::option::Option<std::vec::Vec<LoggingMetric_BucketOptions_ExponentialBuckets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linear_buckets: std::option::Option<std::vec::Vec<LoggingMetric_BucketOptions_LinearBuckets>>,
    dynamic: LoggingMetric_BucketOptionsDynamic,
}

impl LoggingMetric_BucketOptions {
    #[doc = "Set the field `explicit_buckets`.\n"]
    pub fn set_explicit_buckets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingMetric_BucketOptions_ExplicitBuckets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.explicit_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.explicit_buckets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exponential_buckets`.\n"]
    pub fn set_exponential_buckets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingMetric_BucketOptions_ExponentialBuckets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exponential_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exponential_buckets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linear_buckets`.\n"]
    pub fn set_linear_buckets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingMetric_BucketOptions_LinearBuckets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linear_buckets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linear_buckets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LoggingMetric_BucketOptions {
    type O = terrars::BlockAssignable<LoggingMetric_BucketOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_BucketOptions {}

impl BuildLoggingMetric_BucketOptions {
    pub fn build(self) -> LoggingMetric_BucketOptions {
        LoggingMetric_BucketOptions {
            explicit_buckets: core::default::Default::default(),
            exponential_buckets: core::default::Default::default(),
            linear_buckets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_BucketOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_BucketOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingMetric_BucketOptionsRef {
        LoggingMetric_BucketOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_BucketOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `explicit_buckets` after provisioning.\n"]
    pub fn explicit_buckets(&self) -> terrars::ListRef<LoggingMetric_BucketOptions_ExplicitBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explicit_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `exponential_buckets` after provisioning.\n"]
    pub fn exponential_buckets(&self) -> terrars::ListRef<LoggingMetric_BucketOptions_ExponentialBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exponential_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `linear_buckets` after provisioning.\n"]
    pub fn linear_buckets(&self) -> terrars::ListRef<LoggingMetric_BucketOptions_LinearBucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linear_buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingMetric_MetricDescriptor_Labels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoggingMetric_MetricDescriptor_Labels {
    #[doc = "Set the field `description`.\nA human-readable description for the label."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value_type`.\nThe type of data that can be assigned to the label. Default value: \"STRING\" Possible values: [\"BOOL\", \"INT64\", \"STRING\"]"]
    pub fn set_value_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LoggingMetric_MetricDescriptor_Labels {
    type O = terrars::BlockAssignable<LoggingMetric_MetricDescriptor_Labels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_MetricDescriptor_Labels {
    #[doc = "The label key."]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildLoggingMetric_MetricDescriptor_Labels {
    pub fn build(self) -> LoggingMetric_MetricDescriptor_Labels {
        LoggingMetric_MetricDescriptor_Labels {
            description: core::default::Default::default(),
            key: self.key,
            value_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_MetricDescriptor_LabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_MetricDescriptor_LabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingMetric_MetricDescriptor_LabelsRef {
        LoggingMetric_MetricDescriptor_LabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_MetricDescriptor_LabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the label."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe label key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nThe type of data that can be assigned to the label. Default value: \"STRING\" Possible values: [\"BOOL\", \"INT64\", \"STRING\"]"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoggingMetric_MetricDescriptorDynamic {
    labels: std::option::Option<terrars::DynamicBlock<LoggingMetric_MetricDescriptor_Labels>>,
}

#[derive(Serialize)]
pub struct LoggingMetric_MetricDescriptor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    metric_kind: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<LoggingMetric_MetricDescriptor_Labels>>,
    dynamic: LoggingMetric_MetricDescriptorDynamic,
}

impl LoggingMetric_MetricDescriptor {
    #[doc =
        "Set the field `display_name`.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case\nwithout an ending period, for example \"Request count\". This field is optional but it is\nrecommended to be set for any metrics associated with user-visible concepts, such as Quota."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit`.\nThe unit in which the metric value is reported. It is only applicable if the valueType is\n'INT64', 'DOUBLE', or 'DISTRIBUTION'. The supported units are a subset of\n[The Unified Code for Units of Measure](http://unitsofmeasure.org/ucum.html) standard"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LoggingMetric_MetricDescriptor_Labels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LoggingMetric_MetricDescriptor {
    type O = terrars::BlockAssignable<LoggingMetric_MetricDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_MetricDescriptor {
    #[doc =
        "Whether the metric records instantaneous values, changes to a value, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to DELTA. Possible values: [\"DELTA\", \"GAUGE\", \"CUMULATIVE\"]"]
    pub metric_kind: terrars::PrimField<std::string::String>,
    #[doc =
        "Whether the measurement is an integer, a floating-point number, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to INT64. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\", \"MONEY\"]"]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildLoggingMetric_MetricDescriptor {
    pub fn build(self) -> LoggingMetric_MetricDescriptor {
        LoggingMetric_MetricDescriptor {
            display_name: core::default::Default::default(),
            metric_kind: self.metric_kind,
            unit: core::default::Default::default(),
            value_type: self.value_type,
            labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_MetricDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_MetricDescriptorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingMetric_MetricDescriptorRef {
        LoggingMetric_MetricDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_MetricDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA concise name for the metric, which can be displayed in user interfaces. Use sentence case\nwithout an ending period, for example \"Request count\". This field is optional but it is\nrecommended to be set for any metrics associated with user-visible concepts, such as Quota."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metric_kind` after provisioning.\nWhether the metric records instantaneous values, changes to a value, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to DELTA. Possible values: [\"DELTA\", \"GAUGE\", \"CUMULATIVE\"]"]
    pub fn metric_kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_kind", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unit` after provisioning.\nThe unit in which the metric value is reported. It is only applicable if the valueType is\n'INT64', 'DOUBLE', or 'DISTRIBUTION'. The supported units are a subset of\n[The Unified Code for Units of Measure](http://unitsofmeasure.org/ucum.html) standard"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nWhether the measurement is an integer, a floating-point number, etc.\nSome combinations of metricKind and valueType might not be supported.\nFor counter metrics, set this to INT64. Possible values: [\"BOOL\", \"INT64\", \"DOUBLE\", \"STRING\", \"DISTRIBUTION\", \"MONEY\"]"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}

#[derive(Serialize)]
pub struct LoggingMetric_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LoggingMetric_Timeouts {
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

impl terrars::ToListMappable for LoggingMetric_Timeouts {
    type O = terrars::BlockAssignable<LoggingMetric_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoggingMetric_Timeouts {}

impl BuildLoggingMetric_Timeouts {
    pub fn build(self) -> LoggingMetric_Timeouts {
        LoggingMetric_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LoggingMetric_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoggingMetric_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LoggingMetric_TimeoutsRef {
        LoggingMetric_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoggingMetric_TimeoutsRef {
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
