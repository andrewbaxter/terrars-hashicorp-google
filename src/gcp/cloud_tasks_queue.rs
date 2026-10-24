use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudTasksQueueData {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine_routing_override: std::option::Option<std::vec::Vec<CloudTasksQueueAppEngineRoutingOverrideEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limits: std::option::Option<std::vec::Vec<CloudTasksQueueRateLimitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_config: std::option::Option<std::vec::Vec<CloudTasksQueueRetryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stackdriver_logging_config: std::option::Option<std::vec::Vec<CloudTasksQueueStackdriverLoggingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudTasksQueueTimeoutsEl>,
    dynamic: CloudTasksQueueDynamic,
}

struct CloudTasksQueue_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudTasksQueueData>,
}

#[derive(Clone)]
pub struct CloudTasksQueue(std::rc::Rc<CloudTasksQueue_>);

impl CloudTasksQueue {
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

    #[doc = "Set the field `name`.\nThe queue name."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_engine_routing_override`.\n"]
    pub fn set_app_engine_routing_override(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueueAppEngineRoutingOverrideEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_engine_routing_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_engine_routing_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rate_limits`.\n"]
    pub fn set_rate_limits(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueueRateLimitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rate_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rate_limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_config`.\n"]
    pub fn set_retry_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueueRetryConfigEl>>,
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

    #[doc = "Set the field `stackdriver_logging_config`.\n"]
    pub fn set_stackdriver_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueueStackdriverLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stackdriver_logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stackdriver_logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudTasksQueueTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the queue"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe queue name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine_routing_override` after provisioning.\n"]
    pub fn app_engine_routing_override(&self) -> terrars::ListRef<CloudTasksQueueAppEngineRoutingOverrideElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_routing_override", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limits` after provisioning.\n"]
    pub fn rate_limits(&self) -> terrars::ListRef<CloudTasksQueueRateLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> terrars::ListRef<CloudTasksQueueRetryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stackdriver_logging_config` after provisioning.\n"]
    pub fn stackdriver_logging_config(&self) -> terrars::ListRef<CloudTasksQueueStackdriverLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stackdriver_logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudTasksQueueTimeoutsElRef {
        CloudTasksQueueTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudTasksQueue {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudTasksQueue { }

impl terrars::ToListMappable for CloudTasksQueue {
    type O = terrars::ListRef<CloudTasksQueueRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudTasksQueue_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_tasks_queue".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudTasksQueue {
    pub tf_id: std::string::String,
    #[doc = "The location of the queue"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCloudTasksQueue {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudTasksQueue {
        let out = CloudTasksQueue(std::rc::Rc::new(CloudTasksQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudTasksQueueData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                app_engine_routing_override: core::default::Default::default(),
                rate_limits: core::default::Default::default(),
                retry_config: core::default::Default::default(),
                stackdriver_logging_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudTasksQueueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueueRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudTasksQueueRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the queue"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe queue name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine_routing_override` after provisioning.\n"]
    pub fn app_engine_routing_override(&self) -> terrars::ListRef<CloudTasksQueueAppEngineRoutingOverrideElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_routing_override", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limits` after provisioning.\n"]
    pub fn rate_limits(&self) -> terrars::ListRef<CloudTasksQueueRateLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> terrars::ListRef<CloudTasksQueueRetryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stackdriver_logging_config` after provisioning.\n"]
    pub fn stackdriver_logging_config(&self) -> terrars::ListRef<CloudTasksQueueStackdriverLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stackdriver_logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudTasksQueueTimeoutsElRef {
        CloudTasksQueueTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueueAppEngineRoutingOverrideEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueueAppEngineRoutingOverrideEl {
    #[doc =
        "Set the field `instance`.\nApp instance.\n\nBy default, the task is sent to an instance which is available when the task is attempted."]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nApp service.\n\nBy default, the task is sent to the service which is the default service when the task is attempted."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nApp version.\n\nBy default, the task is sent to the version which is the default version when the task is attempted."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueueAppEngineRoutingOverrideEl {
    type O = terrars::BlockAssignable<CloudTasksQueueAppEngineRoutingOverrideEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueueAppEngineRoutingOverrideEl {}

impl BuildCloudTasksQueueAppEngineRoutingOverrideEl {
    pub fn build(self) -> CloudTasksQueueAppEngineRoutingOverrideEl {
        CloudTasksQueueAppEngineRoutingOverrideEl {
            instance: core::default::Default::default(),
            service: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct CloudTasksQueueAppEngineRoutingOverrideElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueueAppEngineRoutingOverrideElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueueAppEngineRoutingOverrideElRef {
        CloudTasksQueueAppEngineRoutingOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueueAppEngineRoutingOverrideElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host that the task is sent to."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nApp instance.\n\nBy default, the task is sent to an instance which is available when the task is attempted."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nApp service.\n\nBy default, the task is sent to the service which is the default service when the task is attempted."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nApp version.\n\nBy default, the task is sent to the version which is the default version when the task is attempted."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueueRateLimitsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_dispatches: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_dispatches_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudTasksQueueRateLimitsEl {
    #[doc =
        "Set the field `max_concurrent_dispatches`.\nThe maximum number of concurrent tasks that Cloud Tasks allows to\nbe dispatched for this queue. After this threshold has been\nreached, Cloud Tasks stops dispatching tasks until the number of\nconcurrent requests decreases."]
    pub fn set_max_concurrent_dispatches(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_dispatches = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_dispatches_per_second`.\nThe maximum rate at which tasks are dispatched from this queue.\n\nIf unspecified when the queue is created, Cloud Tasks will pick the default."]
    pub fn set_max_dispatches_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_dispatches_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueueRateLimitsEl {
    type O = terrars::BlockAssignable<CloudTasksQueueRateLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueueRateLimitsEl {}

impl BuildCloudTasksQueueRateLimitsEl {
    pub fn build(self) -> CloudTasksQueueRateLimitsEl {
        CloudTasksQueueRateLimitsEl {
            max_concurrent_dispatches: core::default::Default::default(),
            max_dispatches_per_second: core::default::Default::default(),
        }
    }
}

pub struct CloudTasksQueueRateLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueueRateLimitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueueRateLimitsElRef {
        CloudTasksQueueRateLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueueRateLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_burst_size` after provisioning.\nThe max burst size.\n\nMax burst size limits how fast tasks in queue are processed when many tasks are\nin the queue and the rate is high. This field allows the queue to have a high\nrate so processing starts shortly after a task is enqueued, but still limits\nresource usage when many tasks are enqueued in a short period of time."]
    pub fn max_burst_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_burst_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_dispatches` after provisioning.\nThe maximum number of concurrent tasks that Cloud Tasks allows to\nbe dispatched for this queue. After this threshold has been\nreached, Cloud Tasks stops dispatching tasks until the number of\nconcurrent requests decreases."]
    pub fn max_concurrent_dispatches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_dispatches", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_dispatches_per_second` after provisioning.\nThe maximum rate at which tasks are dispatched from this queue.\n\nIf unspecified when the queue is created, Cloud Tasks will pick the default."]
    pub fn max_dispatches_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_dispatches_per_second", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueueRetryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_doublings: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retry_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueueRetryConfigEl {
    #[doc =
        "Set the field `max_attempts`.\nNumber of attempts per task.\n\nCloud Tasks will attempt the task maxAttempts times (that is, if\nthe first attempt fails, then there will be maxAttempts - 1\nretries). Must be >= -1.\n\nIf unspecified when the queue is created, Cloud Tasks will pick\nthe default.\n\n-1 indicates unlimited attempts."]
    pub fn set_max_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_backoff`.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn set_max_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_backoff = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_doublings`.\nThe time between retries will double maxDoublings times.\n\nA task's retry interval starts at minBackoff, then doubles maxDoublings times,\nthen increases linearly, and finally retries retries at intervals of maxBackoff\nup to maxAttempts times."]
    pub fn set_max_doublings(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_doublings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retry_duration`.\nIf positive, maxRetryDuration specifies the time limit for\nretrying a failed task, measured from when the task was first\nattempted. Once maxRetryDuration time has passed and the task has\nbeen attempted maxAttempts times, no further attempts will be\nmade and the task will be deleted.\n\nIf zero, then the task age is unlimited."]
    pub fn set_max_retry_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_retry_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_backoff`.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn set_min_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_backoff = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueueRetryConfigEl {
    type O = terrars::BlockAssignable<CloudTasksQueueRetryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueueRetryConfigEl {}

impl BuildCloudTasksQueueRetryConfigEl {
    pub fn build(self) -> CloudTasksQueueRetryConfigEl {
        CloudTasksQueueRetryConfigEl {
            max_attempts: core::default::Default::default(),
            max_backoff: core::default::Default::default(),
            max_doublings: core::default::Default::default(),
            max_retry_duration: core::default::Default::default(),
            min_backoff: core::default::Default::default(),
        }
    }
}

pub struct CloudTasksQueueRetryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueueRetryConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueueRetryConfigElRef {
        CloudTasksQueueRetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueueRetryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_attempts` after provisioning.\nNumber of attempts per task.\n\nCloud Tasks will attempt the task maxAttempts times (that is, if\nthe first attempt fails, then there will be maxAttempts - 1\nretries). Must be >= -1.\n\nIf unspecified when the queue is created, Cloud Tasks will pick\nthe default.\n\n-1 indicates unlimited attempts."]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_backoff` after provisioning.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn max_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_backoff", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_doublings` after provisioning.\nThe time between retries will double maxDoublings times.\n\nA task's retry interval starts at minBackoff, then doubles maxDoublings times,\nthen increases linearly, and finally retries retries at intervals of maxBackoff\nup to maxAttempts times."]
    pub fn max_doublings(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_doublings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_retry_duration` after provisioning.\nIf positive, maxRetryDuration specifies the time limit for\nretrying a failed task, measured from when the task was first\nattempted. Once maxRetryDuration time has passed and the task has\nbeen attempted maxAttempts times, no further attempts will be\nmade and the task will be deleted.\n\nIf zero, then the task age is unlimited."]
    pub fn max_retry_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retry_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_backoff` after provisioning.\nA task will be scheduled for retry between minBackoff and\nmaxBackoff duration after it fails, if the queue's RetryConfig\nspecifies that the task should be retried."]
    pub fn min_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_backoff", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueueStackdriverLoggingConfigEl {
    sampling_ratio: terrars::PrimField<f64>,
}

impl CloudTasksQueueStackdriverLoggingConfigEl { }

impl terrars::ToListMappable for CloudTasksQueueStackdriverLoggingConfigEl {
    type O = terrars::BlockAssignable<CloudTasksQueueStackdriverLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueueStackdriverLoggingConfigEl {
    #[doc =
        "Specifies the fraction of operations to write to Stackdriver Logging.\nThis field may contain any value between 0.0 and 1.0, inclusive. 0.0 is the\ndefault and means that no operations are logged."]
    pub sampling_ratio: terrars::PrimField<f64>,
}

impl BuildCloudTasksQueueStackdriverLoggingConfigEl {
    pub fn build(self) -> CloudTasksQueueStackdriverLoggingConfigEl {
        CloudTasksQueueStackdriverLoggingConfigEl { sampling_ratio: self.sampling_ratio }
    }
}

pub struct CloudTasksQueueStackdriverLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueueStackdriverLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueueStackdriverLoggingConfigElRef {
        CloudTasksQueueStackdriverLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueueStackdriverLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sampling_ratio` after provisioning.\nSpecifies the fraction of operations to write to Stackdriver Logging.\nThis field may contain any value between 0.0 and 1.0, inclusive. 0.0 is the\ndefault and means that no operations are logged."]
    pub fn sampling_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_ratio", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueueTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueueTimeoutsEl {
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

impl terrars::ToListMappable for CloudTasksQueueTimeoutsEl {
    type O = terrars::BlockAssignable<CloudTasksQueueTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueueTimeoutsEl {}

impl BuildCloudTasksQueueTimeoutsEl {
    pub fn build(self) -> CloudTasksQueueTimeoutsEl {
        CloudTasksQueueTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudTasksQueueTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueueTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueueTimeoutsElRef {
        CloudTasksQueueTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueueTimeoutsElRef {
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
struct CloudTasksQueueDynamic {
    app_engine_routing_override: std
    ::option
    ::Option<terrars::DynamicBlock<CloudTasksQueueAppEngineRoutingOverrideEl>>,
    rate_limits: std::option::Option<terrars::DynamicBlock<CloudTasksQueueRateLimitsEl>>,
    retry_config: std::option::Option<terrars::DynamicBlock<CloudTasksQueueRetryConfigEl>>,
    stackdriver_logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudTasksQueueStackdriverLoggingConfigEl>>,
}
