use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudSchedulerJobData {
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
    attempt_deadline: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paused: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine_http_target: std::option::Option<std::vec::Vec<CloudSchedulerJobAppEngineHttpTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_target: std::option::Option<std::vec::Vec<CloudSchedulerJobHttpTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_target: std::option::Option<std::vec::Vec<CloudSchedulerJobPubsubTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_config: std::option::Option<std::vec::Vec<CloudSchedulerJobRetryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudSchedulerJobTimeoutsEl>,
    dynamic: CloudSchedulerJobDynamic,
}

struct CloudSchedulerJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudSchedulerJobData>,
}

#[derive(Clone)]
pub struct CloudSchedulerJob(std::rc::Rc<CloudSchedulerJob_>);

impl CloudSchedulerJob {
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
        "Set the field `attempt_deadline`.\nThe deadline for job attempts. If the request handler does not respond by this deadline then the request is\ncancelled and the attempt is marked as a DEADLINE_EXCEEDED failure. The failed attempt can be viewed in\nexecution logs. Cloud Scheduler will retry the job according to the RetryConfig.\nThe allowed duration for this deadline is:\n* For HTTP targets, between 15 seconds and 30 minutes.\n* For App Engine HTTP targets, between 15 seconds and 24 hours.\n* **Note**: For PubSub targets, this field is ignored - setting it will introduce an unresolvable diff.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn set_attempt_deadline(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().attempt_deadline = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nA human-readable description for the job.\nThis string must not contain more than 500 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `paused`.\nSets the job to a paused state. Jobs default to being enabled when this property is not set."]
    pub fn set_paused(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().paused = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where the scheduler job resides. If it is not provided, Terraform will use the provider default."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\nDescribes the schedule on which the job will be executed."]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nSpecifies the time zone to be used in interpreting schedule.\nThe value of this field must be a time zone name from the tz database."]
    pub fn set_time_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_engine_http_target`.\n"]
    pub fn set_app_engine_http_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSchedulerJobAppEngineHttpTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_engine_http_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_engine_http_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_target`.\n"]
    pub fn set_http_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSchedulerJobHttpTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pubsub_target`.\n"]
    pub fn set_pubsub_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSchedulerJobPubsubTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pubsub_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pubsub_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_config`.\n"]
    pub fn set_retry_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSchedulerJobRetryConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudSchedulerJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `attempt_deadline` after provisioning.\nThe deadline for job attempts. If the request handler does not respond by this deadline then the request is\ncancelled and the attempt is marked as a DEADLINE_EXCEEDED failure. The failed attempt can be viewed in\nexecution logs. Cloud Scheduler will retry the job according to the RetryConfig.\nThe allowed duration for this deadline is:\n* For HTTP targets, between 15 seconds and 30 minutes.\n* For App Engine HTTP targets, between 15 seconds and 24 hours.\n* **Note**: For PubSub targets, this field is ignored - setting it will introduce an unresolvable diff.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn attempt_deadline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_deadline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the job.\nThis string must not contain more than 500 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nSets the job to a paused state. Jobs default to being enabled when this property is not set."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the scheduler job resides. If it is not provided, Terraform will use the provider default."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nDescribes the schedule on which the job will be executed."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the job."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nSpecifies the time zone to be used in interpreting schedule.\nThe value of this field must be a time zone name from the tz database."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine_http_target` after provisioning.\n"]
    pub fn app_engine_http_target(&self) -> terrars::ListRef<CloudSchedulerJobAppEngineHttpTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudSchedulerJobHttpTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pubsub_target` after provisioning.\n"]
    pub fn pubsub_target(&self) -> terrars::ListRef<CloudSchedulerJobPubsubTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> terrars::ListRef<CloudSchedulerJobRetryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSchedulerJobTimeoutsElRef {
        CloudSchedulerJobTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudSchedulerJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudSchedulerJob { }

impl terrars::ToListMappable for CloudSchedulerJob {
    type O = terrars::ListRef<CloudSchedulerJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudSchedulerJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_scheduler_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudSchedulerJob {
    pub tf_id: std::string::String,
    #[doc = "The name of the job."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSchedulerJob {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudSchedulerJob {
        let out = CloudSchedulerJob(std::rc::Rc::new(CloudSchedulerJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudSchedulerJobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                attempt_deadline: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                paused: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                schedule: core::default::Default::default(),
                time_zone: core::default::Default::default(),
                app_engine_http_target: core::default::Default::default(),
                http_target: core::default::Default::default(),
                pubsub_target: core::default::Default::default(),
                retry_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudSchedulerJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudSchedulerJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attempt_deadline` after provisioning.\nThe deadline for job attempts. If the request handler does not respond by this deadline then the request is\ncancelled and the attempt is marked as a DEADLINE_EXCEEDED failure. The failed attempt can be viewed in\nexecution logs. Cloud Scheduler will retry the job according to the RetryConfig.\nThe allowed duration for this deadline is:\n* For HTTP targets, between 15 seconds and 30 minutes.\n* For App Engine HTTP targets, between 15 seconds and 24 hours.\n* **Note**: For PubSub targets, this field is ignored - setting it will introduce an unresolvable diff.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn attempt_deadline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_deadline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description for the job.\nThis string must not contain more than 500 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nSets the job to a paused state. Jobs default to being enabled when this property is not set."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where the scheduler job resides. If it is not provided, Terraform will use the provider default."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nDescribes the schedule on which the job will be executed."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the job."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nSpecifies the time zone to be used in interpreting schedule.\nThe value of this field must be a time zone name from the tz database."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine_http_target` after provisioning.\n"]
    pub fn app_engine_http_target(&self) -> terrars::ListRef<CloudSchedulerJobAppEngineHttpTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudSchedulerJobHttpTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pubsub_target` after provisioning.\n"]
    pub fn pubsub_target(&self) -> terrars::ListRef<CloudSchedulerJobPubsubTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> terrars::ListRef<CloudSchedulerJobRetryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudSchedulerJobTimeoutsElRef {
        CloudSchedulerJobTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {
    #[doc =
        "Set the field `instance`.\nApp instance.\nBy default, the job is sent to an instance which is available when the job is attempted."]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nApp service.\nBy default, the job is sent to the service which is the default service when the job is attempted."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nApp version.\nBy default, the job is sent to the version which is the default version when the job is attempted."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {}

impl BuildCloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {
    pub fn build(self) -> CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {
        CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl {
            instance: core::default::Default::default(),
            service: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingElRef {
        CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nApp instance.\nBy default, the job is sent to an instance which is available when the job is attempted."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nApp service.\nBy default, the job is sent to the service which is the default service when the job is attempted."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nApp version.\nBy default, the job is sent to the version which is the default version when the job is attempted."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSchedulerJobAppEngineHttpTargetElDynamic {
    app_engine_routing: std
    ::option
    ::Option<terrars::DynamicBlock<CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl>>,
}

#[derive(Serialize)]
pub struct CloudSchedulerJobAppEngineHttpTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_method: std::option::Option<terrars::PrimField<std::string::String>>,
    relative_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine_routing: std::option::Option<std::vec::Vec<CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl>>,
    dynamic: CloudSchedulerJobAppEngineHttpTargetElDynamic,
}

impl CloudSchedulerJobAppEngineHttpTargetEl {
    #[doc =
        "Set the field `body`.\nHTTP request body.\nA request body is allowed only if the HTTP method is POST or PUT.\nIt will result in invalid argument error to set a body on a job with an incompatible HttpMethod.\n\nA base64-encoded string."]
    pub fn set_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.body = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `headers`.\nHTTP request headers.\nThis map contains the header field names and values.\nHeaders can be set when the job is created."]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_method`.\nWhich HTTP method to use for the request."]
    pub fn set_http_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_engine_routing`.\n"]
    pub fn set_app_engine_routing(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_engine_routing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_engine_routing = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobAppEngineHttpTargetEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobAppEngineHttpTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobAppEngineHttpTargetEl {
    #[doc =
        "The relative URI.\nThe relative URL must begin with \"/\" and must be a valid HTTP relative URL.\nIt can contain a path, query string arguments, and \\# fragments.\nIf the relative URL is empty, then the root path \"/\" will be used.\nNo spaces are allowed, and the maximum length allowed is 2083 characters"]
    pub relative_uri: terrars::PrimField<std::string::String>,
}

impl BuildCloudSchedulerJobAppEngineHttpTargetEl {
    pub fn build(self) -> CloudSchedulerJobAppEngineHttpTargetEl {
        CloudSchedulerJobAppEngineHttpTargetEl {
            body: core::default::Default::default(),
            headers: core::default::Default::default(),
            http_method: core::default::Default::default(),
            relative_uri: self.relative_uri,
            app_engine_routing: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudSchedulerJobAppEngineHttpTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobAppEngineHttpTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobAppEngineHttpTargetElRef {
        CloudSchedulerJobAppEngineHttpTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobAppEngineHttpTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `body` after provisioning.\nHTTP request body.\nA request body is allowed only if the HTTP method is POST or PUT.\nIt will result in invalid argument error to set a body on a job with an incompatible HttpMethod.\n\nA base64-encoded string."]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nHTTP request headers.\nThis map contains the header field names and values.\nHeaders can be set when the job is created."]
    pub fn headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_method` after provisioning.\nWhich HTTP method to use for the request."]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `relative_uri` after provisioning.\nThe relative URI.\nThe relative URL must begin with \"/\" and must be a valid HTTP relative URL.\nIt can contain a path, query string arguments, and \\# fragments.\nIf the relative URL is empty, then the root path \"/\" will be used.\nNo spaces are allowed, and the maximum length allowed is 2083 characters"]
    pub fn relative_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relative_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `app_engine_routing` after provisioning.\n"]
    pub fn app_engine_routing(&self) -> terrars::ListRef<CloudSchedulerJobAppEngineHttpTargetElAppEngineRoutingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_routing", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSchedulerJobHttpTargetElOauthTokenEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account_email: terrars::PrimField<std::string::String>,
}

impl CloudSchedulerJobHttpTargetElOauthTokenEl {
    #[doc =
        "Set the field `scope`.\nOAuth scope to be used for generating OAuth access token. If not specified,\n\"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobHttpTargetElOauthTokenEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobHttpTargetElOauthTokenEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobHttpTargetElOauthTokenEl {
    #[doc =
        "Service account email to be used for generating OAuth token.\nThe service account must be within the same project as the job."]
    pub service_account_email: terrars::PrimField<std::string::String>,
}

impl BuildCloudSchedulerJobHttpTargetElOauthTokenEl {
    pub fn build(self) -> CloudSchedulerJobHttpTargetElOauthTokenEl {
        CloudSchedulerJobHttpTargetElOauthTokenEl {
            scope: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}

pub struct CloudSchedulerJobHttpTargetElOauthTokenElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobHttpTargetElOauthTokenElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobHttpTargetElOauthTokenElRef {
        CloudSchedulerJobHttpTargetElOauthTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobHttpTargetElOauthTokenElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nOAuth scope to be used for generating OAuth access token. If not specified,\n\"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating OAuth token.\nThe service account must be within the same project as the job."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSchedulerJobHttpTargetElOidcTokenEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account_email: terrars::PrimField<std::string::String>,
}

impl CloudSchedulerJobHttpTargetElOidcTokenEl {
    #[doc =
        "Set the field `audience`.\nAudience to be used when generating OIDC token. If not specified,\nthe URI specified in target will be used."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobHttpTargetElOidcTokenEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobHttpTargetElOidcTokenEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobHttpTargetElOidcTokenEl {
    #[doc =
        "Service account email to be used for generating OAuth token.\nThe service account must be within the same project as the job."]
    pub service_account_email: terrars::PrimField<std::string::String>,
}

impl BuildCloudSchedulerJobHttpTargetElOidcTokenEl {
    pub fn build(self) -> CloudSchedulerJobHttpTargetElOidcTokenEl {
        CloudSchedulerJobHttpTargetElOidcTokenEl {
            audience: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}

pub struct CloudSchedulerJobHttpTargetElOidcTokenElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobHttpTargetElOidcTokenElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobHttpTargetElOidcTokenElRef {
        CloudSchedulerJobHttpTargetElOidcTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobHttpTargetElOidcTokenElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. If not specified,\nthe URI specified in target will be used."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating OAuth token.\nThe service account must be within the same project as the job."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudSchedulerJobHttpTargetElDynamic {
    oauth_token: std::option::Option<terrars::DynamicBlock<CloudSchedulerJobHttpTargetElOauthTokenEl>>,
    oidc_token: std::option::Option<terrars::DynamicBlock<CloudSchedulerJobHttpTargetElOidcTokenEl>>,
}

#[derive(Serialize)]
pub struct CloudSchedulerJobHttpTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_method: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token: std::option::Option<std::vec::Vec<CloudSchedulerJobHttpTargetElOauthTokenEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std::option::Option<std::vec::Vec<CloudSchedulerJobHttpTargetElOidcTokenEl>>,
    dynamic: CloudSchedulerJobHttpTargetElDynamic,
}

impl CloudSchedulerJobHttpTargetEl {
    #[doc =
        "Set the field `body`.\nHTTP request body.\nA request body is allowed only if the HTTP method is POST, PUT, or PATCH.\nIt is an error to set body on a job with an incompatible HttpMethod.\n\nA base64-encoded string."]
    pub fn set_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.body = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `headers`.\nThis map contains the header field names and values.\nRepeated headers are not supported, but a header value can contain commas."]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_method`.\nWhich HTTP method to use for the request."]
    pub fn set_http_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSchedulerJobHttpTargetElOauthTokenEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_token = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudSchedulerJobHttpTargetElOidcTokenEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oidc_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobHttpTargetEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobHttpTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobHttpTargetEl {
    #[doc = "The full URI path that the request will be sent to."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildCloudSchedulerJobHttpTargetEl {
    pub fn build(self) -> CloudSchedulerJobHttpTargetEl {
        CloudSchedulerJobHttpTargetEl {
            body: core::default::Default::default(),
            headers: core::default::Default::default(),
            http_method: core::default::Default::default(),
            uri: self.uri,
            oauth_token: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudSchedulerJobHttpTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobHttpTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobHttpTargetElRef {
        CloudSchedulerJobHttpTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobHttpTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `body` after provisioning.\nHTTP request body.\nA request body is allowed only if the HTTP method is POST, PUT, or PATCH.\nIt is an error to set body on a job with an incompatible HttpMethod.\n\nA base64-encoded string."]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nThis map contains the header field names and values.\nRepeated headers are not supported, but a header value can contain commas."]
    pub fn headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_method` after provisioning.\nWhich HTTP method to use for the request."]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe full URI path that the request will be sent to."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> terrars::ListRef<CloudSchedulerJobHttpTargetElOauthTokenElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> terrars::ListRef<CloudSchedulerJobHttpTargetElOidcTokenElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSchedulerJobPubsubTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    topic_name: terrars::PrimField<std::string::String>,
}

impl CloudSchedulerJobPubsubTargetEl {
    #[doc =
        "Set the field `attributes`.\nAttributes for PubsubMessage.\nPubsub message must contain either non-empty data, or at least one attribute."]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data`.\nThe message payload for PubsubMessage.\nPubsub message must contain either non-empty data, or at least one attribute.\n\n A base64-encoded string."]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobPubsubTargetEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobPubsubTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobPubsubTargetEl {
    #[doc =
        "The full resource name for the Cloud Pub/Sub topic to which\nmessages will be published when a job is delivered. ~>**NOTE:**\nThe topic name must be in the same format as required by PubSub's\nPublishRequest.name, e.g. 'projects/my-project/topics/my-topic'."]
    pub topic_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudSchedulerJobPubsubTargetEl {
    pub fn build(self) -> CloudSchedulerJobPubsubTargetEl {
        CloudSchedulerJobPubsubTargetEl {
            attributes: core::default::Default::default(),
            data: core::default::Default::default(),
            topic_name: self.topic_name,
        }
    }
}

pub struct CloudSchedulerJobPubsubTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobPubsubTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobPubsubTargetElRef {
        CloudSchedulerJobPubsubTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobPubsubTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes` after provisioning.\nAttributes for PubsubMessage.\nPubsub message must contain either non-empty data, or at least one attribute."]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nThe message payload for PubsubMessage.\nPubsub message must contain either non-empty data, or at least one attribute.\n\n A base64-encoded string."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic_name` after provisioning.\nThe full resource name for the Cloud Pub/Sub topic to which\nmessages will be published when a job is delivered. ~>**NOTE:**\nThe topic name must be in the same format as required by PubSub's\nPublishRequest.name, e.g. 'projects/my-project/topics/my-topic'."]
    pub fn topic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSchedulerJobRetryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_backoff_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_doublings: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retry_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_backoff_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_count: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudSchedulerJobRetryConfigEl {
    #[doc =
        "Set the field `max_backoff_duration`.\nThe maximum amount of time to wait before retrying a job after it fails.\nA duration in seconds with up to nine fractional digits, terminated by 's'."]
    pub fn set_max_backoff_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_backoff_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_doublings`.\nThe time between retries will double maxDoublings times.\nA job's retry interval starts at minBackoffDuration,\nthen doubles maxDoublings times, then increases linearly,\nand finally retries retries at intervals of maxBackoffDuration up to retryCount times."]
    pub fn set_max_doublings(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_doublings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retry_duration`.\nThe time limit for retrying a failed job, measured from time when an execution was first attempted.\nIf specified with retryCount, the job will be retried until both limits are reached.\nA duration in seconds with up to nine fractional digits, terminated by 's'."]
    pub fn set_max_retry_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_backoff_duration`.\nThe minimum amount of time to wait before retrying a job after it fails.\nA duration in seconds with up to nine fractional digits, terminated by 's'."]
    pub fn set_min_backoff_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.min_backoff_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_count`.\nThe number of attempts that the system will make to run a\njob using the exponential backoff procedure described by maxDoublings.\nValues greater than 5 and negative values are not allowed."]
    pub fn set_retry_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudSchedulerJobRetryConfigEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobRetryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobRetryConfigEl {}

impl BuildCloudSchedulerJobRetryConfigEl {
    pub fn build(self) -> CloudSchedulerJobRetryConfigEl {
        CloudSchedulerJobRetryConfigEl {
            max_backoff_duration: core::default::Default::default(),
            max_doublings: core::default::Default::default(),
            max_retry_duration: core::default::Default::default(),
            min_backoff_duration: core::default::Default::default(),
            retry_count: core::default::Default::default(),
        }
    }
}

pub struct CloudSchedulerJobRetryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobRetryConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobRetryConfigElRef {
        CloudSchedulerJobRetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobRetryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_backoff_duration` after provisioning.\nThe maximum amount of time to wait before retrying a job after it fails.\nA duration in seconds with up to nine fractional digits, terminated by 's'."]
    pub fn max_backoff_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_backoff_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_doublings` after provisioning.\nThe time between retries will double maxDoublings times.\nA job's retry interval starts at minBackoffDuration,\nthen doubles maxDoublings times, then increases linearly,\nand finally retries retries at intervals of maxBackoffDuration up to retryCount times."]
    pub fn max_doublings(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_doublings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_retry_duration` after provisioning.\nThe time limit for retrying a failed job, measured from time when an execution was first attempted.\nIf specified with retryCount, the job will be retried until both limits are reached.\nA duration in seconds with up to nine fractional digits, terminated by 's'."]
    pub fn max_retry_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retry_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_backoff_duration` after provisioning.\nThe minimum amount of time to wait before retrying a job after it fails.\nA duration in seconds with up to nine fractional digits, terminated by 's'."]
    pub fn min_backoff_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_backoff_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_count` after provisioning.\nThe number of attempts that the system will make to run a\njob using the exponential backoff procedure described by maxDoublings.\nValues greater than 5 and negative values are not allowed."]
    pub fn retry_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudSchedulerJobTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudSchedulerJobTimeoutsEl {
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

impl terrars::ToListMappable for CloudSchedulerJobTimeoutsEl {
    type O = terrars::BlockAssignable<CloudSchedulerJobTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudSchedulerJobTimeoutsEl {}

impl BuildCloudSchedulerJobTimeoutsEl {
    pub fn build(self) -> CloudSchedulerJobTimeoutsEl {
        CloudSchedulerJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudSchedulerJobTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudSchedulerJobTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudSchedulerJobTimeoutsElRef {
        CloudSchedulerJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudSchedulerJobTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct CloudSchedulerJobDynamic {
    app_engine_http_target: std::option::Option<terrars::DynamicBlock<CloudSchedulerJobAppEngineHttpTargetEl>>,
    http_target: std::option::Option<terrars::DynamicBlock<CloudSchedulerJobHttpTargetEl>>,
    pubsub_target: std::option::Option<terrars::DynamicBlock<CloudSchedulerJobPubsubTargetEl>>,
    retry_config: std::option::Option<terrars::DynamicBlock<CloudSchedulerJobRetryConfigEl>>,
}
