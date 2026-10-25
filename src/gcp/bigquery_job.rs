#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryJobData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    job_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_timeout_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy: std::option::Option<std::vec::Vec<BigqueryJob_Copy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extract: std::option::Option<std::vec::Vec<BigqueryJob_Extract>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load: std::option::Option<std::vec::Vec<BigqueryJob_Load>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<std::vec::Vec<BigqueryJob_Query>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryJob_Timeouts>,
    dynamic: BigqueryJobDynamic,
}

struct BigqueryJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryJobData_>,
}

#[derive(Clone)]
pub struct BigqueryJob(std::rc::Rc<BigqueryJob_>);

impl BigqueryJob {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `job_timeout_ms`.\nJob timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
    pub fn set_job_timeout_ms(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().job_timeout_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe geographic location of the job. The default value is US."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy`.\n"]
    pub fn set_copy(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().copy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.copy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `extract`.\n"]
    pub fn set_extract(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Extract>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extract = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extract = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load`.\n"]
    pub fn set_load(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Load>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Query>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `job_timeout_ms` after provisioning.\nJob timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
    pub fn job_timeout_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_timeout_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\nThe type of the job."]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location of the job. The default value is US."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
    pub fn status(&self) -> terrars::ListRef<BigqueryJob_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_email` after provisioning.\nEmail address of the user who ran the job."]
    pub fn user_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy` after provisioning.\n"]
    pub fn copy(&self) -> terrars::ListRef<BigqueryJob_CopyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extract` after provisioning.\n"]
    pub fn extract(&self) -> terrars::ListRef<BigqueryJob_ExtractRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extract", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load` after provisioning.\n"]
    pub fn load(&self) -> terrars::ListRef<BigqueryJob_LoadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::ListRef<BigqueryJob_QueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryJob_TimeoutsRef {
        BigqueryJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigqueryJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryJob { }

impl terrars::ToListMappable for BigqueryJob {
    type O = terrars::ListRef<BigqueryJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryJob {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
    pub job_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryJob {
        let out = BigqueryJob(std::rc::Rc::new(BigqueryJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                job_id: self.job_id,
                job_timeout_ms: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                copy: core::default::Default::default(),
                extract: core::default::Default::default(),
                load: core::default::Default::default(),
                query: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `job_id` after provisioning.\nThe ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters."]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `job_timeout_ms` after provisioning.\nJob timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job."]
    pub fn job_timeout_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_timeout_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_type` after provisioning.\nThe type of the job."]
    pub fn job_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this job. You can use these to organize and group your jobs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location of the job. The default value is US."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this job. Examine this value when polling an asynchronous job to see if the job is complete."]
    pub fn status(&self) -> terrars::ListRef<BigqueryJob_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_email` after provisioning.\nEmail address of the user who ran the job."]
    pub fn user_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy` after provisioning.\n"]
    pub fn copy(&self) -> terrars::ListRef<BigqueryJob_CopyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `extract` after provisioning.\n"]
    pub fn extract(&self) -> terrars::ListRef<BigqueryJob_ExtractRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extract", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load` after provisioning.\n"]
    pub fn load(&self) -> terrars::ListRef<BigqueryJob_LoadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::ListRef<BigqueryJob_QueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryJob_TimeoutsRef {
        BigqueryJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BigqueryJobDynamic {
    copy: std::option::Option<terrars::DynamicBlock<BigqueryJob_Copy>>,
    extract: std::option::Option<terrars::DynamicBlock<BigqueryJob_Extract>>,
    load: std::option::Option<terrars::DynamicBlock<BigqueryJob_Load>>,
    query: std::option::Option<terrars::DynamicBlock<BigqueryJob_Query>>,
}

#[derive(Serialize)]
pub struct BigqueryJob_Status_ErrorResult {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Status_ErrorResult {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Status_ErrorResult {
    type O = terrars::BlockAssignable<BigqueryJob_Status_ErrorResult>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Status_ErrorResult {}

impl BuildBigqueryJob_Status_ErrorResult {
    pub fn build(self) -> BigqueryJob_Status_ErrorResult {
        BigqueryJob_Status_ErrorResult {
            location: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Status_ErrorResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Status_ErrorResultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Status_ErrorResultRef {
        BigqueryJob_Status_ErrorResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Status_ErrorResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Status_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Status_Errors {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Status_Errors {
    type O = terrars::BlockAssignable<BigqueryJob_Status_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Status_Errors {}

impl BuildBigqueryJob_Status_Errors {
    pub fn build(self) -> BigqueryJob_Status_Errors {
        BigqueryJob_Status_Errors {
            location: core::default::Default::default(),
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Status_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Status_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Status_ErrorsRef {
        BigqueryJob_Status_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Status_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_result: std::option::Option<terrars::ListField<BigqueryJob_Status_ErrorResult>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    errors: std::option::Option<terrars::ListField<BigqueryJob_Status_Errors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Status {
    #[doc = "Set the field `error_result`.\n"]
    pub fn set_error_result(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BigqueryJob_Status_ErrorResult>>,
    ) -> Self {
        self.error_result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `errors`.\n"]
    pub fn set_errors(mut self, v: impl std::convert::Into<terrars::ListField<BigqueryJob_Status_Errors>>) -> Self {
        self.errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Status {
    type O = terrars::BlockAssignable<BigqueryJob_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Status {}

impl BuildBigqueryJob_Status {
    pub fn build(self) -> BigqueryJob_Status {
        BigqueryJob_Status {
            error_result: core::default::Default::default(),
            errors: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_StatusRef {
        BigqueryJob_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_result` after provisioning.\n"]
    pub fn error_result(&self) -> terrars::ListRef<BigqueryJob_Status_ErrorResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_result", self.base))
    }

    #[doc = "Get a reference to the value of field `errors` after provisioning.\n"]
    pub fn errors(&self) -> terrars::ListRef<BigqueryJob_Status_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Copy_DestinationEncryptionConfiguration {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Copy_DestinationEncryptionConfiguration { }

impl terrars::ToListMappable for BigqueryJob_Copy_DestinationEncryptionConfiguration {
    type O = terrars::BlockAssignable<BigqueryJob_Copy_DestinationEncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Copy_DestinationEncryptionConfiguration {
    #[doc =
        "Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Copy_DestinationEncryptionConfiguration {
    pub fn build(self) -> BigqueryJob_Copy_DestinationEncryptionConfiguration {
        BigqueryJob_Copy_DestinationEncryptionConfiguration { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Copy_DestinationEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Copy_DestinationEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryJob_Copy_DestinationEncryptionConfigurationRef {
        BigqueryJob_Copy_DestinationEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Copy_DestinationEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery table.\nThe BigQuery Service Account associated with your project requires access to this encryption key."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_version` after provisioning.\nDescribes the Cloud KMS encryption key version used to protect destination BigQuery table."]
    pub fn kms_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Copy_DestinationTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    table_id: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Copy_DestinationTable {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Copy_DestinationTable {
    type O = terrars::BlockAssignable<BigqueryJob_Copy_DestinationTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Copy_DestinationTable {
    #[doc =
        "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Copy_DestinationTable {
    pub fn build(self) -> BigqueryJob_Copy_DestinationTable {
        BigqueryJob_Copy_DestinationTable {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Copy_DestinationTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Copy_DestinationTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Copy_DestinationTableRef {
        BigqueryJob_Copy_DestinationTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Copy_DestinationTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Copy_SourceTables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    table_id: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Copy_SourceTables {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Copy_SourceTables {
    type O = terrars::BlockAssignable<BigqueryJob_Copy_SourceTables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Copy_SourceTables {
    #[doc =
        "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Copy_SourceTables {
    pub fn build(self) -> BigqueryJob_Copy_SourceTables {
        BigqueryJob_Copy_SourceTables {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Copy_SourceTablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Copy_SourceTablesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Copy_SourceTablesRef {
        BigqueryJob_Copy_SourceTablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Copy_SourceTablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryJob_CopyDynamic {
    destination_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    destination_table: std::option::Option<terrars::DynamicBlock<BigqueryJob_Copy_DestinationTable>>,
    source_tables: std::option::Option<terrars::DynamicBlock<BigqueryJob_Copy_SourceTables>>,
}

#[derive(Serialize)]
pub struct BigqueryJob_Copy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_table: std::option::Option<std::vec::Vec<BigqueryJob_Copy_DestinationTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_tables: std::option::Option<std::vec::Vec<BigqueryJob_Copy_SourceTables>>,
    dynamic: BigqueryJob_CopyDynamic,
}

impl BigqueryJob_Copy {
    #[doc =
        "Set the field `create_disposition`.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn set_create_disposition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_disposition`.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn set_write_disposition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.write_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_encryption_configuration`.\n"]
    pub fn set_destination_encryption_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_DestinationTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_tables`.\n"]
    pub fn set_source_tables(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_SourceTables>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Copy {
    type O = terrars::BlockAssignable<BigqueryJob_Copy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Copy {}

impl BuildBigqueryJob_Copy {
    pub fn build(self) -> BigqueryJob_Copy {
        BigqueryJob_Copy {
            create_disposition: core::default::Default::default(),
            write_disposition: core::default::Default::default(),
            destination_encryption_configuration: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            source_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_CopyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_CopyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_CopyRef {
        BigqueryJob_CopyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_CopyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_disposition` after provisioning.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn create_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_disposition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_disposition` after provisioning.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn write_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_disposition", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_encryption_configuration` after provisioning.\n"]
    pub fn destination_encryption_configuration(
        &self,
    ) -> terrars::ListRef<BigqueryJob_Copy_DestinationEncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> terrars::ListRef<BigqueryJob_Copy_DestinationTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_table", self.base))
    }

    #[doc = "Get a reference to the value of field `source_tables` after provisioning.\n"]
    pub fn source_tables(&self) -> terrars::ListRef<BigqueryJob_Copy_SourceTablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_tables", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Extract_SourceModel {
    dataset_id: terrars::PrimField<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Extract_SourceModel { }

impl terrars::ToListMappable for BigqueryJob_Extract_SourceModel {
    type O = terrars::BlockAssignable<BigqueryJob_Extract_SourceModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Extract_SourceModel {
    #[doc = "The ID of the dataset containing this model."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the model."]
    pub model_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this model."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Extract_SourceModel {
    pub fn build(self) -> BigqueryJob_Extract_SourceModel {
        BigqueryJob_Extract_SourceModel {
            dataset_id: self.dataset_id,
            model_id: self.model_id,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Extract_SourceModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Extract_SourceModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Extract_SourceModelRef {
        BigqueryJob_Extract_SourceModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Extract_SourceModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this model."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\nThe ID of the model."]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this model."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Extract_SourceTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    table_id: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Extract_SourceTable {
    #[doc = "Set the field `dataset_id`.\nThe ID of the dataset containing this table."]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Extract_SourceTable {
    type O = terrars::BlockAssignable<BigqueryJob_Extract_SourceTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Extract_SourceTable {
    #[doc =
        "The table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Extract_SourceTable {
    pub fn build(self) -> BigqueryJob_Extract_SourceTable {
        BigqueryJob_Extract_SourceTable {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Extract_SourceTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Extract_SourceTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Extract_SourceTableRef {
        BigqueryJob_Extract_SourceTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Extract_SourceTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe table. Can be specified '{{table_id}}' if 'project_id' and 'dataset_id' are also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}' if not."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryJob_ExtractDynamic {
    source_model: std::option::Option<terrars::DynamicBlock<BigqueryJob_Extract_SourceModel>>,
    source_table: std::option::Option<terrars::DynamicBlock<BigqueryJob_Extract_SourceTable>>,
}

#[derive(Serialize)]
pub struct BigqueryJob_Extract {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_format: std::option::Option<terrars::PrimField<std::string::String>>,
    destination_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    print_header: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_avro_logical_types: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_model: std::option::Option<std::vec::Vec<BigqueryJob_Extract_SourceModel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_table: std::option::Option<std::vec::Vec<BigqueryJob_Extract_SourceTable>>,
    dynamic: BigqueryJob_ExtractDynamic,
}

impl BigqueryJob_Extract {
    #[doc =
        "Set the field `compression`.\nThe compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE.\nThe default value is NONE. DEFLATE and SNAPPY are only supported for Avro."]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `destination_format`.\nThe exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO for tables and SAVED_MODEL for models.\nThe default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV.\nThe default value for models is SAVED_MODEL."]
    pub fn set_destination_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_delimiter`.\nWhen extracting data in CSV format, this defines the delimiter to use between fields in the exported data.\nDefault is ','"]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `print_header`.\nWhether to print out a header row in the results. Default is true."]
    pub fn set_print_header(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.print_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_avro_logical_types`.\nWhether to use logical types when extracting to AVRO format."]
    pub fn set_use_avro_logical_types(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_avro_logical_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_model`.\n"]
    pub fn set_source_model(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Extract_SourceModel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_table`.\n"]
    pub fn set_source_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Extract_SourceTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Extract {
    type O = terrars::BlockAssignable<BigqueryJob_Extract>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Extract {
    #[doc = "A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
    pub destination_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBigqueryJob_Extract {
    pub fn build(self) -> BigqueryJob_Extract {
        BigqueryJob_Extract {
            compression: core::default::Default::default(),
            destination_format: core::default::Default::default(),
            destination_uris: self.destination_uris,
            field_delimiter: core::default::Default::default(),
            print_header: core::default::Default::default(),
            use_avro_logical_types: core::default::Default::default(),
            source_model: core::default::Default::default(),
            source_table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_ExtractRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_ExtractRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_ExtractRef {
        BigqueryJob_ExtractRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_ExtractRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compression` after provisioning.\nThe compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE.\nThe default value is NONE. DEFLATE and SNAPPY are only supported for Avro."]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_format` after provisioning.\nThe exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO for tables and SAVED_MODEL for models.\nThe default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV.\nThe default value for models is SAVED_MODEL."]
    pub fn destination_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_uris` after provisioning.\nA list of fully-qualified Google Cloud Storage URIs where the extracted table should be written."]
    pub fn destination_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_delimiter` after provisioning.\nWhen extracting data in CSV format, this defines the delimiter to use between fields in the exported data.\nDefault is ','"]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `print_header` after provisioning.\nWhether to print out a header row in the results. Default is true."]
    pub fn print_header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.print_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_avro_logical_types` after provisioning.\nWhether to use logical types when extracting to AVRO format."]
    pub fn use_avro_logical_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_avro_logical_types", self.base))
    }

    #[doc = "Get a reference to the value of field `source_model` after provisioning.\n"]
    pub fn source_model(&self) -> terrars::ListRef<BigqueryJob_Extract_SourceModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_model", self.base))
    }

    #[doc = "Get a reference to the value of field `source_table` after provisioning.\n"]
    pub fn source_table(&self) -> terrars::ListRef<BigqueryJob_Extract_SourceTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_table", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Load_ParquetOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_list_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_as_string: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryJob_Load_ParquetOptions {
    #[doc =
        "Set the field `enable_list_inference`.\nIf sourceFormat is set to PARQUET, indicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn set_enable_list_inference(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_list_inference = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enum_as_string`.\nIf sourceFormat is set to PARQUET, indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn set_enum_as_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enum_as_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Load_ParquetOptions {
    type O = terrars::BlockAssignable<BigqueryJob_Load_ParquetOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Load_ParquetOptions {}

impl BuildBigqueryJob_Load_ParquetOptions {
    pub fn build(self) -> BigqueryJob_Load_ParquetOptions {
        BigqueryJob_Load_ParquetOptions {
            enable_list_inference: core::default::Default::default(),
            enum_as_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Load_ParquetOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Load_ParquetOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Load_ParquetOptionsRef {
        BigqueryJob_Load_ParquetOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Load_ParquetOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_list_inference` after provisioning.\nIf sourceFormat is set to PARQUET, indicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn enable_list_inference(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_list_inference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enum_as_string` after provisioning.\nIf sourceFormat is set to PARQUET, indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn enum_as_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enum_as_string", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Load_TimePartitioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Load_TimePartitioning {
    #[doc =
        "Set the field `expiration_ms`.\nNumber of milliseconds for which to keep the storage for a partition. A wrapper is used here because 0 is an invalid value."]
    pub fn set_expiration_ms(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field`.\nIf not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field.\nThe field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.\nA wrapper is used here because an empty string is an invalid value."]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Load_TimePartitioning {
    type O = terrars::BlockAssignable<BigqueryJob_Load_TimePartitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Load_TimePartitioning {
    #[doc =
        "The only type supported is DAY, which will generate one partition per day. Providing an empty string used to cause an error,\nbut in OnePlatform the field will be treated as unset."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Load_TimePartitioning {
    pub fn build(self) -> BigqueryJob_Load_TimePartitioning {
        BigqueryJob_Load_TimePartitioning {
            expiration_ms: core::default::Default::default(),
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Load_TimePartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Load_TimePartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Load_TimePartitioningRef {
        BigqueryJob_Load_TimePartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Load_TimePartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expiration_ms` after provisioning.\nNumber of milliseconds for which to keep the storage for a partition. A wrapper is used here because 0 is an invalid value."]
    pub fn expiration_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nIf not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field.\nThe field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.\nA wrapper is used here because an empty string is an invalid value."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe only type supported is DAY, which will generate one partition per day. Providing an empty string used to cause an error,\nbut in OnePlatform the field will be treated as unset."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryJob_LoadDynamic {
    destination_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    destination_table: std::option::Option<terrars::DynamicBlock<BigqueryJob_Copy_DestinationTable>>,
    parquet_options: std::option::Option<terrars::DynamicBlock<BigqueryJob_Load_ParquetOptions>>,
    time_partitioning: std::option::Option<terrars::DynamicBlock<BigqueryJob_Load_TimePartitioning>>,
}

#[derive(Serialize)]
pub struct BigqueryJob_Load {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_jagged_rows: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_quoted_newlines: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodetect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_values: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_extension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bad_records: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    null_marker: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quote: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_update_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_leading_rows: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
    source_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_table: std::option::Option<std::vec::Vec<BigqueryJob_Copy_DestinationTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_options: std::option::Option<std::vec::Vec<BigqueryJob_Load_ParquetOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_partitioning: std::option::Option<std::vec::Vec<BigqueryJob_Load_TimePartitioning>>,
    dynamic: BigqueryJob_LoadDynamic,
}

impl BigqueryJob_Load {
    #[doc =
        "Set the field `allow_jagged_rows`.\nAccept rows that are missing trailing optional columns. The missing values are treated as nulls.\nIf false, records with missing trailing columns are treated as bad records, and if there are too many bad records,\nan invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
    pub fn set_allow_jagged_rows(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_jagged_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_quoted_newlines`.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file.\nThe default value is false."]
    pub fn set_allow_quoted_newlines(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_quoted_newlines = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `autodetect`.\nIndicates if we should automatically infer the options and schema for CSV and JSON sources."]
    pub fn set_autodetect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.autodetect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `create_disposition`.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn set_create_disposition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encoding`.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1.\nThe default value is UTF-8. BigQuery decodes the data after the raw, binary data\nhas been split using the values of the quote and fieldDelimiter properties."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_delimiter`.\nThe separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character.\nTo use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts\nthe string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the\ndata in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator.\nThe default value is a comma (',')."]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_unknown_values`.\nIndicates if BigQuery should allow extra values that are not represented in the table schema.\nIf true, the extra values are ignored. If false, records with extra columns are treated as bad records,\nand if there are too many bad records, an invalid error is returned in the job result.\nThe default value is false. The sourceFormat property determines what BigQuery treats as an extra value:\nCSV: Trailing columns\nJSON: Named values that don't match any column names"]
    pub fn set_ignore_unknown_values(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_values = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `json_extension`.\nIf sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON.\nFor a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited\nGeoJSON: set to GEOJSON."]
    pub fn set_json_extension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_extension = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_bad_records`.\nThe maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value,\nan invalid error is returned in the job result. The default value is 0, which requires that all records are valid."]
    pub fn set_max_bad_records(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bad_records = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `null_marker`.\nSpecifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value\nwhen loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an\nempty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as\nan empty value."]
    pub fn set_null_marker(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.null_marker = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `projection_fields`.\nIf sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup.\nProperty names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties.\nIf any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
    pub fn set_projection_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.projection_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quote`.\nThe value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding,\nand then uses the first byte of the encoded string to split the data in its raw, binary state.\nThe default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string.\nIf your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
    pub fn set_quote(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quote = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema_update_options`.\nAllows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or\nsupplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators.\nFor normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn set_schema_update_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.schema_update_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_leading_rows`.\nThe number of rows at the top of a CSV file that BigQuery will skip when loading the data.\nThe default value is 0. This property is useful if you have header rows in the file that should be skipped.\nWhen autodetect is on, the behavior is the following:\nskipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected,\nthe row is read as data. Otherwise data is read starting from the second row.\nskipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row.\nskipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected,\nrow N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
    pub fn set_skip_leading_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.skip_leading_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_format`.\nThe format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\".\nFor newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\".\nFor orc, specify \"ORC\". [Beta] For Bigtable, specify \"BIGTABLE\".\nThe default value is CSV."]
    pub fn set_source_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_disposition`.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn set_write_disposition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.write_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_encryption_configuration`.\n"]
    pub fn set_destination_encryption_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_DestinationTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Load_ParquetOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parquet_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parquet_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_partitioning`.\n"]
    pub fn set_time_partitioning(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Load_TimePartitioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_partitioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_partitioning = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Load {
    type O = terrars::BlockAssignable<BigqueryJob_Load>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Load {
    #[doc =
        "The fully-qualified URIs that point to your data in Google Cloud.\nFor Google Cloud Storage URIs: Each URI can contain one '\\*' wildcard character\nand it must come after the 'bucket' name. Size limits related to load jobs apply\nto external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be\nspecified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table.\nFor Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '\\*' wildcard character is not allowed."]
    pub source_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBigqueryJob_Load {
    pub fn build(self) -> BigqueryJob_Load {
        BigqueryJob_Load {
            allow_jagged_rows: core::default::Default::default(),
            allow_quoted_newlines: core::default::Default::default(),
            autodetect: core::default::Default::default(),
            create_disposition: core::default::Default::default(),
            encoding: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            ignore_unknown_values: core::default::Default::default(),
            json_extension: core::default::Default::default(),
            max_bad_records: core::default::Default::default(),
            null_marker: core::default::Default::default(),
            projection_fields: core::default::Default::default(),
            quote: core::default::Default::default(),
            schema_update_options: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
            source_format: core::default::Default::default(),
            source_uris: self.source_uris,
            write_disposition: core::default::Default::default(),
            destination_encryption_configuration: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            parquet_options: core::default::Default::default(),
            time_partitioning: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_LoadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_LoadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_LoadRef {
        BigqueryJob_LoadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_LoadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_jagged_rows` after provisioning.\nAccept rows that are missing trailing optional columns. The missing values are treated as nulls.\nIf false, records with missing trailing columns are treated as bad records, and if there are too many bad records,\nan invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats."]
    pub fn allow_jagged_rows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_jagged_rows", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_quoted_newlines` after provisioning.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file.\nThe default value is false."]
    pub fn allow_quoted_newlines(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_quoted_newlines", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autodetect` after provisioning.\nIndicates if we should automatically infer the options and schema for CSV and JSON sources."]
    pub fn autodetect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodetect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `create_disposition` after provisioning.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn create_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_disposition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1.\nThe default value is UTF-8. BigQuery decodes the data after the raw, binary data\nhas been split using the values of the quote and fieldDelimiter properties."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_delimiter` after provisioning.\nThe separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character.\nTo use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts\nthe string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the\ndata in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator.\nThe default value is a comma (',')."]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_unknown_values` after provisioning.\nIndicates if BigQuery should allow extra values that are not represented in the table schema.\nIf true, the extra values are ignored. If false, records with extra columns are treated as bad records,\nand if there are too many bad records, an invalid error is returned in the job result.\nThe default value is false. The sourceFormat property determines what BigQuery treats as an extra value:\nCSV: Trailing columns\nJSON: Named values that don't match any column names"]
    pub fn ignore_unknown_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `json_extension` after provisioning.\nIf sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON.\nFor a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited\nGeoJSON: set to GEOJSON."]
    pub fn json_extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_extension", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_bad_records` after provisioning.\nThe maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value,\nan invalid error is returned in the job result. The default value is 0, which requires that all records are valid."]
    pub fn max_bad_records(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bad_records", self.base))
    }

    #[doc =
        "Get a reference to the value of field `null_marker` after provisioning.\nSpecifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value\nwhen loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an\nempty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as\nan empty value."]
    pub fn null_marker(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.null_marker", self.base))
    }

    #[doc =
        "Get a reference to the value of field `projection_fields` after provisioning.\nIf sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup.\nProperty names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties.\nIf any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result."]
    pub fn projection_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.projection_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `quote` after provisioning.\nThe value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding,\nand then uses the first byte of the encoded string to split the data in its raw, binary state.\nThe default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string.\nIf your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true."]
    pub fn quote(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_update_options` after provisioning.\nAllows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or\nsupplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators.\nFor normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn schema_update_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_update_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip_leading_rows` after provisioning.\nThe number of rows at the top of a CSV file that BigQuery will skip when loading the data.\nThe default value is 0. This property is useful if you have header rows in the file that should be skipped.\nWhen autodetect is on, the behavior is the following:\nskipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected,\nthe row is read as data. Otherwise data is read starting from the second row.\nskipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row.\nskipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected,\nrow N is just skipped. Otherwise row N is used to extract column names for the detected schema."]
    pub fn skip_leading_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_format` after provisioning.\nThe format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\".\nFor newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\".\nFor orc, specify \"ORC\". [Beta] For Bigtable, specify \"BIGTABLE\".\nThe default value is CSV."]
    pub fn source_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_uris` after provisioning.\nThe fully-qualified URIs that point to your data in Google Cloud.\nFor Google Cloud Storage URIs: Each URI can contain one '\\*' wildcard character\nand it must come after the 'bucket' name. Size limits related to load jobs apply\nto external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be\nspecified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table.\nFor Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '\\*' wildcard character is not allowed."]
    pub fn source_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_disposition` after provisioning.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn write_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_disposition", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_encryption_configuration` after provisioning.\n"]
    pub fn destination_encryption_configuration(
        &self,
    ) -> terrars::ListRef<BigqueryJob_Copy_DestinationEncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> terrars::ListRef<BigqueryJob_Copy_DestinationTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_table", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> terrars::ListRef<BigqueryJob_Load_ParquetOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_options", self.base))
    }

    #[doc = "Get a reference to the value of field `time_partitioning` after provisioning.\n"]
    pub fn time_partitioning(&self) -> terrars::ListRef<BigqueryJob_Load_TimePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Query_ConnectionProperties {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BigqueryJob_Query_ConnectionProperties { }

impl terrars::ToListMappable for BigqueryJob_Query_ConnectionProperties {
    type O = terrars::BlockAssignable<BigqueryJob_Query_ConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Query_ConnectionProperties {
    #[doc =
        "The key of the property to set. Currently supported connection properties:\n* 'dataset_project_id': represents the default project for datasets that are used in the query\n* 'time_zone': represents the default timezone used to run the query\n* 'session_id': associates the query with a given session\n* 'query_label': associates the query with a given job label\n* 'service_account': indicates the service account to use to run a continuous query"]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The value of the property to set."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Query_ConnectionProperties {
    pub fn build(self) -> BigqueryJob_Query_ConnectionProperties {
        BigqueryJob_Query_ConnectionProperties {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Query_ConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Query_ConnectionPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Query_ConnectionPropertiesRef {
        BigqueryJob_Query_ConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Query_ConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThe key of the property to set. Currently supported connection properties:\n* 'dataset_project_id': represents the default project for datasets that are used in the query\n* 'time_zone': represents the default timezone used to run the query\n* 'session_id': associates the query with a given session\n* 'query_label': associates the query with a given job label\n* 'service_account': indicates the service account to use to run a continuous query"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the property to set."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Query_DefaultDataset {
    dataset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Query_DefaultDataset {
    #[doc = "Set the field `project_id`.\nThe ID of the project containing this table."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Query_DefaultDataset {
    type O = terrars::BlockAssignable<BigqueryJob_Query_DefaultDataset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Query_DefaultDataset {
    #[doc =
        "The dataset. Can be specified '{{dataset_id}}' if 'project_id' is also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}' if not."]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Query_DefaultDataset {
    pub fn build(self) -> BigqueryJob_Query_DefaultDataset {
        BigqueryJob_Query_DefaultDataset {
            dataset_id: self.dataset_id,
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Query_DefaultDatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Query_DefaultDatasetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Query_DefaultDatasetRef {
        BigqueryJob_Query_DefaultDatasetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Query_DefaultDatasetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset. Can be specified '{{dataset_id}}' if 'project_id' is also set,\nor of the form 'projects/{{project}}/datasets/{{dataset_id}}' if not."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Query_ScriptOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_result_statement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_byte_budget: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_timeout_ms: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Query_ScriptOptions {
    #[doc =
        "Set the field `key_result_statement`.\nDetermines which statement in the script represents the \"key result\",\nused to populate the schema and query results of the script job. Possible values: [\"LAST\", \"FIRST_SELECT\"]"]
    pub fn set_key_result_statement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_result_statement = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `statement_byte_budget`.\nLimit on the number of bytes billed per statement. Exceeding this budget results in an error."]
    pub fn set_statement_byte_budget(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.statement_byte_budget = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement_timeout_ms`.\nTimeout period for each statement in a script."]
    pub fn set_statement_timeout_ms(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.statement_timeout_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Query_ScriptOptions {
    type O = terrars::BlockAssignable<BigqueryJob_Query_ScriptOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Query_ScriptOptions {}

impl BuildBigqueryJob_Query_ScriptOptions {
    pub fn build(self) -> BigqueryJob_Query_ScriptOptions {
        BigqueryJob_Query_ScriptOptions {
            key_result_statement: core::default::Default::default(),
            statement_byte_budget: core::default::Default::default(),
            statement_timeout_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Query_ScriptOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Query_ScriptOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_Query_ScriptOptionsRef {
        BigqueryJob_Query_ScriptOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Query_ScriptOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_result_statement` after provisioning.\nDetermines which statement in the script represents the \"key result\",\nused to populate the schema and query results of the script job. Possible values: [\"LAST\", \"FIRST_SELECT\"]"]
    pub fn key_result_statement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_result_statement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `statement_byte_budget` after provisioning.\nLimit on the number of bytes billed per statement. Exceeding this budget results in an error."]
    pub fn statement_byte_budget(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_byte_budget", self.base))
    }

    #[doc =
        "Get a reference to the value of field `statement_timeout_ms` after provisioning.\nTimeout period for each statement in a script."]
    pub fn statement_timeout_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_timeout_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Query_UserDefinedFunctionResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inline_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Query_UserDefinedFunctionResources {
    #[doc =
        "Set the field `inline_code`.\nAn inline resource that contains code for a user-defined function (UDF).\nProviding a inline code resource is equivalent to providing a URI for a file containing the same code."]
    pub fn set_inline_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inline_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_uri`.\nA code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
    pub fn set_resource_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Query_UserDefinedFunctionResources {
    type O = terrars::BlockAssignable<BigqueryJob_Query_UserDefinedFunctionResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Query_UserDefinedFunctionResources {}

impl BuildBigqueryJob_Query_UserDefinedFunctionResources {
    pub fn build(self) -> BigqueryJob_Query_UserDefinedFunctionResources {
        BigqueryJob_Query_UserDefinedFunctionResources {
            inline_code: core::default::Default::default(),
            resource_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_Query_UserDefinedFunctionResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_Query_UserDefinedFunctionResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryJob_Query_UserDefinedFunctionResourcesRef {
        BigqueryJob_Query_UserDefinedFunctionResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_Query_UserDefinedFunctionResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inline_code` after provisioning.\nAn inline resource that contains code for a user-defined function (UDF).\nProviding a inline code resource is equivalent to providing a URI for a file containing the same code."]
    pub fn inline_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inline_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_uri` after provisioning.\nA code resource to load from a Google Cloud Storage URI (gs://bucket/path)."]
    pub fn resource_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryJob_QueryDynamic {
    connection_properties: std::option::Option<terrars::DynamicBlock<BigqueryJob_Query_ConnectionProperties>>,
    default_dataset: std::option::Option<terrars::DynamicBlock<BigqueryJob_Query_DefaultDataset>>,
    destination_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    destination_table: std::option::Option<terrars::DynamicBlock<BigqueryJob_Copy_DestinationTable>>,
    script_options: std::option::Option<terrars::DynamicBlock<BigqueryJob_Query_ScriptOptions>>,
    user_defined_function_resources: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryJob_Query_UserDefinedFunctionResources>>,
}

#[derive(Serialize)]
pub struct BigqueryJob_Query {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_large_results: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flatten_results: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_billing_tier: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_bytes_billed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<std::string::String>>,
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_update_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_legacy_sql: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_query_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_properties: std::option::Option<std::vec::Vec<BigqueryJob_Query_ConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_dataset: std::option::Option<std::vec::Vec<BigqueryJob_Query_DefaultDataset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_table: std::option::Option<std::vec::Vec<BigqueryJob_Copy_DestinationTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_options: std::option::Option<std::vec::Vec<BigqueryJob_Query_ScriptOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_function_resources: std::option::Option<std::vec::Vec<BigqueryJob_Query_UserDefinedFunctionResources>>,
    dynamic: BigqueryJob_QueryDynamic,
}

impl BigqueryJob_Query {
    #[doc =
        "Set the field `allow_large_results`.\nIf true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance.\nRequires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed.\nHowever, you must still set destinationTable when result size exceeds the allowed maximum response size."]
    pub fn set_allow_large_results(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_large_results = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `create_disposition`.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn set_create_disposition(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `flatten_results`.\nIf true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results.\nallowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
    pub fn set_flatten_results(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flatten_results = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maximum_billing_tier`.\nLimits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn set_maximum_billing_tier(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_billing_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maximum_bytes_billed`.\nLimits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn set_maximum_bytes_billed(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.maximum_bytes_billed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameter_mode`.\nStandard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
    pub fn set_parameter_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `priority`.\nSpecifies a priority for the query. Default value: \"INTERACTIVE\" Possible values: [\"INTERACTIVE\", \"BATCH\"]"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema_update_options`.\nAllows the schema of the destination table to be updated as a side effect of the query job.\nSchema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table,\nspecified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema.\nOne or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn set_schema_update_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.schema_update_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_legacy_sql`.\nSpecifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.\nIf set to false, the query will use BigQuery's standard SQL."]
    pub fn set_use_legacy_sql(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_legacy_sql = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_query_cache`.\nWhether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever\ntables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified.\nThe default value is true."]
    pub fn set_use_query_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_query_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_disposition`.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn set_write_disposition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.write_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_properties`.\n"]
    pub fn set_connection_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Query_ConnectionProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_dataset`.\n"]
    pub fn set_default_dataset(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Query_DefaultDataset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_dataset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_encryption_configuration`.\n"]
    pub fn set_destination_encryption_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_DestinationEncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_table`.\n"]
    pub fn set_destination_table(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Copy_DestinationTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `script_options`.\n"]
    pub fn set_script_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Query_ScriptOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.script_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.script_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_defined_function_resources`.\n"]
    pub fn set_user_defined_function_resources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryJob_Query_UserDefinedFunctionResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_defined_function_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_function_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryJob_Query {
    type O = terrars::BlockAssignable<BigqueryJob_Query>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Query {
    #[doc =
        "SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.\n*NOTE*: queries containing [DML language](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language)\n('DELETE', 'UPDATE', 'MERGE', 'INSERT') must specify 'create_disposition = \"\"' and 'write_disposition = \"\"'."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryJob_Query {
    pub fn build(self) -> BigqueryJob_Query {
        BigqueryJob_Query {
            allow_large_results: core::default::Default::default(),
            create_disposition: core::default::Default::default(),
            flatten_results: core::default::Default::default(),
            maximum_billing_tier: core::default::Default::default(),
            maximum_bytes_billed: core::default::Default::default(),
            parameter_mode: core::default::Default::default(),
            priority: core::default::Default::default(),
            query: self.query,
            schema_update_options: core::default::Default::default(),
            use_legacy_sql: core::default::Default::default(),
            use_query_cache: core::default::Default::default(),
            write_disposition: core::default::Default::default(),
            connection_properties: core::default::Default::default(),
            default_dataset: core::default::Default::default(),
            destination_encryption_configuration: core::default::Default::default(),
            destination_table: core::default::Default::default(),
            script_options: core::default::Default::default(),
            user_defined_function_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_QueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_QueryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_QueryRef {
        BigqueryJob_QueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_QueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_large_results` after provisioning.\nIf true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance.\nRequires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed.\nHowever, you must still set destinationTable when result size exceeds the allowed maximum response size."]
    pub fn allow_large_results(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_large_results", self.base))
    }

    #[doc =
        "Get a reference to the value of field `create_disposition` after provisioning.\nSpecifies whether the job is allowed to create new tables. The following values are supported:\nCREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.\nCREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.\nCreation, truncation and append actions occur as one atomic update upon job completion Default value: \"CREATE_IF_NEEDED\" Possible values: [\"CREATE_IF_NEEDED\", \"CREATE_NEVER\"]"]
    pub fn create_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_disposition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flatten_results` after provisioning.\nIf true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results.\nallowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened."]
    pub fn flatten_results(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten_results", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maximum_billing_tier` after provisioning.\nLimits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn maximum_billing_tier(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_billing_tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maximum_bytes_billed` after provisioning.\nLimits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge).\nIf unspecified, this will be set to your project default."]
    pub fn maximum_bytes_billed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_bytes_billed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameter_mode` after provisioning.\nStandard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query."]
    pub fn parameter_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nSpecifies a priority for the query. Default value: \"INTERACTIVE\" Possible values: [\"INTERACTIVE\", \"BATCH\"]"]
    pub fn priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query` after provisioning.\nSQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.\n*NOTE*: queries containing [DML language](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language)\n('DELETE', 'UPDATE', 'MERGE', 'INSERT') must specify 'create_disposition = \"\"' and 'write_disposition = \"\"'."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_update_options` after provisioning.\nAllows the schema of the destination table to be updated as a side effect of the query job.\nSchema update options are supported in two cases: when writeDisposition is WRITE_APPEND;\nwhen writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table,\nspecified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema.\nOne or more of the following values are specified:\nALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.\nALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable."]
    pub fn schema_update_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_update_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_legacy_sql` after provisioning.\nSpecifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true.\nIf set to false, the query will use BigQuery's standard SQL."]
    pub fn use_legacy_sql(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_legacy_sql", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_query_cache` after provisioning.\nWhether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever\ntables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified.\nThe default value is true."]
    pub fn use_query_cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_query_cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_disposition` after provisioning.\nSpecifies the action that occurs if the destination table already exists. The following values are supported:\nWRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.\nWRITE_APPEND: If the table already exists, BigQuery appends the data to the table.\nWRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.\nEach action is atomic and only occurs if BigQuery is able to complete the job successfully.\nCreation, truncation and append actions occur as one atomic update upon job completion. Default value: \"WRITE_EMPTY\" Possible values: [\"WRITE_TRUNCATE\", \"WRITE_APPEND\", \"WRITE_EMPTY\"]"]
    pub fn write_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_disposition", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_properties` after provisioning.\n"]
    pub fn connection_properties(&self) -> terrars::ListRef<BigqueryJob_Query_ConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `default_dataset` after provisioning.\n"]
    pub fn default_dataset(&self) -> terrars::ListRef<BigqueryJob_Query_DefaultDatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_encryption_configuration` after provisioning.\n"]
    pub fn destination_encryption_configuration(
        &self,
    ) -> terrars::ListRef<BigqueryJob_Copy_DestinationEncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_table` after provisioning.\n"]
    pub fn destination_table(&self) -> terrars::ListRef<BigqueryJob_Copy_DestinationTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_table", self.base))
    }

    #[doc = "Get a reference to the value of field `script_options` after provisioning.\n"]
    pub fn script_options(&self) -> terrars::ListRef<BigqueryJob_Query_ScriptOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.script_options", self.base))
    }

    #[doc = "Get a reference to the value of field `user_defined_function_resources` after provisioning.\n"]
    pub fn user_defined_function_resources(&self) -> terrars::ListRef<BigqueryJob_Query_UserDefinedFunctionResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_function_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryJob_Timeouts {
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

impl terrars::ToListMappable for BigqueryJob_Timeouts {
    type O = terrars::BlockAssignable<BigqueryJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryJob_Timeouts {}

impl BuildBigqueryJob_Timeouts {
    pub fn build(self) -> BigqueryJob_Timeouts {
        BigqueryJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryJob_TimeoutsRef {
        BigqueryJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryJob_TimeoutsRef {
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
