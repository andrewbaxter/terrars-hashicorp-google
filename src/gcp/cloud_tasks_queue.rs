#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudTasksQueueData_ {
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
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine_routing_override: std::option::Option<std::vec::Vec<CloudTasksQueue_AppEngineRoutingOverride>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_target: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limits: std::option::Option<std::vec::Vec<CloudTasksQueue_RateLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_config: std::option::Option<std::vec::Vec<CloudTasksQueue_RetryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stackdriver_logging_config: std::option::Option<std::vec::Vec<CloudTasksQueue_StackdriverLoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudTasksQueue_Timeouts>,
    dynamic: CloudTasksQueueDynamic,
}

struct CloudTasksQueue_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudTasksQueueData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_state`.\nThe desired state of the queue. Use this to pause and resume the queue.\n\n* RUNNING: The queue is running. Tasks can be dispatched.\n* PAUSED: The queue is paused. Tasks are not dispatched but can be added to the queue. Default value: \"RUNNING\" Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn set_desired_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_AppEngineRoutingOverride>>,
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

    #[doc = "Set the field `http_target`.\n"]
    pub fn set_http_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget>>,
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

    #[doc = "Set the field `rate_limits`.\n"]
    pub fn set_rate_limits(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_RateLimits>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_RetryConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_StackdriverLoggingConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudTasksQueue_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state of the queue. Use this to pause and resume the queue.\n\n* RUNNING: The queue is running. Tasks can be dispatched.\n* PAUSED: The queue is paused. Tasks are not dispatched but can be added to the queue. Default value: \"RUNNING\" Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the queue."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine_routing_override` after provisioning.\n"]
    pub fn app_engine_routing_override(&self) -> terrars::ListRef<CloudTasksQueue_AppEngineRoutingOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_routing_override", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudTasksQueue_HttpTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limits` after provisioning.\n"]
    pub fn rate_limits(&self) -> terrars::ListRef<CloudTasksQueue_RateLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> terrars::ListRef<CloudTasksQueue_RetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stackdriver_logging_config` after provisioning.\n"]
    pub fn stackdriver_logging_config(&self) -> terrars::ListRef<CloudTasksQueue_StackdriverLoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stackdriver_logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudTasksQueue_TimeoutsRef {
        CloudTasksQueue_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
    #[doc = "The queue name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudTasksQueue {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudTasksQueue {
        let out = CloudTasksQueue(std::rc::Rc::new(CloudTasksQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudTasksQueueData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                app_engine_routing_override: core::default::Default::default(),
                http_target: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state of the queue. Use this to pause and resume the queue.\n\n* RUNNING: The queue is running. Tasks can be dispatched.\n* PAUSED: The queue is paused. Tasks are not dispatched but can be added to the queue. Default value: \"RUNNING\" Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the queue."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine_routing_override` after provisioning.\n"]
    pub fn app_engine_routing_override(&self) -> terrars::ListRef<CloudTasksQueue_AppEngineRoutingOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_routing_override", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_target` after provisioning.\n"]
    pub fn http_target(&self) -> terrars::ListRef<CloudTasksQueue_HttpTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rate_limits` after provisioning.\n"]
    pub fn rate_limits(&self) -> terrars::ListRef<CloudTasksQueue_RateLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limits", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_config` after provisioning.\n"]
    pub fn retry_config(&self) -> terrars::ListRef<CloudTasksQueue_RetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stackdriver_logging_config` after provisioning.\n"]
    pub fn stackdriver_logging_config(&self) -> terrars::ListRef<CloudTasksQueue_StackdriverLoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stackdriver_logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudTasksQueue_TimeoutsRef {
        CloudTasksQueue_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudTasksQueueDynamic {
    app_engine_routing_override: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_AppEngineRoutingOverride>>,
    http_target: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget>>,
    rate_limits: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_RateLimits>>,
    retry_config: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_RetryConfig>>,
    stackdriver_logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudTasksQueue_StackdriverLoggingConfig>>,
}

#[derive(Serialize)]
pub struct CloudTasksQueue_AppEngineRoutingOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueue_AppEngineRoutingOverride {
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

impl terrars::ToListMappable for CloudTasksQueue_AppEngineRoutingOverride {
    type O = terrars::BlockAssignable<CloudTasksQueue_AppEngineRoutingOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_AppEngineRoutingOverride {}

impl BuildCloudTasksQueue_AppEngineRoutingOverride {
    pub fn build(self) -> CloudTasksQueue_AppEngineRoutingOverride {
        CloudTasksQueue_AppEngineRoutingOverride {
            instance: core::default::Default::default(),
            service: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_AppEngineRoutingOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_AppEngineRoutingOverrideRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_AppEngineRoutingOverrideRef {
        CloudTasksQueue_AppEngineRoutingOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_AppEngineRoutingOverrideRef {
    #[allow(dead_code)]
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
pub struct CloudTasksQueue_HttpTarget_HeaderOverrides_Header {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudTasksQueue_HttpTarget_HeaderOverrides_Header { }

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_HeaderOverrides_Header {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_HeaderOverrides_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_HeaderOverrides_Header {
    #[doc = "The Key of the header."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The Value of the header."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudTasksQueue_HttpTarget_HeaderOverrides_Header {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_HeaderOverrides_Header {
        CloudTasksQueue_HttpTarget_HeaderOverrides_Header {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_HeaderOverrides_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_HeaderOverrides_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudTasksQueue_HttpTarget_HeaderOverrides_HeaderRef {
        CloudTasksQueue_HttpTarget_HeaderOverrides_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_HeaderOverrides_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe Key of the header."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe Value of the header."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudTasksQueue_HttpTarget_HeaderOverridesDynamic {
    header: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_HeaderOverrides_Header>>,
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget_HeaderOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_HeaderOverrides_Header>>,
    dynamic: CloudTasksQueue_HttpTarget_HeaderOverridesDynamic,
}

impl CloudTasksQueue_HttpTarget_HeaderOverrides {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_HeaderOverrides_Header>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_HeaderOverrides {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_HeaderOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_HeaderOverrides {}

impl BuildCloudTasksQueue_HttpTarget_HeaderOverrides {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_HeaderOverrides {
        CloudTasksQueue_HttpTarget_HeaderOverrides {
            header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_HeaderOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_HeaderOverridesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_HttpTarget_HeaderOverridesRef {
        CloudTasksQueue_HttpTarget_HeaderOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_HeaderOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_HeaderOverrides_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget_OauthToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account_email: terrars::PrimField<std::string::String>,
}

impl CloudTasksQueue_HttpTarget_OauthToken {
    #[doc =
        "Set the field `scope`.\nOAuth scope to be used for generating OAuth access token.\nIf not specified, \"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_OauthToken {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_OauthToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_OauthToken {
    #[doc =
        "Service account email to be used for generating OAuth token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: terrars::PrimField<std::string::String>,
}

impl BuildCloudTasksQueue_HttpTarget_OauthToken {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_OauthToken {
        CloudTasksQueue_HttpTarget_OauthToken {
            scope: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_OauthTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_OauthTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_HttpTarget_OauthTokenRef {
        CloudTasksQueue_HttpTarget_OauthTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_OauthTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nOAuth scope to be used for generating OAuth access token.\nIf not specified, \"https://www.googleapis.com/auth/cloud-platform\" will be used."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating OAuth token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget_OidcToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account_email: terrars::PrimField<std::string::String>,
}

impl CloudTasksQueue_HttpTarget_OidcToken {
    #[doc =
        "Set the field `audience`.\nAudience to be used when generating OIDC token. If not specified, the URI specified in target will be used."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_OidcToken {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_OidcToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_OidcToken {
    #[doc =
        "Service account email to be used for generating OIDC token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: terrars::PrimField<std::string::String>,
}

impl BuildCloudTasksQueue_HttpTarget_OidcToken {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_OidcToken {
        CloudTasksQueue_HttpTarget_OidcToken {
            audience: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_OidcTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_OidcTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_HttpTarget_OidcTokenRef {
        CloudTasksQueue_HttpTarget_OidcTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_OidcTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. If not specified, the URI specified in target will be used."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating OIDC token.\nThe service account must be within the same project as the queue.\nThe caller must have iam.serviceAccounts.actAs permission for the service account."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget_UriOverride_PathOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueue_HttpTarget_UriOverride_PathOverride {
    #[doc = "Set the field `path`.\nThe URI path (e.g., /users/1234). Default is an empty string."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_UriOverride_PathOverride {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_UriOverride_PathOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_UriOverride_PathOverride {}

impl BuildCloudTasksQueue_HttpTarget_UriOverride_PathOverride {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_UriOverride_PathOverride {
        CloudTasksQueue_HttpTarget_UriOverride_PathOverride { path: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_UriOverride_PathOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_UriOverride_PathOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudTasksQueue_HttpTarget_UriOverride_PathOverrideRef {
        CloudTasksQueue_HttpTarget_UriOverride_PathOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_UriOverride_PathOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe URI path (e.g., /users/1234). Default is an empty string."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget_UriOverride_QueryOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_params: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueue_HttpTarget_UriOverride_QueryOverride {
    #[doc =
        "Set the field `query_params`.\nThe query parameters (e.g., qparam1=123&qparam2=456). Default is an empty string."]
    pub fn set_query_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_params = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_UriOverride_QueryOverride {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_UriOverride_QueryOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_UriOverride_QueryOverride {}

impl BuildCloudTasksQueue_HttpTarget_UriOverride_QueryOverride {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_UriOverride_QueryOverride {
        CloudTasksQueue_HttpTarget_UriOverride_QueryOverride { query_params: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_UriOverride_QueryOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_UriOverride_QueryOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudTasksQueue_HttpTarget_UriOverride_QueryOverrideRef {
        CloudTasksQueue_HttpTarget_UriOverride_QueryOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_UriOverride_QueryOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query_params` after provisioning.\nThe query parameters (e.g., qparam1=123&qparam2=456). Default is an empty string."]
    pub fn query_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_params", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudTasksQueue_HttpTarget_UriOverrideDynamic {
    path_override: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_UriOverride_PathOverride>>,
    query_override: std
    ::option
    ::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_UriOverride_QueryOverride>>,
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget_UriOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_override_enforce_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_override: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_UriOverride_PathOverride>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_override: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_UriOverride_QueryOverride>>,
    dynamic: CloudTasksQueue_HttpTarget_UriOverrideDynamic,
}

impl CloudTasksQueue_HttpTarget_UriOverride {
    #[doc =
        "Set the field `host`.\nHost override.\n\nWhen specified, replaces the host part of the task URL.\nFor example, if the task URL is \"https://www.google.com\", and host value\nis set to \"example.net\", the overridden URI will be changed to \"https://example.net\".\nHost value cannot be an empty string (INVALID_ARGUMENT)."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nPort override.\n\nWhen specified, replaces the port part of the task URI.\nFor instance, for a URI http://www.google.com/foo and port=123, the overridden URI becomes http://www.google.com:123/foo.\nNote that the port value must be a positive integer.\nSetting the port to 0 (Zero) clears the URI port."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scheme`.\nScheme override.\n\nWhen specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS). Possible values: [\"HTTP\", \"HTTPS\"]"]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `uri_override_enforce_mode`.\nURI Override Enforce Mode\n\nWhen specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS. Possible values: [\"ALWAYS\", \"IF_NOT_EXISTS\"]"]
    pub fn set_uri_override_enforce_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.uri_override_enforce_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_override`.\n"]
    pub fn set_path_override(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_UriOverride_PathOverride>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_override`.\n"]
    pub fn set_query_override(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_UriOverride_QueryOverride>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget_UriOverride {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget_UriOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget_UriOverride {}

impl BuildCloudTasksQueue_HttpTarget_UriOverride {
    pub fn build(self) -> CloudTasksQueue_HttpTarget_UriOverride {
        CloudTasksQueue_HttpTarget_UriOverride {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            scheme: core::default::Default::default(),
            uri_override_enforce_mode: core::default::Default::default(),
            path_override: core::default::Default::default(),
            query_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTarget_UriOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTarget_UriOverrideRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_HttpTarget_UriOverrideRef {
        CloudTasksQueue_HttpTarget_UriOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTarget_UriOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nHost override.\n\nWhen specified, replaces the host part of the task URL.\nFor example, if the task URL is \"https://www.google.com\", and host value\nis set to \"example.net\", the overridden URI will be changed to \"https://example.net\".\nHost value cannot be an empty string (INVALID_ARGUMENT)."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort override.\n\nWhen specified, replaces the port part of the task URI.\nFor instance, for a URI http://www.google.com/foo and port=123, the overridden URI becomes http://www.google.com:123/foo.\nNote that the port value must be a positive integer.\nSetting the port to 0 (Zero) clears the URI port."]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme` after provisioning.\nScheme override.\n\nWhen specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS). Possible values: [\"HTTP\", \"HTTPS\"]"]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri_override_enforce_mode` after provisioning.\nURI Override Enforce Mode\n\nWhen specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS. Possible values: [\"ALWAYS\", \"IF_NOT_EXISTS\"]"]
    pub fn uri_override_enforce_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri_override_enforce_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `path_override` after provisioning.\n"]
    pub fn path_override(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_UriOverride_PathOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_override", self.base))
    }

    #[doc = "Get a reference to the value of field `query_override` after provisioning.\n"]
    pub fn query_override(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_UriOverride_QueryOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_override", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudTasksQueue_HttpTargetDynamic {
    header_overrides: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_HeaderOverrides>>,
    oauth_token: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_OauthToken>>,
    oidc_token: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_OidcToken>>,
    uri_override: std::option::Option<terrars::DynamicBlock<CloudTasksQueue_HttpTarget_UriOverride>>,
}

#[derive(Serialize)]
pub struct CloudTasksQueue_HttpTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_overrides: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_HeaderOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_OauthToken>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_OidcToken>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_override: std::option::Option<std::vec::Vec<CloudTasksQueue_HttpTarget_UriOverride>>,
    dynamic: CloudTasksQueue_HttpTargetDynamic,
}

impl CloudTasksQueue_HttpTarget {
    #[doc =
        "Set the field `http_method`.\nThe HTTP method to use for the request.\n\nWhen specified, it overrides HttpRequest for the task.\nNote that if the value is set to GET the body of the task will be ignored at execution time. Possible values: [\"HTTP_METHOD_UNSPECIFIED\", \"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn set_http_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_overrides`.\n"]
    pub fn set_header_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_HeaderOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_token`.\n"]
    pub fn set_oauth_token(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_OauthToken>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_OidcToken>>,
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

    #[doc = "Set the field `uri_override`.\n"]
    pub fn set_uri_override(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudTasksQueue_HttpTarget_UriOverride>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uri_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudTasksQueue_HttpTarget {
    type O = terrars::BlockAssignable<CloudTasksQueue_HttpTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_HttpTarget {}

impl BuildCloudTasksQueue_HttpTarget {
    pub fn build(self) -> CloudTasksQueue_HttpTarget {
        CloudTasksQueue_HttpTarget {
            http_method: core::default::Default::default(),
            header_overrides: core::default::Default::default(),
            oauth_token: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            uri_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_HttpTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_HttpTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_HttpTargetRef {
        CloudTasksQueue_HttpTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_HttpTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_method` after provisioning.\nThe HTTP method to use for the request.\n\nWhen specified, it overrides HttpRequest for the task.\nNote that if the value is set to GET the body of the task will be ignored at execution time. Possible values: [\"HTTP_METHOD_UNSPECIFIED\", \"POST\", \"GET\", \"HEAD\", \"PUT\", \"DELETE\", \"PATCH\", \"OPTIONS\"]"]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc = "Get a reference to the value of field `header_overrides` after provisioning.\n"]
    pub fn header_overrides(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_HeaderOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_token` after provisioning.\n"]
    pub fn oauth_token(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_OauthTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_token", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_OidcTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_override` after provisioning.\n"]
    pub fn uri_override(&self) -> terrars::ListRef<CloudTasksQueue_HttpTarget_UriOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri_override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudTasksQueue_RateLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_dispatches: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_dispatches_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudTasksQueue_RateLimits {
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

impl terrars::ToListMappable for CloudTasksQueue_RateLimits {
    type O = terrars::BlockAssignable<CloudTasksQueue_RateLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_RateLimits {}

impl BuildCloudTasksQueue_RateLimits {
    pub fn build(self) -> CloudTasksQueue_RateLimits {
        CloudTasksQueue_RateLimits {
            max_concurrent_dispatches: core::default::Default::default(),
            max_dispatches_per_second: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_RateLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_RateLimitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_RateLimitsRef {
        CloudTasksQueue_RateLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_RateLimitsRef {
    #[allow(dead_code)]
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
pub struct CloudTasksQueue_RetryConfig {
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

impl CloudTasksQueue_RetryConfig {
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

impl terrars::ToListMappable for CloudTasksQueue_RetryConfig {
    type O = terrars::BlockAssignable<CloudTasksQueue_RetryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_RetryConfig {}

impl BuildCloudTasksQueue_RetryConfig {
    pub fn build(self) -> CloudTasksQueue_RetryConfig {
        CloudTasksQueue_RetryConfig {
            max_attempts: core::default::Default::default(),
            max_backoff: core::default::Default::default(),
            max_doublings: core::default::Default::default(),
            max_retry_duration: core::default::Default::default(),
            min_backoff: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_RetryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_RetryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_RetryConfigRef {
        CloudTasksQueue_RetryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_RetryConfigRef {
    #[allow(dead_code)]
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
pub struct CloudTasksQueue_StackdriverLoggingConfig {
    sampling_ratio: terrars::PrimField<f64>,
}

impl CloudTasksQueue_StackdriverLoggingConfig { }

impl terrars::ToListMappable for CloudTasksQueue_StackdriverLoggingConfig {
    type O = terrars::BlockAssignable<CloudTasksQueue_StackdriverLoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_StackdriverLoggingConfig {
    #[doc =
        "Specifies the fraction of operations to write to Stackdriver Logging.\nThis field may contain any value between 0.0 and 1.0, inclusive. 0.0 is the\ndefault and means that no operations are logged."]
    pub sampling_ratio: terrars::PrimField<f64>,
}

impl BuildCloudTasksQueue_StackdriverLoggingConfig {
    pub fn build(self) -> CloudTasksQueue_StackdriverLoggingConfig {
        CloudTasksQueue_StackdriverLoggingConfig { sampling_ratio: self.sampling_ratio }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_StackdriverLoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_StackdriverLoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_StackdriverLoggingConfigRef {
        CloudTasksQueue_StackdriverLoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_StackdriverLoggingConfigRef {
    #[allow(dead_code)]
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
pub struct CloudTasksQueue_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudTasksQueue_Timeouts {
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

impl terrars::ToListMappable for CloudTasksQueue_Timeouts {
    type O = terrars::BlockAssignable<CloudTasksQueue_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudTasksQueue_Timeouts {}

impl BuildCloudTasksQueue_Timeouts {
    pub fn build(self) -> CloudTasksQueue_Timeouts {
        CloudTasksQueue_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudTasksQueue_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudTasksQueue_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudTasksQueue_TimeoutsRef {
        CloudTasksQueue_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudTasksQueue_TimeoutsRef {
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
