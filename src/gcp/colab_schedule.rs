#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ColabScheduleData_ {
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
    allow_queueing: std::option::Option<terrars::PrimField<bool>>,
    cron: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_active_run_count: std::option::Option<terrars::PrimField<std::string::String>>,
    max_concurrent_run_count: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_run_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_notebook_execution_job_request: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreateNotebookExecutionJobRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_pipeline_job_request: std::option::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ColabSchedule_Timeouts>,
    dynamic: ColabScheduleDynamic,
}

struct ColabSchedule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ColabScheduleData_>,
}

#[derive(Clone)]
pub struct ColabSchedule(std::rc::Rc<ColabSchedule_>);

impl ColabSchedule {
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
        "Set the field `allow_queueing`.\nWhether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false."]
    pub fn set_allow_queueing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_queueing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_state`.\nDesired state of the Colab Schedule. Set this field to 'ACTIVE' to start/resume the schedule, and 'PAUSED' to pause the schedule."]
    pub fn set_desired_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `end_time`.\nTimestamp after which no new runs can be scheduled. If specified, the schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn set_end_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_concurrent_active_run_count`.\nSpecifies the maximum number of active runs that can be executed concurrently for this Schedule. This limits the number of runs that can be in a non-terminal state at the same time. Currently, this field is only supported for requests of type CreatePipelineJobRequest."]
    pub fn set_max_concurrent_active_run_count(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().max_concurrent_active_run_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_run_count`.\nMaximum run count of the schedule. If specified, The schedule will be completed when either startedRunCount >= maxRunCount or when endTime is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified."]
    pub fn set_max_run_count(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().max_run_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time`.\nThe timestamp after which the first run can be scheduled. Defaults to the schedule creation time. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn set_start_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_notebook_execution_job_request`.\n"]
    pub fn set_create_notebook_execution_job_request(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_notebook_execution_job_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.create_notebook_execution_job_request =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_pipeline_job_request`.\n"]
    pub fn set_create_pipeline_job_request(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_pipeline_job_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.create_pipeline_job_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ColabSchedule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_queueing` after provisioning.\nWhether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false."]
    pub fn allow_queueing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_queueing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `catch_up` after provisioning.\nWhether to backfill missed runs when the schedule is resumed from PAUSED state. If set to true, all missed runs will be scheduled. New runs will be scheduled after the backfill is complete. Default to false."]
    pub fn catch_up(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catch_up", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this Schedule was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cron` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs."]
    pub fn cron(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Colab Schedule. Set this field to 'ACTIVE' to start/resume the schedule, and 'PAUSED' to pause the schedule."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Schedule."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nTimestamp after which no new runs can be scheduled. If specified, the schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_pause_time` after provisioning.\nTimestamp when this Schedule was last paused. Unset if never paused."]
    pub fn last_pause_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_pause_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_resume_time` after provisioning.\nTimestamp when this Schedule was last resumed. Unset if never resumed from pause."]
    pub fn last_resume_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_resume_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_scheduled_run_response` after provisioning.\nStatus of a scheduled run."]
    pub fn last_scheduled_run_response(&self) -> terrars::ListRef<ColabSchedule_LastScheduledRunResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_scheduled_run_response", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_active_run_count` after provisioning.\nSpecifies the maximum number of active runs that can be executed concurrently for this Schedule. This limits the number of runs that can be in a non-terminal state at the same time. Currently, this field is only supported for requests of type CreatePipelineJobRequest."]
    pub fn max_concurrent_active_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_active_run_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_run_count` after provisioning.\nMaximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the notebook execution jobs created by the requests."]
    pub fn max_concurrent_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_run_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_run_count` after provisioning.\nMaximum run count of the schedule. If specified, The schedule will be completed when either startedRunCount >= maxRunCount or when endTime is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified."]
    pub fn max_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_run_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Schedule"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_run_time` after provisioning.\nTimestamp when this Schedule should schedule the next run. Having a next_run_time in the past means the runs are being started behind schedule."]
    pub fn next_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_run_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe timestamp after which the first run can be scheduled. Defaults to the schedule creation time. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `started_run_count` after provisioning.\nThe number of runs started by this schedule."]
    pub fn started_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_run_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the schedule."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this Schedule was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_notebook_execution_job_request` after provisioning.\n"]
    pub fn create_notebook_execution_job_request(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreateNotebookExecutionJobRequestRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_notebook_execution_job_request", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_pipeline_job_request` after provisioning.\n"]
    pub fn create_pipeline_job_request(&self) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_pipeline_job_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabSchedule_TimeoutsRef {
        ColabSchedule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ColabSchedule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ColabSchedule { }

impl terrars::ToListMappable for ColabSchedule {
    type O = terrars::ListRef<ColabScheduleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ColabSchedule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_colab_schedule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildColabSchedule {
    pub tf_id: std::string::String,
    #[doc = "Cron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs."]
    pub cron: terrars::PrimField<std::string::String>,
    #[doc = "Required. The display name of the Schedule."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Maximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the notebook execution jobs created by the requests."]
    pub max_concurrent_run_count: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule {
    pub fn build(self, stack: &mut terrars::Stack) -> ColabSchedule {
        let out = ColabSchedule(std::rc::Rc::new(ColabSchedule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ColabScheduleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_queueing: core::default::Default::default(),
                cron: self.cron,
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                display_name: self.display_name,
                end_time: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                max_concurrent_active_run_count: core::default::Default::default(),
                max_concurrent_run_count: self.max_concurrent_run_count,
                max_run_count: core::default::Default::default(),
                project: core::default::Default::default(),
                start_time: core::default::Default::default(),
                create_notebook_execution_job_request: core::default::Default::default(),
                create_pipeline_job_request: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ColabScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ColabScheduleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_queueing` after provisioning.\nWhether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false."]
    pub fn allow_queueing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_queueing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `catch_up` after provisioning.\nWhether to backfill missed runs when the schedule is resumed from PAUSED state. If set to true, all missed runs will be scheduled. New runs will be scheduled after the backfill is complete. Default to false."]
    pub fn catch_up(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catch_up", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this Schedule was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cron` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs."]
    pub fn cron(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Colab Schedule. Set this field to 'ACTIVE' to start/resume the schedule, and 'PAUSED' to pause the schedule."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Schedule."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nTimestamp after which no new runs can be scheduled. If specified, the schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_pause_time` after provisioning.\nTimestamp when this Schedule was last paused. Unset if never paused."]
    pub fn last_pause_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_pause_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_resume_time` after provisioning.\nTimestamp when this Schedule was last resumed. Unset if never resumed from pause."]
    pub fn last_resume_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_resume_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_scheduled_run_response` after provisioning.\nStatus of a scheduled run."]
    pub fn last_scheduled_run_response(&self) -> terrars::ListRef<ColabSchedule_LastScheduledRunResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_scheduled_run_response", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_active_run_count` after provisioning.\nSpecifies the maximum number of active runs that can be executed concurrently for this Schedule. This limits the number of runs that can be in a non-terminal state at the same time. Currently, this field is only supported for requests of type CreatePipelineJobRequest."]
    pub fn max_concurrent_active_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_active_run_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_run_count` after provisioning.\nMaximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the notebook execution jobs created by the requests."]
    pub fn max_concurrent_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_run_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_run_count` after provisioning.\nMaximum run count of the schedule. If specified, The schedule will be completed when either startedRunCount >= maxRunCount or when endTime is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified."]
    pub fn max_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_run_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Schedule"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_run_time` after provisioning.\nTimestamp when this Schedule should schedule the next run. Having a next_run_time in the past means the runs are being started behind schedule."]
    pub fn next_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_run_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe timestamp after which the first run can be scheduled. Defaults to the schedule creation time. Must be in the RFC 3339 (https://www.ietf.org/rfc/rfc3339.txt) format."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `started_run_count` after provisioning.\nThe number of runs started by this schedule."]
    pub fn started_run_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_run_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the schedule."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this Schedule was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_notebook_execution_job_request` after provisioning.\n"]
    pub fn create_notebook_execution_job_request(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreateNotebookExecutionJobRequestRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.create_notebook_execution_job_request", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_pipeline_job_request` after provisioning.\n"]
    pub fn create_pipeline_job_request(&self) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_pipeline_job_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabSchedule_TimeoutsRef {
        ColabSchedule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ColabScheduleDynamic {
    create_notebook_execution_job_request: std
    ::option
    ::Option<terrars::DynamicBlock<ColabSchedule_CreateNotebookExecutionJobRequest>>,
    create_pipeline_job_request: std::option::Option<terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest>>,
}

#[derive(Serialize)]
pub struct ColabSchedule_LastScheduledRunResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_run_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabSchedule_LastScheduledRunResponse {
    #[doc = "Set the field `run_response`.\n"]
    pub fn set_run_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheduled_run_time`.\n"]
    pub fn set_scheduled_run_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scheduled_run_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_LastScheduledRunResponse {
    type O = terrars::BlockAssignable<ColabSchedule_LastScheduledRunResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_LastScheduledRunResponse {}

impl BuildColabSchedule_LastScheduledRunResponse {
    pub fn build(self) -> ColabSchedule_LastScheduledRunResponse {
        ColabSchedule_LastScheduledRunResponse {
            run_response: core::default::Default::default(),
            scheduled_run_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_LastScheduledRunResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_LastScheduledRunResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabSchedule_LastScheduledRunResponseRef {
        ColabSchedule_LastScheduledRunResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_LastScheduledRunResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `run_response` after provisioning.\n"]
    pub fn run_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_response", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduled_run_time` after provisioning.\n"]
    pub fn scheduled_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduled_run_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    reservation_affinity_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_reservation_pool: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
    #[doc =
        "Set the field `key`.\nCorresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, use 'compute.googleapis.com/reservation-name' as the key and specify the name of your reservation as its value."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_reservation_pool`.\nWhen set to true, resources will be drawn from go/cloud-ai-gcp-pool."]
    pub fn set_use_reservation_pool(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_reservation_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `values`.\nCorresponds to the label values of a reservation resource. This must be the full resource name of the reservation or reservation block."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
    type O =
        terrars
        ::BlockAssignable<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
    #[doc =
        "Specifies the reservation affinity type. Possible values: NO_RESERVATION ANY_RESERVATION SPECIFIC_RESERVATION SPECIFIC_THEN_ANY_RESERVATION SPECIFIC_THEN_NO_RESERVATION"]
    pub reservation_affinity_type: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
    pub fn build(
        self,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity {
            key: core::default::Default::default(),
            reservation_affinity_type: self.reservation_affinity_type,
            use_reservation_pool: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinityRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, use 'compute.googleapis.com/reservation-name' as the key and specify the name of your reservation as its value."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reservation_affinity_type` after provisioning.\nSpecifies the reservation affinity type. Possible values: NO_RESERVATION ANY_RESERVATION SPECIFIC_RESERVATION SPECIFIC_THEN_ANY_RESERVATION SPECIFIC_THEN_NO_RESERVATION"]
    pub fn reservation_affinity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_affinity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_reservation_pool` after provisioning.\nWhen set to true, resources will be drawn from go/cloud-ai-gcp-pool."]
    pub fn use_reservation_pool(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_reservation_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource. This must be the full resource name of the reservation or reservation block."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecDynamic {
    reservation_affinity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity,
        >,
    >,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tpu_topology: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity,
        >,
    >,
    dynamic: ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecDynamic,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {
    #[doc =
        "Set the field `accelerator_count`.\nThe number of accelerators to attach to the machine. For accelerator optimized machine types (https://cloud.google.com/compute/docs/accelerator-optimized-machines), One may set the accelerator_count from 1 to N for machine with N GPUs. If accelerator_count is less than or equal to N / 2, Vertex will co-schedule the replicas of the model into the same VM to save cost. For example, if the machine type is a3-highgpu-8g, which has 8 H100 GPUs, one can set accelerator_count to 1 to 8. If accelerator_count is 1, 2, 3, or 4, Vertex will co-schedule 8, 4, 2, or 2 replicas of the model into the same VM to save cost. When co-scheduling, CPU, memory and storage on the VM will be distributed to replicas on the VM. For example, one can expect a co-scheduled replica requesting 2 GPUs out of a 8-GPU VM will receive 25% of the CPU, memory and storage of the VM. Note that the feature is not compatible with multihost_gpu_node_count. When multihost_gpu_node_count is set, the co-scheduling will not be enabled."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nPossible values: NVIDIA_TESLA_K80 NVIDIA_TESLA_P100 NVIDIA_TESLA_V100 NVIDIA_TESLA_P4 NVIDIA_TESLA_T4 NVIDIA_TESLA_A100 NVIDIA_A100_80GB NVIDIA_L4 NVIDIA_H100_80GB NVIDIA_H100_MEGA_80GB NVIDIA_H200_141GB NVIDIA_B200 NVIDIA_GB200 NVIDIA_RTX_PRO_6000 TPU_V2 TPU_V3 TPU_V4_POD TPU_V5_LITEPOD"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gpu_partition_size`.\nThe Nvidia GPU partition size. When specified, the requested accelerators will be partitioned into smaller GPU partitions. For example, if the request is for 8 units of NVIDIA A100 GPUs, and gpu_partition_size=\"1g.10gb\", the service will create 8 * 7 = 56 partitioned MIG instances. The partition size must be a value supported by the requested accelerator. Refer to [Nvidia GPU Partitioning](https://cloud.google.com/kubernetes-engine/docs/how-to/gpus-multi#multi-instance_gpu_partitions) for the available partition sizes. If set, the accelerator_count should be set to 1."]
    pub fn set_gpu_partition_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gpu_partition_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types) See the [list of machine types supported for custom training](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types). For DeployedModel this field is optional, and the default value is 'n1-standard-2'. For BatchPredictionJob or as part of WorkerPoolSpec this field is required."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tpu_topology`.\nThe topology of the TPUs. Corresponds to the TPU topologies available from GKE. (Example: tpu_topology: \"2x2x1\")."]
    pub fn set_tpu_topology(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tpu_topology = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reservation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {
    type O =
        terrars
        ::BlockAssignable<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {
    pub fn build(
        self,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            gpu_partition_size: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            tpu_topology: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators to attach to the machine. For accelerator optimized machine types (https://cloud.google.com/compute/docs/accelerator-optimized-machines), One may set the accelerator_count from 1 to N for machine with N GPUs. If accelerator_count is less than or equal to N / 2, Vertex will co-schedule the replicas of the model into the same VM to save cost. For example, if the machine type is a3-highgpu-8g, which has 8 H100 GPUs, one can set accelerator_count to 1 to 8. If accelerator_count is 1, 2, 3, or 4, Vertex will co-schedule 8, 4, 2, or 2 replicas of the model into the same VM to save cost. When co-scheduling, CPU, memory and storage on the VM will be distributed to replicas on the VM. For example, one can expect a co-scheduled replica requesting 2 GPUs out of a 8-GPU VM will receive 25% of the CPU, memory and storage of the VM. Note that the feature is not compatible with multihost_gpu_node_count. When multihost_gpu_node_count is set, the co-scheduling will not be enabled."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nPossible values: NVIDIA_TESLA_K80 NVIDIA_TESLA_P100 NVIDIA_TESLA_V100 NVIDIA_TESLA_P4 NVIDIA_TESLA_T4 NVIDIA_TESLA_A100 NVIDIA_A100_80GB NVIDIA_L4 NVIDIA_H100_80GB NVIDIA_H100_MEGA_80GB NVIDIA_H200_141GB NVIDIA_B200 NVIDIA_GB200 NVIDIA_RTX_PRO_6000 TPU_V2 TPU_V3 TPU_V4_POD TPU_V5_LITEPOD"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gpu_partition_size` after provisioning.\nThe Nvidia GPU partition size. When specified, the requested accelerators will be partitioned into smaller GPU partitions. For example, if the request is for 8 units of NVIDIA A100 GPUs, and gpu_partition_size=\"1g.10gb\", the service will create 8 * 7 = 56 partitioned MIG instances. The partition size must be a value supported by the requested accelerator. Refer to [Nvidia GPU Partitioning](https://cloud.google.com/kubernetes-engine/docs/how-to/gpus-multi#multi-instance_gpu_partitions) for the available partition sizes. If set, the accelerator_count should be set to 1."]
    pub fn gpu_partition_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine. See the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types) See the [list of machine types supported for custom training](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types). For DeployedModel this field is optional, and the default value is 'n1-standard-2'. For BatchPredictionJob or as part of WorkerPoolSpec this field is required."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tpu_topology` after provisioning.\nThe topology of the TPUs. Corresponds to the TPU topologies available from GKE. (Example: tpu_topology: \"2x2x1\")."]
    pub fn tpu_topology(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tpu_topology", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> terrars
    ::ListRef<
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec_ReservationAffinityRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_internet_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {
    #[doc = "Set the field `enable_internet_access`.\nWhether to enable public internet access. Default false."]
    pub fn set_enable_internet_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_internet_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks)"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe name of the subnet that this instance is in. Format: 'projects/{project_id_or_number}/regions/{region}/subnetworks/{subnetwork_id}'"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {
    type O =
        terrars
        ::BlockAssignable<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {
    pub fn build(
        self,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec {
            enable_internet_access: core::default::Default::default(),
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpecRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_internet_access` after provisioning.\nWhether to enable public internet access. Default false."]
    pub fn enable_internet_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_internet_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks)"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnet that this instance is in. Format: 'projects/{project_id_or_number}/regions/{region}/subnetworks/{subnetwork_id}'"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {
    #[doc = "Set the field `disk_size_gb`.\nSize in GB of the disk (default is 100GB)."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nType of the disk (default is \"pd-standard\"). Valid values: \"pd-ssd\" (Persistent Disk Solid State Drive) \"pd-standard\" (Persistent Disk Hard Disk Drive) \"pd-balanced\" (Balanced Persistent Disk) \"pd-extreme\" (Extreme Persistent Disk)"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {
    type O =
        terrars
        ::BlockAssignable<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {
    pub fn build(
        self,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpecRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize in GB of the disk (default is 100GB)."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk (default is \"pd-standard\"). Valid values: \"pd-ssd\" (Persistent Disk Solid State Drive) \"pd-standard\" (Persistent Disk Hard Disk Drive) \"pd-balanced\" (Balanced Persistent Disk) \"pd-extreme\" (Extreme Persistent Disk)"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecDynamic {
    machine_spec: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec,
        >,
    >,
    network_spec: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec,
        >,
    >,
    persistent_disk_spec: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec,
        >,
    >,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_spec: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_disk_spec: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec,
        >,
    >,
    dynamic: ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecDynamic,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {
    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.machine_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_spec`.\n"]
    pub fn set_network_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `persistent_disk_spec`.\n"]
    pub fn set_persistent_disk_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.persistent_disk_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.persistent_disk_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {
    type O =
        terrars
        ::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {
    pub fn build(self) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec {
            machine_spec: core::default::Default::default(),
            network_spec: core::default::Default::default(),
            persistent_disk_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(
        &self,
    ) -> terrars
    ::ListRef<
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_MachineSpecRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(
        &self,
    ) -> terrars
    ::ListRef<
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_NetworkSpecRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `persistent_disk_spec` after provisioning.\n"]
    pub fn persistent_disk_spec(
        &self,
    ) -> terrars
    ::ListRef<
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec_PersistentDiskSpecRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_disk_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    dataform_repository_resource_name: terrars::PrimField<std::string::String>,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
    #[doc =
        "Set the field `commit_sha`.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
    type O =
        terrars
        ::BlockAssignable<
            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
    #[doc =
        "The resource name of the Dataform Repository. Format: 'projects/{project_id}/locations/{location}/repositories/{repository_id}'"]
    pub dataform_repository_resource_name: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
    pub fn build(
        self,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource {
            commit_sha: core::default::Default::default(),
            dataform_repository_resource_name: self.dataform_repository_resource_name,
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySourceRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `commit_sha` after provisioning.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dataform_repository_resource_name` after provisioning.\nThe resource name of the Dataform Repository. Format: 'projects/{project_id}/locations/{location}/repositories/{repository_id}'"]
    pub fn dataform_repository_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataform_repository_resource_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec { }

impl terrars::ToListMappable for ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec {
    type O = terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec {
    #[doc =
        "Resource name of the Cloud KMS key used to protect the resource. The Cloud KMS key must be in the same region as the resource. It must have the format 'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key used to protect the resource. The Cloud KMS key must be in the same region as the resource. It must have the format 'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
    #[doc =
        "Set the field `generation`.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
    type O =
        terrars
        ::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
    #[doc = "The Cloud Storage uri pointing to the ipynb file. Format: gs://bucket/notebook_file.ipynb"]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
    pub fn build(self) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource {
            generation: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSourceRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe Cloud Storage uri pointing to the ipynb file. Format: gs://bucket/notebook_file.ipynb"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime {}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime { }

impl terrars
::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime {
    type O =
        terrars
        ::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime {}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime {
    pub fn build(self) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime {}
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntimeRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobDynamic {
    custom_environment_spec: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec>,
    >,
    dataform_repository_source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource>,
    >,
    encryption_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>>,
    gcs_notebook_source: std
    ::option
    ::Option<
        terrars::DynamicBlock<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource>,
    >,
    workbench_runtime: std
    ::option
    ::Option<
        terrars::DynamicBlock<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime>,
    >,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_user: std::option::Option<terrars::PrimField<std::string::String>>,
    gcs_output_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_runtime_template_resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_environment_spec: std
    ::option
    ::Option<
        std::vec::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataform_repository_source: std
    ::option
    ::Option<
        std::vec::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_notebook_source: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workbench_runtime: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime>>,
    dynamic: ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobDynamic,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
    #[doc =
        "Set the field `execution_timeout`.\nMax running time of the execution job in seconds (default 86400s / 24 hrs). A duration in seconds with up to nine fractional digits, ending with \"s\". Example: \"3.5s\"."]
    pub fn set_execution_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_user`.\nThe user email to run the execution as."]
    pub fn set_execution_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_user = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kernel_name`.\nThe name of the kernel to use during notebook execution. If unset, the default kernel is used."]
    pub fn set_kernel_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kernel_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nThe labels with user-defined metadata to organize NotebookExecutionJobs."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notebook_runtime_template_resource_name`.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn set_notebook_runtime_template_resource_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.notebook_runtime_template_resource_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nThe service account to run the execution as."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_environment_spec`.\n"]
    pub fn set_custom_environment_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpec,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_environment_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_environment_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataform_repository_source`.\n"]
    pub fn set_dataform_repository_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataform_repository_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataform_repository_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_notebook_source`.\n"]
    pub fn set_gcs_notebook_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_notebook_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_notebook_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workbench_runtime`.\n"]
    pub fn set_workbench_runtime(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntime,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workbench_runtime = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workbench_runtime = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
    type O = terrars::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
    #[doc = "Required. The display name of the Notebook Execution."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub gcs_output_uri: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
    pub fn build(self) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob {
            display_name: self.display_name,
            execution_timeout: core::default::Default::default(),
            execution_user: core::default::Default::default(),
            gcs_output_uri: self.gcs_output_uri,
            kernel_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            notebook_runtime_template_resource_name: core::default::Default::default(),
            service_account: core::default::Default::default(),
            custom_environment_spec: core::default::Default::default(),
            dataform_repository_source: core::default::Default::default(),
            encryption_spec: core::default::Default::default(),
            gcs_notebook_source: core::default::Default::default(),
            workbench_runtime: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobRef {
        ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when this NotebookExecutionJob was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Notebook Execution."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_timeout` after provisioning.\nMax running time of the execution job in seconds (default 86400s / 24 hrs). A duration in seconds with up to nine fractional digits, ending with \"s\". Example: \"3.5s\"."]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_user` after provisioning.\nThe user email to run the execution as."]
    pub fn execution_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_user", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcs_output_uri` after provisioning.\nThe Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub fn gcs_output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_output_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `job_state` after provisioning.\nPossible values: JOB_STATE_QUEUED JOB_STATE_PENDING JOB_STATE_RUNNING JOB_STATE_SUCCEEDED JOB_STATE_FAILED JOB_STATE_CANCELLING JOB_STATE_CANCELLED JOB_STATE_PAUSED JOB_STATE_EXPIRED JOB_STATE_UPDATING JOB_STATE_PARTIALLY_SUCCEEDED"]
    pub fn job_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kernel_name` after provisioning.\nThe name of the kernel to use during notebook execution. If unset, the default kernel is used."]
    pub fn kernel_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kernel_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize NotebookExecutionJobs."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of this NotebookExecutionJob. Format: 'projects/{project_id}/locations/{location}/notebookExecutionJobs/{job_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `notebook_runtime_template_resource_name` after provisioning.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn notebook_runtime_template_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template_resource_name", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `schedule_resource_name` after provisioning.\nThe Schedule resource name if this job is triggered by one. Format: 'projects/{project_id}/locations/{location}/schedules/{schedule_id}'"]
    pub fn schedule_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_resource_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run the execution as."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this NotebookExecutionJob was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_environment_spec` after provisioning.\n"]
    pub fn custom_environment_spec(
        &self,
    ) -> terrars
    ::ListRef<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_CustomEnvironmentSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_environment_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `dataform_repository_source` after provisioning.\n"]
    pub fn dataform_repository_source(
        &self,
    ) -> terrars
    ::ListRef<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_DataformRepositorySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataform_repository_source", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_notebook_source` after provisioning.\n"]
    pub fn gcs_notebook_source(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_GcsNotebookSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_notebook_source", self.base))
    }

    #[doc = "Get a reference to the value of field `workbench_runtime` after provisioning.\n"]
    pub fn workbench_runtime(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob_WorkbenchRuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workbench_runtime", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreateNotebookExecutionJobRequestDynamic {
    notebook_execution_job: std
    ::option
    ::Option<terrars::DynamicBlock<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob>>,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_execution_job: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob>>,
    dynamic: ColabSchedule_CreateNotebookExecutionJobRequestDynamic,
}

impl ColabSchedule_CreateNotebookExecutionJobRequest {
    #[doc =
        "Set the field `parent`.\nThe resource name of the Location to create the NotebookExecutionJob. Format: 'projects/{project}/locations/{location}'"]
    pub fn set_parent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notebook_execution_job`.\n"]
    pub fn set_notebook_execution_job(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJob>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.notebook_execution_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.notebook_execution_job = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreateNotebookExecutionJobRequest {
    type O = terrars::BlockAssignable<ColabSchedule_CreateNotebookExecutionJobRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreateNotebookExecutionJobRequest {}

impl BuildColabSchedule_CreateNotebookExecutionJobRequest {
    pub fn build(self) -> ColabSchedule_CreateNotebookExecutionJobRequest {
        ColabSchedule_CreateNotebookExecutionJobRequest {
            parent: core::default::Default::default(),
            notebook_execution_job: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreateNotebookExecutionJobRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreateNotebookExecutionJobRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreateNotebookExecutionJobRequestRef {
        ColabSchedule_CreateNotebookExecutionJobRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreateNotebookExecutionJobRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `notebook_execution_job_id` after provisioning.\nUser specified ID for the NotebookExecutionJob."]
    pub fn notebook_execution_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notebook_execution_job_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the Location to create the NotebookExecutionJob. Format: 'projects/{project}/locations/{location}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }

    #[doc = "Get a reference to the value of field `notebook_execution_job` after provisioning.\n"]
    pub fn notebook_execution_job(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreateNotebookExecutionJobRequest_NotebookExecutionJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notebook_execution_job", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {
    type O = terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {}

impl BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadata {
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadataRef {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs {
    domain: terrars::PrimField<std::string::String>,
    target_network: terrars::PrimField<std::string::String>,
    target_project: terrars::PrimField<std::string::String>,
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs { }

impl terrars
::ToListMappable for ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs {
    type O =
        terrars
        ::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs {
    #[doc = "The DNS name suffix of the zone being peered to, e.g., \"my-internal-domain.corp.\". Must end with a dot."]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = "The VPC network name in the target_project where the DNS zone specified by 'domain' is visible."]
    pub target_network: terrars::PrimField<std::string::String>,
    #[doc =
        "The project ID hosting the Cloud DNS managed zone that contains the 'domain'. The Vertex AI Service Agent requires the dns.peer role on this project."]
    pub target_project: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs {
            domain: self.domain,
            target_network: self.target_network,
            target_project: self.target_project,
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigsRef {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe DNS name suffix of the zone being peered to, e.g., \"my-internal-domain.corp.\". Must end with a dot."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_network` after provisioning.\nThe VPC network name in the target_project where the DNS zone specified by 'domain' is visible."]
    pub fn target_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_project` after provisioning.\nThe project ID hosting the Cloud DNS managed zone that contains the 'domain'. The Vertex AI Service Agent requires the dns.peer role on this project."]
    pub fn target_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_project", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigDynamic {
    dns_peering_configs: std
    ::option
    ::Option<
        terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs>,
    >,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_peering_configs: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs>>,
    dynamic: ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigDynamic,
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {
    #[doc =
        "Set the field `network_attachment`.\nThe name of the Compute Engine [network attachment](https://cloud.google.com/vpc/docs/about-network-attachments) to attach to the resource within the region and user project. To specify this field, you must have already [created a network attachment] (https://cloud.google.com/vpc/docs/create-manage-network-attachments#create-network-attachments). This field is only used for resources using PSC-I."]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_peering_configs`.\n"]
    pub fn set_dns_peering_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_peering_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_peering_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {
    type O = terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {}

impl BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig {
            network_attachment: core::default::Default::default(),
            dns_peering_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigRef {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nThe name of the Compute Engine [network attachment](https://cloud.google.com/vpc/docs/about-network-attachments) to attach to the resource within the region and user project. To specify this field, you must have already [created a network attachment] (https://cloud.google.com/vpc/docs/create-manage-network-attachments#create-network-attachments). This field is only used for resources using PSC-I."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_peering_configs` after provisioning.\n"]
    pub fn dns_peering_configs(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig_DnsPeeringConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_peering_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    gcs_output_directory: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_values: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
    #[doc =
        "Set the field `failure_policy`.\nPossible values: PIPELINE_FAILURE_POLICY_FAIL_SLOW PIPELINE_FAILURE_POLICY_FAIL_FAST"]
    pub fn set_failure_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameter_values`.\nThe runtime parameters of the PipelineJob. The parameters will be passed into PipelineJob.pipeline_spec to replace the placeholders at runtime. This field is used by pipelines built using 'PipelineJob.pipeline_spec.schema_version' 2.1.0, such as pipelines built using Kubeflow Pipelines SDK 1.9 or higher and the v2 DSL."]
    pub fn set_parameter_values(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameter_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
    type O = terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
    #[doc =
        "A path in a Cloud Storage bucket, which will be treated as the root output directory of the pipeline. It is used by the system to generate the paths of output artifacts. The artifact paths are generated with a sub-path pattern '{job_id}/{task_id}/{output_key}' under the specified output directory. The service account specified in this pipeline must have the 'storage.objects.get' and 'storage.objects.create' permissions for this bucket."]
    pub gcs_output_directory: terrars::PrimField<std::string::String>,
}

impl BuildColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig {
            failure_policy: core::default::Default::default(),
            gcs_output_directory: self.gcs_output_directory,
            parameter_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfigRef {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_policy` after provisioning.\nPossible values: PIPELINE_FAILURE_POLICY_FAIL_SLOW PIPELINE_FAILURE_POLICY_FAIL_FAST"]
    pub fn failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcs_output_directory` after provisioning.\nA path in a Cloud Storage bucket, which will be treated as the root output directory of the pipeline. It is used by the system to generate the paths of output artifacts. The artifact paths are generated with a sub-path pattern '{job_id}/{task_id}/{output_key}' under the specified output directory. The service account specified in this pipeline must have the 'storage.objects.get' and 'storage.objects.create' permissions for this bucket."]
    pub fn gcs_output_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_output_directory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameter_values` after provisioning.\nThe runtime parameters of the PipelineJob. The parameters will be passed into PipelineJob.pipeline_spec to replace the placeholders at runtime. This field is used by pipelines built using 'PipelineJob.pipeline_spec.schema_version' 2.1.0, such as pipelines built using Kubeflow Pipelines SDK 1.9 or higher and the v2 DSL."]
    pub fn parameter_values(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameter_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreatePipelineJobRequest_PipelineJobDynamic {
    encryption_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>>,
    psc_interface_config: std
    ::option
    ::Option<terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig>>,
    runtime_config: std
    ::option
    ::Option<terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig>>,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preflight_validations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_interface_config: std
    ::option
    ::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_config: std::option::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig>>,
    dynamic: ColabSchedule_CreatePipelineJobRequest_PipelineJobDynamic,
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJob {
    #[doc =
        "Set the field `display_name`.\nThe display name of the Pipeline. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels with user-defined metadata to organize PipelineJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. Note there is some reserved label key for Vertex AI Pipelines. - 'vertex-ai-pipelines-run-billing-id', user set value will get overrided."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the Pipeline Job's workload should be peered. For example, 'projects/12345/global/networks/myVPC'. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form 'projects/{project}/global/networks/{network}'. Where {project} is a project number, as in '12345', and {network} is a network name. Private services access must already be configured for the network. Pipeline job will apply the network configuration to the Google Cloud resources being launched, if applied, such as Vertex AI Training or Dataflow job. If left unspecified, the workload is not peered with any network."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pipeline_spec`.\nA compiled definition of a pipeline, represented as a 'JSON' object. Defines the structure of the pipeline, including its components, tasks, and parameters. This specification is generated by compiling a pipeline function defined in 'Python' using the 'Kubeflow Pipelines SDK'."]
    pub fn set_pipeline_spec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pipeline_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preflight_validations`.\nWhether to do component level validations before job creation."]
    pub fn set_preflight_validations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preflight_validations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_ip_ranges`.\nA list of names for the reserved ip ranges under the VPC network that can be used for this Pipeline Job's workload. If set, we will deploy the Pipeline Job's workload within the provided ip ranges. Otherwise, the job will be deployed to any ip ranges under the provided VPC network. Example: ['vertex-ai-ip-range']."]
    pub fn set_reserved_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.reserved_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe service account that the pipeline workload runs as. If not specified, the Compute Engine default service account in the project will be used. See https://cloud.google.com/compute/docs/access/service-accounts#default_service_account Users starting the pipeline must have the 'iam.serviceAccounts.actAs' permission on this service account."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `template_uri`.\nA template uri from where the PipelineJob.pipeline_spec, if empty, will be downloaded. Currently, only uri from Vertex Template Registry & Gallery is supported. Reference to https://cloud.google.com/vertex-ai/docs/pipelines/create-pipeline-template."]
    pub fn set_template_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.template_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_interface_config`.\n"]
    pub fn set_psc_interface_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_interface_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_interface_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.runtime_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.runtime_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreatePipelineJobRequest_PipelineJob {
    type O = terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest_PipelineJob {}

impl BuildColabSchedule_CreatePipelineJobRequest_PipelineJob {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest_PipelineJob {
        ColabSchedule_CreatePipelineJobRequest_PipelineJob {
            display_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            network: core::default::Default::default(),
            pipeline_spec: core::default::Default::default(),
            preflight_validations: core::default::Default::default(),
            reserved_ip_ranges: core::default::Default::default(),
            service_account: core::default::Default::default(),
            template_uri: core::default::Default::default(),
            encryption_spec: core::default::Default::default(),
            psc_interface_config: core::default::Default::default(),
            runtime_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequest_PipelineJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequest_PipelineJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabSchedule_CreatePipelineJobRequest_PipelineJobRef {
        ColabSchedule_CreatePipelineJobRequest_PipelineJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequest_PipelineJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nPipeline creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Pipeline. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\nPipeline end time."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize PipelineJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. Note there is some reserved label key for Vertex AI Pipelines. - 'vertex-ai-pipelines-run-billing-id', user set value will get overrided."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the PipelineJob."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the Pipeline Job's workload should be peered. For example, 'projects/12345/global/networks/myVPC'. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form 'projects/{project}/global/networks/{network}'. Where {project} is a project number, as in '12345', and {network} is a network name. Private services access must already be configured for the network. Pipeline job will apply the network configuration to the Google Cloud resources being launched, if applied, such as Vertex AI Training or Dataflow job. If left unspecified, the workload is not peered with any network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_spec` after provisioning.\nA compiled definition of a pipeline, represented as a 'JSON' object. Defines the structure of the pipeline, including its components, tasks, and parameters. This specification is generated by compiling a pipeline function defined in 'Python' using the 'Kubeflow Pipelines SDK'."]
    pub fn pipeline_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_spec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preflight_validations` after provisioning.\nWhether to do component level validations before job creation."]
    pub fn preflight_validations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preflight_validations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of names for the reserved ip ranges under the VPC network that can be used for this Pipeline Job's workload. If set, we will deploy the Pipeline Job's workload within the provided ip ranges. Otherwise, the job will be deployed to any ip ranges under the provided VPC network. Example: ['vertex-ai-ip-range']."]
    pub fn reserved_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule_name` after provisioning.\nThe schedule resource name. Only returned if the Pipeline is created by Schedule API."]
    pub fn schedule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account that the pipeline workload runs as. If not specified, the Compute Engine default service account in the project will be used. See https://cloud.google.com/compute/docs/access/service-accounts#default_service_account Users starting the pipeline must have the 'iam.serviceAccounts.actAs' permission on this service account."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nPipeline start time."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nPossible values: PIPELINE_STATE_QUEUED PIPELINE_STATE_PENDING PIPELINE_STATE_RUNNING PIPELINE_STATE_SUCCEEDED PIPELINE_STATE_FAILED PIPELINE_STATE_CANCELLING PIPELINE_STATE_CANCELLED PIPELINE_STATE_PAUSED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `template_metadata` after provisioning.\nPipeline template metadata if PipelineJob.template_uri is from supported template registry. Currently, the only supported registry is Artifact Registry."]
    pub fn template_metadata(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJob_TemplateMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `template_uri` after provisioning.\nA template uri from where the PipelineJob.pipeline_spec, if empty, will be downloaded. Currently, only uri from Vertex Template Registry & Gallery is supported. Reference to https://cloud.google.com/vertex-ai/docs/pipelines/create-pipeline-template."]
    pub fn template_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when this PipelineJob was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJob_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJob_PscInterfaceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_config", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(
        &self,
    ) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJob_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabSchedule_CreatePipelineJobRequestDynamic {
    pipeline_job: std::option::Option<terrars::DynamicBlock<ColabSchedule_CreatePipelineJobRequest_PipelineJob>>,
}

#[derive(Serialize)]
pub struct ColabSchedule_CreatePipelineJobRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_job: std::option::Option<std::vec::Vec<ColabSchedule_CreatePipelineJobRequest_PipelineJob>>,
    dynamic: ColabSchedule_CreatePipelineJobRequestDynamic,
}

impl ColabSchedule_CreatePipelineJobRequest {
    #[doc =
        "Set the field `parent`.\nThe resource name of the Location to create the PipelineJob in. Format: 'projects/{project}/locations/{location}'"]
    pub fn set_parent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pipeline_job`.\n"]
    pub fn set_pipeline_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest_PipelineJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pipeline_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_job = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabSchedule_CreatePipelineJobRequest {
    type O = terrars::BlockAssignable<ColabSchedule_CreatePipelineJobRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_CreatePipelineJobRequest {}

impl BuildColabSchedule_CreatePipelineJobRequest {
    pub fn build(self) -> ColabSchedule_CreatePipelineJobRequest {
        ColabSchedule_CreatePipelineJobRequest {
            parent: core::default::Default::default(),
            pipeline_job: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_CreatePipelineJobRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_CreatePipelineJobRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabSchedule_CreatePipelineJobRequestRef {
        ColabSchedule_CreatePipelineJobRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_CreatePipelineJobRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the Location to create the PipelineJob in. Format: 'projects/{project}/locations/{location}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_job_id` after provisioning.\nThe ID to use for the PipelineJob, which will become the final component of the PipelineJob name. If not provided, an ID will be automatically generated. This value should be less than 128 characters, and valid characters are '/a-z-/'."]
    pub fn pipeline_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_job_id", self.base))
    }

    #[doc = "Get a reference to the value of field `pipeline_job` after provisioning.\n"]
    pub fn pipeline_job(&self) -> terrars::ListRef<ColabSchedule_CreatePipelineJobRequest_PipelineJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pipeline_job", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabSchedule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabSchedule_Timeouts {
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

impl terrars::ToListMappable for ColabSchedule_Timeouts {
    type O = terrars::BlockAssignable<ColabSchedule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabSchedule_Timeouts {}

impl BuildColabSchedule_Timeouts {
    pub fn build(self) -> ColabSchedule_Timeouts {
        ColabSchedule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabSchedule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabSchedule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabSchedule_TimeoutsRef {
        ColabSchedule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabSchedule_TimeoutsRef {
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
