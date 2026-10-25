#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataPipelinePipelineData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_sources: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduler_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    state: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_info: std::option::Option<std::vec::Vec<DataPipelinePipeline_ScheduleInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataPipelinePipeline_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload: std::option::Option<std::vec::Vec<DataPipelinePipeline_Workload>>,
    dynamic: DataPipelinePipelineDynamic,
}

struct DataPipelinePipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPipelinePipelineData_>,
}

#[derive(Clone)]
pub struct DataPipelinePipeline(std::rc::Rc<DataPipelinePipeline_>);

impl DataPipelinePipeline {
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
        "Set the field `display_name`.\nThe display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_)."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pipeline_sources`.\nThe sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_pipeline_sources(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().pipeline_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nA reference to the region"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scheduler_service_account_email`.\nOptional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used."]
    pub fn set_scheduler_service_account_email(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().scheduler_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_info`.\n"]
    pub fn set_schedule_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataPipelinePipeline_ScheduleInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataPipelinePipeline_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workload`.\n"]
    pub fn set_workload(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataPipelinePipeline_Workload>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the pipeline was initially created. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_count` after provisioning.\nNumber of jobs."]
    pub fn job_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_time` after provisioning.\nThe timestamp when the pipeline was last modified. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\n\"The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID.\"\n\"- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects.\"\n\"LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions.\"\n\"PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location.\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_sources` after provisioning.\nThe sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn pipeline_sources(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pipeline_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scheduler_service_account_email` after provisioning.\nOptional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used."]
    pub fn scheduler_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduler_service_account_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state Possible values: [\"STATE_UNSPECIFIED\", \"STATE_RESUMING\", \"STATE_ACTIVE\", \"STATE_STOPPING\", \"STATE_ARCHIVED\", \"STATE_PAUSED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype Possible values: [\"PIPELINE_TYPE_UNSPECIFIED\", \"PIPELINE_TYPE_BATCH\", \"PIPELINE_TYPE_STREAMING\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> terrars::ListRef<DataPipelinePipeline_ScheduleInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataPipelinePipeline_TimeoutsRef {
        DataPipelinePipeline_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload` after provisioning.\n"]
    pub fn workload(&self) -> terrars::ListRef<DataPipelinePipeline_WorkloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload", self.extract_ref()))
    }
}

impl terrars::Referable for DataPipelinePipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataPipelinePipeline { }

impl terrars::ToListMappable for DataPipelinePipeline {
    type O = terrars::ListRef<DataPipelinePipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataPipelinePipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_pipeline_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPipelinePipeline {
    pub tf_id: std::string::String,
    #[doc =
        "\"The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID.\"\n\"- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects.\"\n\"LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions.\"\n\"PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location.\""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state Possible values: [\"STATE_UNSPECIFIED\", \"STATE_RESUMING\", \"STATE_ACTIVE\", \"STATE_STOPPING\", \"STATE_ARCHIVED\", \"STATE_PAUSED\"]"]
    pub state: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype Possible values: [\"PIPELINE_TYPE_UNSPECIFIED\", \"PIPELINE_TYPE_BATCH\", \"PIPELINE_TYPE_STREAMING\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataPipelinePipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPipelinePipeline {
        let out = DataPipelinePipeline(std::rc::Rc::new(DataPipelinePipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPipelinePipelineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                pipeline_sources: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                scheduler_service_account_email: core::default::Default::default(),
                state: self.state,
                type_: self.type_,
                schedule_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workload: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataPipelinePipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPipelinePipelineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the pipeline was initially created. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `job_count` after provisioning.\nNumber of jobs."]
    pub fn job_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_update_time` after provisioning.\nThe timestamp when the pipeline was last modified. Set by the Data Pipelines service.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\n\"The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID.\"\n\"- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects.\"\n\"LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions.\"\n\"PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location.\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pipeline_sources` after provisioning.\nThe sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn pipeline_sources(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.pipeline_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scheduler_service_account_email` after provisioning.\nOptional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used."]
    pub fn scheduler_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.scheduler_service_account_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state Possible values: [\"STATE_UNSPECIFIED\", \"STATE_RESUMING\", \"STATE_ACTIVE\", \"STATE_STOPPING\", \"STATE_ARCHIVED\", \"STATE_PAUSED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype Possible values: [\"PIPELINE_TYPE_UNSPECIFIED\", \"PIPELINE_TYPE_BATCH\", \"PIPELINE_TYPE_STREAMING\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_info` after provisioning.\n"]
    pub fn schedule_info(&self) -> terrars::ListRef<DataPipelinePipeline_ScheduleInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataPipelinePipeline_TimeoutsRef {
        DataPipelinePipeline_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload` after provisioning.\n"]
    pub fn workload(&self) -> terrars::ListRef<DataPipelinePipeline_WorkloadRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataPipelinePipelineDynamic {
    schedule_info: std::option::Option<terrars::DynamicBlock<DataPipelinePipeline_ScheduleInfo>>,
    workload: std::option::Option<terrars::DynamicBlock<DataPipelinePipeline_Workload>>,
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_ScheduleInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPipelinePipeline_ScheduleInfo {
    #[doc =
        "Set the field `schedule`.\nUnix-cron format of the schedule. This information is retrieved from the linked Cloud Scheduler."]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nTimezone ID. This matches the timezone IDs used by the Cloud Scheduler API. If empty, UTC time is assumed."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPipelinePipeline_ScheduleInfo {
    type O = terrars::BlockAssignable<DataPipelinePipeline_ScheduleInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_ScheduleInfo {}

impl BuildDataPipelinePipeline_ScheduleInfo {
    pub fn build(self) -> DataPipelinePipeline_ScheduleInfo {
        DataPipelinePipeline_ScheduleInfo {
            schedule: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_ScheduleInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_ScheduleInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelinePipeline_ScheduleInfoRef {
        DataPipelinePipeline_ScheduleInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_ScheduleInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `next_job_time` after provisioning.\nWhen the next Scheduler job is going to run.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_job_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_job_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nUnix-cron format of the schedule. This information is retrieved from the linked Cloud Scheduler."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nTimezone ID. This matches the timezone IDs used by the Cloud Scheduler API. If empty, UTC time is assumed."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPipelinePipeline_Timeouts {
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

impl terrars::ToListMappable for DataPipelinePipeline_Timeouts {
    type O = terrars::BlockAssignable<DataPipelinePipeline_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Timeouts {}

impl BuildDataPipelinePipeline_Timeouts {
    pub fn build(self) -> DataPipelinePipeline_Timeouts {
        DataPipelinePipeline_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelinePipeline_TimeoutsRef {
        DataPipelinePipeline_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_TimeoutsRef {
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

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_experiments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_streaming_engine: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flexrs_goal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temp_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {
    #[doc = "Set the field `additional_experiments`.\nAdditional experiment flags for the job."]
    pub fn set_additional_experiments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_experiments = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `additional_user_labels`.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_additional_user_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_streaming_engine`.\nWhether to enable Streaming Engine for the job."]
    pub fn set_enable_streaming_engine(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_streaming_engine = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `flexrs_goal`.\nSet FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexResourceSchedulingGoal Possible values: [\"FLEXRS_UNSPECIFIED\", \"FLEXRS_SPEED_OPTIMIZED\", \"FLEXRS_COST_OPTIMIZED\"]"]
    pub fn set_flexrs_goal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flexrs_goal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_configuration`.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn set_ip_configuration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_workers`.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn set_max_workers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_workers`.\nThe initial number of Compute Engine instances for the job."]
    pub fn set_num_workers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\nThe email address of the service account to run the job as."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temp_location`.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn set_temp_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temp_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_region`.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn set_worker_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_zone`.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn set_worker_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {
    type O =
        terrars
        ::BlockAssignable<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {}

impl BuildDataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {
    pub fn build(self) -> DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {
        DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment {
            additional_experiments: core::default::Default::default(),
            additional_user_labels: core::default::Default::default(),
            enable_streaming_engine: core::default::Default::default(),
            flexrs_goal: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            kms_key_name: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_workers: core::default::Default::default(),
            network: core::default::Default::default(),
            num_workers: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            temp_location: core::default::Default::default(),
            worker_region: core::default::Default::default(),
            worker_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_EnvironmentRef {
        DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_experiments` after provisioning.\nAdditional experiment flags for the job."]
    pub fn additional_experiments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_experiments", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_user_labels` after provisioning.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn additional_user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_user_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_streaming_engine` after provisioning.\nWhether to enable Streaming Engine for the job."]
    pub fn enable_streaming_engine(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_streaming_engine", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flexrs_goal` after provisioning.\nSet FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexResourceSchedulingGoal Possible values: [\"FLEXRS_UNSPECIFIED\", \"FLEXRS_SPEED_OPTIMIZED\", \"FLEXRS_COST_OPTIMIZED\"]"]
    pub fn flexrs_goal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flexrs_goal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_configuration` after provisioning.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn ip_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_workers` after provisioning.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_workers` after provisioning.\nThe initial number of Compute Engine instances for the job."]
    pub fn num_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_workers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe email address of the service account to run the job as."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temp_location` after provisioning.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn temp_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temp_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_region` after provisioning.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn worker_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_zone` after provisioning.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn worker_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterDynamic {
    environment: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment>,
    >,
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_spec_gcs_path: std::option::Option<terrars::PrimField<std::string::String>>,
    job_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_name_mappings: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std
    ::option
    ::Option<std::vec::Vec<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment>>,
    dynamic: DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterDynamic,
}

impl DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
    #[doc =
        "Set the field `container_spec_gcs_path`.\nCloud Storage path to a file with a JSON-serialized ContainerSpec as content."]
    pub fn set_container_spec_gcs_path(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.container_spec_gcs_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `launch_options`.\nLaunch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_launch_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.launch_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameters`.\n'The parameters for the Flex Template. Example: {\"numWorkers\":\"5\"}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transform_name_mappings`.\n'Use this to pass transform name mappings for streaming update jobs. Example: {\"oldTransformName\":\"newTransformName\",...}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_transform_name_mappings(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.transform_name_mappings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nSet this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_Environment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
    type O = terrars::BlockAssignable<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
    #[doc =
        "The job name to use for the created job. For an update job request, the job name should be the same as the existing running job."]
    pub job_name: terrars::PrimField<std::string::String>,
}

impl BuildDataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
    pub fn build(self) -> DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
        DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter {
            container_spec_gcs_path: core::default::Default::default(),
            job_name: self.job_name,
            launch_options: core::default::Default::default(),
            parameters: core::default::Default::default(),
            transform_name_mappings: core::default::Default::default(),
            update: core::default::Default::default(),
            environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterRef {
        DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_spec_gcs_path` after provisioning.\nCloud Storage path to a file with a JSON-serialized ContainerSpec as content."]
    pub fn container_spec_gcs_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_spec_gcs_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `job_name` after provisioning.\nThe job name to use for the created job. For an update job request, the job name should be the same as the existing running job."]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `launch_options` after provisioning.\nLaunch options for this Flex Template job. This is a common set of options across languages and templates. This should not be used to pass job parameters.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn launch_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.launch_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\n'The parameters for the Flex Template. Example: {\"numWorkers\":\"5\"}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transform_name_mappings` after provisioning.\n'Use this to pass transform name mappings for streaming update jobs. Example: {\"oldTransformName\":\"newTransformName\",...}'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn transform_name_mappings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.transform_name_mappings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nSet this to true if you are sending a request to update a running streaming job. When set, the job name should be the same as the running job."]
    pub fn update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> terrars::ListRef<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequestDynamic {
    launch_parameter: std
    ::option
    ::Option<terrars::DynamicBlock<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter>>,
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
    location: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_parameter: std
    ::option
    ::Option<std::vec::Vec<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter>>,
    dynamic: DataPipelinePipeline_Workload_DataflowFlexTemplateRequestDynamic,
}

impl DataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
    #[doc =
        "Set the field `validate_only`.\nIf true, the request is validated but not actually executed. Defaults to false."]
    pub fn set_validate_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.validate_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_parameter`.\n"]
    pub fn set_launch_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
    type O = terrars::BlockAssignable<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
    #[doc = "The regional endpoint to which to direct the request. For example, us-central1, us-west1."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the Cloud Platform project that the job belongs to."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
    pub fn build(self) -> DataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
        DataPipelinePipeline_Workload_DataflowFlexTemplateRequest {
            location: self.location,
            project_id: self.project_id,
            validate_only: core::default::Default::default(),
            launch_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_Workload_DataflowFlexTemplateRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_Workload_DataflowFlexTemplateRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelinePipeline_Workload_DataflowFlexTemplateRequestRef {
        DataPipelinePipeline_Workload_DataflowFlexTemplateRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_Workload_DataflowFlexTemplateRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe regional endpoint to which to direct the request. For example, us-central1, us-west1."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the Cloud Platform project that the job belongs to."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validate_only` after provisioning.\nIf true, the request is validated but not actually executed. Defaults to false."]
    pub fn validate_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validate_only", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_parameter` after provisioning.\n"]
    pub fn launch_parameter(
        &self,
    ) -> terrars::ListRef<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest_LaunchParameterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_experiments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_temp_dir_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_streaming_engine: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temp_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {
    #[doc = "Set the field `additional_experiments`.\nAdditional experiment flags for the job."]
    pub fn set_additional_experiments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_experiments = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `additional_user_labels`.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_additional_user_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bypass_temp_dir_validation`.\nWhether to bypass the safety checks for the job's temporary directory. Use with caution."]
    pub fn set_bypass_temp_dir_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.bypass_temp_dir_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_streaming_engine`.\nWhether to enable Streaming Engine for the job."]
    pub fn set_enable_streaming_engine(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_streaming_engine = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_configuration`.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn set_ip_configuration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_workers`.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn set_max_workers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_workers`.\nThe initial number of Compute Engine instances for the job."]
    pub fn set_num_workers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\nThe email address of the service account to run the job as."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temp_location`.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn set_temp_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temp_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_region`.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn set_worker_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_zone`.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn set_worker_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {
    type O =
        terrars
        ::BlockAssignable<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {}

impl BuildDataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {
    pub fn build(self) -> DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {
        DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment {
            additional_experiments: core::default::Default::default(),
            additional_user_labels: core::default::Default::default(),
            bypass_temp_dir_validation: core::default::Default::default(),
            enable_streaming_engine: core::default::Default::default(),
            ip_configuration: core::default::Default::default(),
            kms_key_name: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            max_workers: core::default::Default::default(),
            network: core::default::Default::default(),
            num_workers: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            temp_location: core::default::Default::default(),
            worker_region: core::default::Default::default(),
            worker_zone: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_EnvironmentRef {
        DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_experiments` after provisioning.\nAdditional experiment flags for the job."]
    pub fn additional_experiments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_experiments", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_user_labels` after provisioning.\nAdditional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.\n'Example: { \"name\": \"wrench\", \"mass\": \"1kg\", \"count\": \"3\" }.'\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn additional_user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_user_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bypass_temp_dir_validation` after provisioning.\nWhether to bypass the safety checks for the job's temporary directory. Use with caution."]
    pub fn bypass_temp_dir_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_temp_dir_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_streaming_engine` after provisioning.\nWhether to enable Streaming Engine for the job."]
    pub fn enable_streaming_engine(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_streaming_engine", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_configuration` after provisioning.\nConfiguration for VM IPs.\nhttps://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration Possible values: [\"WORKER_IP_UNSPECIFIED\", \"WORKER_IP_PUBLIC\", \"WORKER_IP_PRIVATE\"]"]
    pub fn ip_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\n'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to use for the job. Defaults to the value from the template if not specified."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_workers` after provisioning.\nThe maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000."]
    pub fn max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_workers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nNetwork to which VMs will be assigned. If empty or unspecified, the service will use the network \"default\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_workers` after provisioning.\nThe initial number of Compute Engine instances for the job."]
    pub fn num_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_workers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe email address of the service account to run the job as."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nSubnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form \"https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK\" or \"regions/REGION/subnetworks/SUBNETWORK\". If the subnetwork is located in a Shared VPC network, you must use the complete URL."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temp_location` after provisioning.\nThe Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://."]
    pub fn temp_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temp_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_region` after provisioning.\nThe Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1\". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region."]
    pub fn worker_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_zone` after provisioning.\nThe Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. \"us-west1-a\". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence."]
    pub fn worker_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersDynamic {
    environment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment>,
    >,
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
    job_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_name_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std
    ::option
    ::Option<std::vec::Vec<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment>>,
    dynamic: DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersDynamic,
}

impl DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
    #[doc =
        "Set the field `parameters`.\nThe runtime parameters to pass to the job.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transform_name_mapping`.\nMap of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. Only applicable when updating a pipeline.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn set_transform_name_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.transform_name_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nIf set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_Environment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
    type O = terrars::BlockAssignable<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
    #[doc = "The job name to use for the created job."]
    pub job_name: terrars::PrimField<std::string::String>,
}

impl BuildDataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
    pub fn build(self) -> DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
        DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters {
            job_name: self.job_name,
            parameters: core::default::Default::default(),
            transform_name_mapping: core::default::Default::default(),
            update: core::default::Default::default(),
            environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersRef {
        DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `job_name` after provisioning.\nThe job name to use for the created job."]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nThe runtime parameters to pass to the job.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transform_name_mapping` after provisioning.\nMap of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. Only applicable when updating a pipeline.\n'An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.'"]
    pub fn transform_name_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.transform_name_mapping", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nIf set, replace the existing pipeline with the name specified by jobName with this pipeline, preserving state."]
    pub fn update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> terrars
    ::ListRef<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestDynamic {
    launch_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters>>,
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_parameters: std
    ::option
    ::Option<std::vec::Vec<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters>>,
    dynamic: DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestDynamic,
}

impl DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
    #[doc =
        "Set the field `gcs_path`.\nA Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'."]
    pub fn set_gcs_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe regional endpoint to which to direct the request."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validate_only`.\n"]
    pub fn set_validate_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.validate_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_parameters`.\n"]
    pub fn set_launch_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
    type O = terrars::BlockAssignable<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
    #[doc = "The ID of the Cloud Platform project that the job belongs to."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
    pub fn build(self) -> DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
        DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest {
            gcs_path: core::default::Default::default(),
            location: core::default::Default::default(),
            project_id: self.project_id,
            validate_only: core::default::Default::default(),
            launch_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestRef {
        DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcs_path` after provisioning.\nA Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with 'gs://'."]
    pub fn gcs_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe regional endpoint to which to direct the request."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the Cloud Platform project that the job belongs to."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `validate_only` after provisioning.\n"]
    pub fn validate_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validate_only", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_parameters` after provisioning.\n"]
    pub fn launch_parameters(
        &self,
    ) -> terrars::ListRef<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest_LaunchParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataPipelinePipeline_WorkloadDynamic {
    dataflow_flex_template_request: std
    ::option
    ::Option<terrars::DynamicBlock<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest>>,
    dataflow_launch_template_request: std
    ::option
    ::Option<terrars::DynamicBlock<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest>>,
}

#[derive(Serialize)]
pub struct DataPipelinePipeline_Workload {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataflow_flex_template_request: std
    ::option
    ::Option<std::vec::Vec<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataflow_launch_template_request: std
    ::option
    ::Option<std::vec::Vec<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest>>,
    dynamic: DataPipelinePipeline_WorkloadDynamic,
}

impl DataPipelinePipeline_Workload {
    #[doc = "Set the field `dataflow_flex_template_request`.\n"]
    pub fn set_dataflow_flex_template_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataPipelinePipeline_Workload_DataflowFlexTemplateRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataflow_flex_template_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataflow_flex_template_request = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataflow_launch_template_request`.\n"]
    pub fn set_dataflow_launch_template_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataflow_launch_template_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataflow_launch_template_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataPipelinePipeline_Workload {
    type O = terrars::BlockAssignable<DataPipelinePipeline_Workload>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPipelinePipeline_Workload {}

impl BuildDataPipelinePipeline_Workload {
    pub fn build(self) -> DataPipelinePipeline_Workload {
        DataPipelinePipeline_Workload {
            dataflow_flex_template_request: core::default::Default::default(),
            dataflow_launch_template_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPipelinePipeline_WorkloadRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPipelinePipeline_WorkloadRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPipelinePipeline_WorkloadRef {
        DataPipelinePipeline_WorkloadRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPipelinePipeline_WorkloadRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataflow_flex_template_request` after provisioning.\n"]
    pub fn dataflow_flex_template_request(
        &self,
    ) -> terrars::ListRef<DataPipelinePipeline_Workload_DataflowFlexTemplateRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataflow_flex_template_request", self.base))
    }

    #[doc = "Get a reference to the value of field `dataflow_launch_template_request` after provisioning.\n"]
    pub fn dataflow_launch_template_request(
        &self,
    ) -> terrars::ListRef<DataPipelinePipeline_Workload_DataflowLaunchTemplateRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataflow_launch_template_request", self.base))
    }
}
