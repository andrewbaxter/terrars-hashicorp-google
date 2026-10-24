use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocWorkflowTemplateData {
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
    dag_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jobs: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocWorkflowTemplateTimeoutsEl>,
    dynamic: DataprocWorkflowTemplateDynamic,
}

struct DataprocWorkflowTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocWorkflowTemplateData>,
}

#[derive(Clone)]
pub struct DataprocWorkflowTemplate(std::rc::Rc<DataprocWorkflowTemplate_>);

impl DataprocWorkflowTemplate {
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
        "Set the field `dag_timeout`.\nOptional. Timeout duration for the DAG of jobs, expressed in seconds (see [JSON representation of duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes (\"600s\") to 24 hours (\"86400s\"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a [managed cluster](/dataproc/docs/concepts/workflows/using-workflows#configuring_or_selecting_a_cluster), the cluster is deleted."]
    pub fn set_dag_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dag_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance. Label **keys** must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nOutput only. The current version of this workflow template."]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jobs`.\n"]
    pub fn set_jobs(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jobs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jobs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplatePlacementEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocWorkflowTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time template was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dag_timeout` after provisioning.\nOptional. Timeout duration for the DAG of jobs, expressed in seconds (see [JSON representation of duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes (\"600s\") to 24 hours (\"86400s\"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a [managed cluster](/dataproc/docs/concepts/workflows/using-workflows#configuring_or_selecting_a_cluster), the cluster is deleted."]
    pub fn dag_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_timeout", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance. Label **keys** must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. * For `projects.regions.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}` * For `projects.locations.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time template was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOutput only. The current version of this workflow template."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jobs` after provisioning.\n"]
    pub fn jobs(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DataprocWorkflowTemplateParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocWorkflowTemplateTimeoutsElRef {
        DataprocWorkflowTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocWorkflowTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocWorkflowTemplate { }

impl terrars::ToListMappable for DataprocWorkflowTemplate {
    type O = terrars::ListRef<DataprocWorkflowTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocWorkflowTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_workflow_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocWorkflowTemplate {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. * For `projects.regions.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}` * For `projects.locations.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocWorkflowTemplate {
        let out = DataprocWorkflowTemplate(std::rc::Rc::new(DataprocWorkflowTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocWorkflowTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dag_timeout: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                version: core::default::Default::default(),
                jobs: core::default::Default::default(),
                parameters: core::default::Default::default(),
                placement: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocWorkflowTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocWorkflowTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time template was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dag_timeout` after provisioning.\nOptional. Timeout duration for the DAG of jobs, expressed in seconds (see [JSON representation of duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes (\"600s\") to 24 hours (\"86400s\"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a [managed cluster](/dataproc/docs/concepts/workflows/using-workflows#configuring_or_selecting_a_cluster), the cluster is deleted."]
    pub fn dag_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dag_timeout", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance. Label **keys** must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63 characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. * For `projects.regions.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}` * For `projects.locations.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time template was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOutput only. The current version of this workflow template."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jobs` after provisioning.\n"]
    pub fn jobs(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DataprocWorkflowTemplateParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocWorkflowTemplateTimeoutsElRef {
        DataprocWorkflowTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElHadoopJobElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElHadoopJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_jar_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl>>,
    dynamic: DataprocWorkflowTemplateJobsElHadoopJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElHadoopJobEl {
    #[doc =
        "Set the field `archive_uris`.\nOptional. HCFS URIs of archives to be extracted in the working directory of Hadoop drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, or .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `-libjars` or `-Dfoo=bar`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nOptional. HCFS (Hadoop Compatible Filesystem) URIs of files to be copied to the working directory of Hadoop drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nOptional. Jar file URIs to add to the CLASSPATHs of the Hadoop driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe name of the driver's main class. The jar file containing the class must be in the default CLASSPATH or specified in `jar_file_uris`."]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_jar_file_uri`.\nThe HCFS URI of the jar file containing the main class. Examples: 'gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar' 'hdfs:/tmp/test-samples/custom-wordcount.jar' 'file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar'"]
    pub fn set_main_jar_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_jar_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure Hadoop. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElHadoopJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHadoopJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElHadoopJobEl {}

impl BuildDataprocWorkflowTemplateJobsElHadoopJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElHadoopJobEl {
        DataprocWorkflowTemplateJobsElHadoopJobEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElHadoopJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElHadoopJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElHadoopJobElRef {
        DataprocWorkflowTemplateJobsElHadoopJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElHadoopJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nOptional. HCFS URIs of archives to be extracted in the working directory of Hadoop drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, or .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `-libjars` or `-Dfoo=bar`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nOptional. HCFS (Hadoop Compatible Filesystem) URIs of files to be copied to the working directory of Hadoop drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. Jar file URIs to add to the CLASSPATHs of the Hadoop driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver's main class. The jar file containing the class must be in the default CLASSPATH or specified in `jar_file_uris`."]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of the jar file containing the main class. Examples: 'gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar' 'hdfs:/tmp/test-samples/custom-wordcount.jar' 'file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar'"]
    pub fn main_jar_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_jar_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure Hadoop. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElHadoopJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElHiveJobElQueryListEl {
    queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateJobsElHiveJobElQueryListEl { }

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElHiveJobElQueryListEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHiveJobElQueryListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElHiveJobElQueryListEl {
    #[doc =
        "Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplateJobsElHiveJobElQueryListEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElHiveJobElQueryListEl {
        DataprocWorkflowTemplateJobsElHiveJobElQueryListEl { queries: self.queries }
    }
}

pub struct DataprocWorkflowTemplateJobsElHiveJobElQueryListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElHiveJobElQueryListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElHiveJobElQueryListElRef {
        DataprocWorkflowTemplateJobsElHiveJobElQueryListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElHiveJobElQueryListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `queries` after provisioning.\nRequired. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub fn queries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElHiveJobElDynamic {
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElHiveJobElQueryListEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElHiveJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_on_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElHiveJobElQueryListEl>>,
    dynamic: DataprocWorkflowTemplateJobsElHiveJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElHiveJobEl {
    #[doc =
        "Set the field `continue_on_failure`.\nOptional. Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries."]
    pub fn set_continue_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continue_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains Hive queries."]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nOptional. Mapping of query variable names to values (equivalent to the Hive command: `SET name=\"value\";`)."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_list`.\n"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHiveJobElQueryListEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElHiveJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHiveJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElHiveJobEl {}

impl BuildDataprocWorkflowTemplateJobsElHiveJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElHiveJobEl {
        DataprocWorkflowTemplateJobsElHiveJobEl {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            query_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElHiveJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElHiveJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElHiveJobElRef {
        DataprocWorkflowTemplateJobsElHiveJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElHiveJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `continue_on_failure` after provisioning.\nOptional. Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries."]
    pub fn continue_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continue_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains Hive queries."]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nOptional. Mapping of query variable names to values (equivalent to the Hive command: `SET name=\"value\";`)."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElHiveJobElQueryListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElPigJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPigJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElPigJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElPigJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPigJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPigJobElQueryListEl {
    queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateJobsElPigJobElQueryListEl { }

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPigJobElQueryListEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPigJobElQueryListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPigJobElQueryListEl {
    #[doc =
        "Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplateJobsElPigJobElQueryListEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPigJobElQueryListEl {
        DataprocWorkflowTemplateJobsElPigJobElQueryListEl { queries: self.queries }
    }
}

pub struct DataprocWorkflowTemplateJobsElPigJobElQueryListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPigJobElQueryListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElPigJobElQueryListElRef {
        DataprocWorkflowTemplateJobsElPigJobElQueryListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPigJobElQueryListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `queries` after provisioning.\nRequired. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub fn queries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElPigJobElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl>>,
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPigJobElQueryListEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPigJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_on_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPigJobElQueryListEl>>,
    dynamic: DataprocWorkflowTemplateJobsElPigJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElPigJobEl {
    #[doc =
        "Set the field `continue_on_failure`.\nOptional. Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries."]
    pub fn set_continue_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continue_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains the Pig queries."]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nOptional. Mapping of query variable names to values (equivalent to the Pig command: `name=[value]`)."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPigJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_list`.\n"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPigJobElQueryListEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPigJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPigJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPigJobEl {}

impl BuildDataprocWorkflowTemplateJobsElPigJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPigJobEl {
        DataprocWorkflowTemplateJobsElPigJobEl {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            query_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElPigJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPigJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElPigJobElRef {
        DataprocWorkflowTemplateJobsElPigJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPigJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `continue_on_failure` after provisioning.\nOptional. Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries."]
    pub fn continue_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continue_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains the Pig queries."]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nOptional. Mapping of query variable names to values (equivalent to the Pig command: `name=[value]`)."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPigJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPigJobElQueryListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl {
    queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl { }

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPrestoJobElQueryListEl {
    #[doc =
        "Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplateJobsElPrestoJobElQueryListEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl {
        DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl { queries: self.queries }
    }
}

pub struct DataprocWorkflowTemplateJobsElPrestoJobElQueryListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPrestoJobElQueryListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElPrestoJobElQueryListElRef {
        DataprocWorkflowTemplateJobsElPrestoJobElQueryListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPrestoJobElQueryListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `queries` after provisioning.\nRequired. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub fn queries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElPrestoJobElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl>>,
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPrestoJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_on_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl>>,
    dynamic: DataprocWorkflowTemplateJobsElPrestoJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElPrestoJobEl {
    #[doc = "Set the field `client_tags`.\nOptional. Presto client tags to attach to this query"]
    pub fn set_client_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.client_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `continue_on_failure`.\nOptional. Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries."]
    pub fn set_continue_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continue_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_format`.\nOptional. The format in which query output will be displayed. See the Presto documentation for supported output formats"]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values. Used to set Presto [session properties](https://prestodb.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Presto CLI"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_list`.\n"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPrestoJobElQueryListEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPrestoJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPrestoJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPrestoJobEl {}

impl BuildDataprocWorkflowTemplateJobsElPrestoJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPrestoJobEl {
        DataprocWorkflowTemplateJobsElPrestoJobEl {
            client_tags: core::default::Default::default(),
            continue_on_failure: core::default::Default::default(),
            output_format: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            query_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElPrestoJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPrestoJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElPrestoJobElRef {
        DataprocWorkflowTemplateJobsElPrestoJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPrestoJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_tags` after provisioning.\nOptional. Presto client tags to attach to this query"]
    pub fn client_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `continue_on_failure` after provisioning.\nOptional. Whether to continue executing queries if a query fails. The default value is `false`. Setting to `true` can be useful when executing independent parallel queries."]
    pub fn continue_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continue_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_format` after provisioning.\nOptional. The format in which query output will be displayed. See the Presto documentation for supported output formats"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values. Used to set Presto [session properties](https://prestodb.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Presto CLI"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPrestoJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPrestoJobElQueryListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElPysparkJobElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElPysparkJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    main_python_file_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl>>,
    dynamic: DataprocWorkflowTemplateJobsElPysparkJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElPysparkJobEl {
    #[doc =
        "Set the field `archive_uris`.\nOptional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nOptional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_file_uris`.\nOptional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub fn set_python_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.python_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElPysparkJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPysparkJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElPysparkJobEl {
    #[doc = "Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub main_python_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplateJobsElPysparkJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElPysparkJobEl {
        DataprocWorkflowTemplateJobsElPysparkJobEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: self.main_python_file_uri,
            properties: core::default::Default::default(),
            python_file_uris: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElPysparkJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElPysparkJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElPysparkJobElRef {
        DataprocWorkflowTemplateJobsElPysparkJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElPysparkJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nOptional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nOptional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_python_file_uri` after provisioning.\nRequired. The HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub fn main_python_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_python_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_file_uris` after provisioning.\nOptional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub fn python_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_file_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPysparkJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSchedulingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_failures_per_hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_failures_total: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplateJobsElSchedulingEl {
    #[doc =
        "Set the field `max_failures_per_hour`.\nOptional. Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. A job may be reported as thrashing if driver exits with non-zero code 4 times within 10 minute window. Maximum value is 10."]
    pub fn set_max_failures_per_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_failures_per_hour = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_failures_total`.\nOptional. Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. Maximum value is 240."]
    pub fn set_max_failures_total(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_failures_total = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSchedulingEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSchedulingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSchedulingEl {}

impl BuildDataprocWorkflowTemplateJobsElSchedulingEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSchedulingEl {
        DataprocWorkflowTemplateJobsElSchedulingEl {
            max_failures_per_hour: core::default::Default::default(),
            max_failures_total: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSchedulingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSchedulingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElSchedulingElRef {
        DataprocWorkflowTemplateJobsElSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSchedulingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_failures_per_hour` after provisioning.\nOptional. Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. A job may be reported as thrashing if driver exits with non-zero code 4 times within 10 minute window. Maximum value is 10."]
    pub fn max_failures_per_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_failures_per_hour", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_failures_total` after provisioning.\nOptional. Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. Maximum value is 240."]
    pub fn max_failures_total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_failures_total", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElSparkJobElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_jar_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl>>,
    dynamic: DataprocWorkflowTemplateJobsElSparkJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElSparkJobEl {
    #[doc =
        "Set the field `archive_uris`.\nOptional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nOptional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in `jar_file_uris`."]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `main_jar_file_uri`.\nThe HCFS URI of the jar file that contains the main class."]
    pub fn set_main_jar_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_jar_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkJobEl {}

impl BuildDataprocWorkflowTemplateJobsElSparkJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkJobEl {
        DataprocWorkflowTemplateJobsElSparkJobEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElSparkJobElRef {
        DataprocWorkflowTemplateJobsElSparkJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nOptional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nOptional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in `jar_file_uris`."]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of the jar file that contains the main class."]
    pub fn main_jar_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_jar_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElSparkRJobElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkRJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    main_r_file_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl>>,
    dynamic: DataprocWorkflowTemplateJobsElSparkRJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElSparkRJobEl {
    #[doc =
        "Set the field `archive_uris`.\nOptional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nOptional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure SparkR. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkRJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkRJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkRJobEl {
    #[doc = "Required. The HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub main_r_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplateJobsElSparkRJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkRJobEl {
        DataprocWorkflowTemplateJobsElSparkRJobEl {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_r_file_uri: self.main_r_file_uri,
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkRJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkRJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElSparkRJobElRef {
        DataprocWorkflowTemplateJobsElSparkRJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkRJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nOptional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional. The arguments to pass to the driver. Do not include arguments, such as `--conf`, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nOptional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_r_file_uri` after provisioning.\nRequired. The HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub fn main_r_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_r_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure SparkR. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkRJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {
    #[doc =
        "Set the field `driver_log_levels`.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn set_driver_log_levels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_log_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {}

impl BuildDataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {
        DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl {
            driver_log_levels: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigElRef {
        DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nThe per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl {
    queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl { }

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl {
    #[doc =
        "Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl {
        DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl { queries: self.queries }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListElRef {
        DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `queries` after provisioning.\nRequired. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub fn queries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElSparkSqlJobElDynamic {
    logging_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl>>,
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsElSparkSqlJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl>>,
    dynamic: DataprocWorkflowTemplateJobsElSparkSqlJobElDynamic,
}

impl DataprocWorkflowTemplateJobsElSparkSqlJobEl {
    #[doc = "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Dataproc API may be overwritten."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nOptional. Mapping of query variable names to values (equivalent to the Spark SQL command: SET `name=\"value\";`)."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_list`.\n"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsElSparkSqlJobEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkSqlJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsElSparkSqlJobEl {}

impl BuildDataprocWorkflowTemplateJobsElSparkSqlJobEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsElSparkSqlJobEl {
        DataprocWorkflowTemplateJobsElSparkSqlJobEl {
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            query_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElSparkSqlJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElSparkSqlJobElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateJobsElSparkSqlJobElRef {
        DataprocWorkflowTemplateJobsElSparkSqlJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElSparkSqlJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Dataproc API may be overwritten."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nOptional. Mapping of query variable names to values (equivalent to the Spark SQL command: SET `name=\"value\";`)."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkSqlJobElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkSqlJobElQueryListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateJobsElDynamic {
    hadoop_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElHadoopJobEl>>,
    hive_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElHiveJobEl>>,
    pig_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPigJobEl>>,
    presto_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPrestoJobEl>>,
    pyspark_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElPysparkJobEl>>,
    scheduling: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSchedulingEl>>,
    spark_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkJobEl>>,
    spark_r_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkRJobEl>>,
    spark_sql_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsElSparkSqlJobEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateJobsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prerequisite_step_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    step_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hadoop_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElHadoopJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElHiveJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pig_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPigJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    presto_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPrestoJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pyspark_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElPysparkJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSchedulingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_r_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkRJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_sql_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateJobsElSparkSqlJobEl>>,
    dynamic: DataprocWorkflowTemplateJobsElDynamic,
}

impl DataprocWorkflowTemplateJobsEl {
    #[doc =
        "Set the field `labels`.\nOptional. The labels to associate with this job. Label keys must be between 1 and 63 characters long, and must conform to the following regular expression: p{Ll}p{Lo}{0,62} Label values must be between 1 and 63 characters long, and must conform to the following regular expression: [p{Ll}p{Lo}p{N}_-]{0,63} No more than 32 labels can be associated with a given job."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prerequisite_step_ids`.\nOptional. The optional list of prerequisite job step_ids. If not specified, the job will start at the beginning of workflow."]
    pub fn set_prerequisite_step_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prerequisite_step_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hadoop_job`.\n"]
    pub fn set_hadoop_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHadoopJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hadoop_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hadoop_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hive_job`.\n"]
    pub fn set_hive_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElHiveJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hive_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hive_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pig_job`.\n"]
    pub fn set_pig_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPigJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pig_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pig_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `presto_job`.\n"]
    pub fn set_presto_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPrestoJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.presto_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.presto_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pyspark_job`.\n"]
    pub fn set_pyspark_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElPysparkJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pyspark_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pyspark_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSchedulingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_job`.\n"]
    pub fn set_spark_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_r_job`.\n"]
    pub fn set_spark_r_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkRJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_r_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_r_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_sql_job`.\n"]
    pub fn set_spark_sql_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateJobsElSparkSqlJobEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_sql_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_sql_job = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateJobsEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateJobsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateJobsEl {
    #[doc =
        "Required. The step id. The id must be unique among all jobs within the template. The step id is used as prefix for job id, as job `goog-dataproc-workflow-step-id` label, and in prerequisiteStepIds field from other steps. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters."]
    pub step_id: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplateJobsEl {
    pub fn build(self) -> DataprocWorkflowTemplateJobsEl {
        DataprocWorkflowTemplateJobsEl {
            labels: core::default::Default::default(),
            prerequisite_step_ids: core::default::Default::default(),
            step_id: self.step_id,
            hadoop_job: core::default::Default::default(),
            hive_job: core::default::Default::default(),
            pig_job: core::default::Default::default(),
            presto_job: core::default::Default::default(),
            pyspark_job: core::default::Default::default(),
            scheduling: core::default::Default::default(),
            spark_job: core::default::Default::default(),
            spark_r_job: core::default::Default::default(),
            spark_sql_job: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateJobsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateJobsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateJobsElRef {
        DataprocWorkflowTemplateJobsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateJobsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this job. Label keys must be between 1 and 63 characters long, and must conform to the following regular expression: p{Ll}p{Lo}{0,62} Label values must be between 1 and 63 characters long, and must conform to the following regular expression: [p{Ll}p{Lo}p{N}_-]{0,63} No more than 32 labels can be associated with a given job."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prerequisite_step_ids` after provisioning.\nOptional. The optional list of prerequisite job step_ids. If not specified, the job will start at the beginning of workflow."]
    pub fn prerequisite_step_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prerequisite_step_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `step_id` after provisioning.\nRequired. The step id. The id must be unique among all jobs within the template. The step id is used as prefix for job id, as job `goog-dataproc-workflow-step-id` label, and in prerequisiteStepIds field from other steps. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters."]
    pub fn step_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_id", self.base))
    }

    #[doc = "Get a reference to the value of field `hadoop_job` after provisioning.\n"]
    pub fn hadoop_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElHadoopJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_job", self.base))
    }

    #[doc = "Get a reference to the value of field `hive_job` after provisioning.\n"]
    pub fn hive_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElHiveJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_job", self.base))
    }

    #[doc = "Get a reference to the value of field `pig_job` after provisioning.\n"]
    pub fn pig_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPigJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pig_job", self.base))
    }

    #[doc = "Get a reference to the value of field `presto_job` after provisioning.\n"]
    pub fn presto_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPrestoJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto_job", self.base))
    }

    #[doc = "Get a reference to the value of field `pyspark_job` after provisioning.\n"]
    pub fn pyspark_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElPysparkJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_job", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_job` after provisioning.\n"]
    pub fn spark_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_job", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_r_job` after provisioning.\n"]
    pub fn spark_r_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkRJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_r_job", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_sql_job` after provisioning.\n"]
    pub fn spark_sql_job(&self) -> terrars::ListRef<DataprocWorkflowTemplateJobsElSparkSqlJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_job", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateParametersElValidationElRegexEl {
    regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateParametersElValidationElRegexEl { }

impl terrars::ToListMappable for DataprocWorkflowTemplateParametersElValidationElRegexEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateParametersElValidationElRegexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateParametersElValidationElRegexEl {
    #[doc =
        "Required. RE2 regular expressions used to validate the parameter's value. The value must match the regex in its entirety (substring matches are not sufficient)."]
    pub regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplateParametersElValidationElRegexEl {
    pub fn build(self) -> DataprocWorkflowTemplateParametersElValidationElRegexEl {
        DataprocWorkflowTemplateParametersElValidationElRegexEl { regexes: self.regexes }
    }
}

pub struct DataprocWorkflowTemplateParametersElValidationElRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateParametersElValidationElRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateParametersElValidationElRegexElRef {
        DataprocWorkflowTemplateParametersElValidationElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateParametersElValidationElRegexElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `regexes` after provisioning.\nRequired. RE2 regular expressions used to validate the parameter's value. The value must match the regex in its entirety (substring matches are not sufficient)."]
    pub fn regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateParametersElValidationElValuesEl {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateParametersElValidationElValuesEl { }

impl terrars::ToListMappable for DataprocWorkflowTemplateParametersElValidationElValuesEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateParametersElValidationElValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateParametersElValidationElValuesEl {
    #[doc = "Required. List of allowed values for the parameter."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplateParametersElValidationElValuesEl {
    pub fn build(self) -> DataprocWorkflowTemplateParametersElValidationElValuesEl {
        DataprocWorkflowTemplateParametersElValidationElValuesEl { values: self.values }
    }
}

pub struct DataprocWorkflowTemplateParametersElValidationElValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateParametersElValidationElValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateParametersElValidationElValuesElRef {
        DataprocWorkflowTemplateParametersElValidationElValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateParametersElValidationElValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nRequired. List of allowed values for the parameter."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateParametersElValidationElDynamic {
    regex: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateParametersElValidationElRegexEl>>,
    values: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateParametersElValidationElValuesEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateParametersElValidationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateParametersElValidationElRegexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateParametersElValidationElValuesEl>>,
    dynamic: DataprocWorkflowTemplateParametersElValidationElDynamic,
}

impl DataprocWorkflowTemplateParametersElValidationEl {
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateParametersElValidationElRegexEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplateParametersElValidationElValuesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateParametersElValidationEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateParametersElValidationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateParametersElValidationEl {}

impl BuildDataprocWorkflowTemplateParametersElValidationEl {
    pub fn build(self) -> DataprocWorkflowTemplateParametersElValidationEl {
        DataprocWorkflowTemplateParametersElValidationEl {
            regex: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateParametersElValidationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateParametersElValidationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplateParametersElValidationElRef {
        DataprocWorkflowTemplateParametersElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateParametersElValidationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataprocWorkflowTemplateParametersElValidationElRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<DataprocWorkflowTemplateParametersElValidationElValuesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateParametersElDynamic {
    validation: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateParametersElValidationEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<DataprocWorkflowTemplateParametersElValidationEl>>,
    dynamic: DataprocWorkflowTemplateParametersElDynamic,
}

impl DataprocWorkflowTemplateParametersEl {
    #[doc =
        "Set the field `description`.\nOptional. Brief description of the parameter. Must not exceed 1024 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplateParametersElValidationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplateParametersEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateParametersEl {
    #[doc =
        "Required. Paths to all fields that the parameter replaces. A field is allowed to appear in at most one parameter's list of field paths. A field path is similar in syntax to a google.protobuf.FieldMask. For example, a field path that references the zone field of a workflow template's cluster selector would be specified as `placement.clusterSelector.zone`. Also, field paths can reference fields using the following syntax: * Values in maps can be referenced by key: * labels['key'] * placement.clusterSelector.clusterLabels['key'] * placement.managedCluster.labels['key'] * placement.clusterSelector.clusterLabels['key'] * jobs['step-id'].labels['key'] * Jobs in the jobs list can be referenced by step-id: * jobs['step-id'].hadoopJob.mainJarFileUri * jobs['step-id'].hiveJob.queryFileUri * jobs['step-id'].pySparkJob.mainPythonFileUri * jobs['step-id'].hadoopJob.jarFileUris[0] * jobs['step-id'].hadoopJob.archiveUris[0] * jobs['step-id'].hadoopJob.fileUris[0] * jobs['step-id'].pySparkJob.pythonFileUris[0] * Items in repeated fields can be referenced by a zero-based index: * jobs['step-id'].sparkJob.args[0] * Other examples: * jobs['step-id'].hadoopJob.properties['key'] * jobs['step-id'].hadoopJob.args[0] * jobs['step-id'].hiveJob.scriptVariables['key'] * jobs['step-id'].hadoopJob.mainJarFileUri * placement.clusterSelector.zone It may not be possible to parameterize maps and repeated fields in their entirety since only individual map values and individual items in repeated fields can be referenced. For example, the following field paths are invalid: - placement.clusterSelector.clusterLabels - jobs['step-id'].sparkJob.args"]
    pub fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Required. Parameter name. The parameter name is used as the key, and paired with the parameter value, which are passed to the template when the template is instantiated. The name must contain only capital letters (A-Z), numbers (0-9), and underscores (_), and must not start with a number. The maximum length is 40 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplateParametersEl {
    pub fn build(self) -> DataprocWorkflowTemplateParametersEl {
        DataprocWorkflowTemplateParametersEl {
            description: core::default::Default::default(),
            fields: self.fields,
            name: self.name,
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateParametersElRef {
        DataprocWorkflowTemplateParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Brief description of the parameter. Must not exceed 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fields` after provisioning.\nRequired. Paths to all fields that the parameter replaces. A field is allowed to appear in at most one parameter's list of field paths. A field path is similar in syntax to a google.protobuf.FieldMask. For example, a field path that references the zone field of a workflow template's cluster selector would be specified as `placement.clusterSelector.zone`. Also, field paths can reference fields using the following syntax: * Values in maps can be referenced by key: * labels['key'] * placement.clusterSelector.clusterLabels['key'] * placement.managedCluster.labels['key'] * placement.clusterSelector.clusterLabels['key'] * jobs['step-id'].labels['key'] * Jobs in the jobs list can be referenced by step-id: * jobs['step-id'].hadoopJob.mainJarFileUri * jobs['step-id'].hiveJob.queryFileUri * jobs['step-id'].pySparkJob.mainPythonFileUri * jobs['step-id'].hadoopJob.jarFileUris[0] * jobs['step-id'].hadoopJob.archiveUris[0] * jobs['step-id'].hadoopJob.fileUris[0] * jobs['step-id'].pySparkJob.pythonFileUris[0] * Items in repeated fields can be referenced by a zero-based index: * jobs['step-id'].sparkJob.args[0] * Other examples: * jobs['step-id'].hadoopJob.properties['key'] * jobs['step-id'].hadoopJob.args[0] * jobs['step-id'].hiveJob.scriptVariables['key'] * jobs['step-id'].hadoopJob.mainJarFileUri * placement.clusterSelector.zone It may not be possible to parameterize maps and repeated fields in their entirety since only individual map values and individual items in repeated fields can be referenced. For example, the following field paths are invalid: - placement.clusterSelector.clusterLabels - jobs['step-id'].sparkJob.args"]
    pub fn fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nRequired. Parameter name. The parameter name is used as the key, and paired with the parameter value, which are passed to the template when the template is instantiated. The name must contain only capital letters (A-Z), numbers (0-9), and underscores (_), and must not start with a number. The maximum length is 40 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<DataprocWorkflowTemplateParametersElValidationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElClusterSelectorEl {
    cluster_labels: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElClusterSelectorEl {
    #[doc =
        "Set the field `zone`.\nOptional. The zone where workflow process executes. This parameter does not affect the selection of the cluster. If unspecified, the zone of the first cluster matching the selector is used."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElClusterSelectorEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElClusterSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElClusterSelectorEl {
    #[doc = "Required. The cluster labels. Cluster must have all labels to match."]
    pub cluster_labels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplatePlacementElClusterSelectorEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElClusterSelectorEl {
        DataprocWorkflowTemplatePlacementElClusterSelectorEl {
            cluster_labels: self.cluster_labels,
            zone: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElClusterSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElClusterSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElClusterSelectorElRef {
        DataprocWorkflowTemplatePlacementElClusterSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElClusterSelectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_labels` after provisioning.\nRequired. The cluster labels. Cluster must have all labels to match."]
    pub fn cluster_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.cluster_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nOptional. The zone where workflow process executes. This parameter does not affect the selection of the cluster. If unspecified, the zone of the first cluster matching the selector is used."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {
    #[doc =
        "Set the field `policy`.\nOptional. The autoscaling policy used by the cluster. Only resource names including projectid and location (region) are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]` * `projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]` Note that the policy must be in the same project and Dataproc region."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl {
            policy: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `policy` after provisioning.\nOptional. The autoscaling policy used by the cluster. Only resource names including projectid and location (region) are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]` * `projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]` Note that the policy must be in the same project and Dataproc region."]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_pd_kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {
    #[doc =
        "Set the field `gce_pd_kms_key_name`.\nOptional. The Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub fn set_gce_pd_kms_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gce_pd_kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl {
            gce_pd_kms_key_name: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gce_pd_kms_key_name` after provisioning.\nOptional. The Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub fn gce_pd_kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_pd_kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_http_port_access: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {
    #[doc =
        "Set the field `enable_http_port_access`.\nOptional. If true, enable http access to specific ports on the cluster from external sources. Defaults to false."]
    pub fn set_enable_http_port_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_http_port_access = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl {
            enable_http_port_access: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_http_port_access` after provisioning.\nOptional. If true, enable http access to specific ports on the cluster from external sources. Defaults to false."]
    pub fn enable_http_port_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_http_port_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_ports` after provisioning.\nOutput only. The map of port descriptions to URLs. Will only be populated if enable_http_port_access is true."]
    pub fn http_ports(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.http_ports", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl {
    node_group: terrars::PrimField<std::string::String>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl { }

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl {
    #[doc =
        "Required. The URI of a sole-tenant [node group resource](https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups) that the cluster will be created on. A full URL, partial URI, or node group name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1` * `projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1` * `node-group-1`"]
    pub node_group: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl {
            node_group: self.node_group,
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_group` after provisioning.\nRequired. The URI of a sole-tenant [node group resource](https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups) that the cluster will be created on. A full URL, partial URI, or node group name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1` * `projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1` * `node-group-1`"]
    pub fn node_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consume_reservation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {
    #[doc =
        "Set the field `consume_reservation_type`.\nOptional. Type of reservation to consume Possible values: TYPE_UNSPECIFIED, NO_RESERVATION, ANY_RESERVATION, SPECIFIC_RESERVATION"]
    pub fn set_consume_reservation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consume_reservation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nOptional. Corresponds to the label key of reservation resource."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\nOptional. Corresponds to the label values of reservation resource."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consume_reservation_type` after provisioning.\nOptional. Type of reservation to consume Possible values: TYPE_UNSPECIFIED, NO_RESERVATION, ANY_RESERVATION, SPECIFIC_RESERVATION"]
    pub fn consume_reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nOptional. Corresponds to the label key of reservation resource."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nOptional. Corresponds to the label values of reservation resource."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {
    #[doc =
        "Set the field `enable_integrity_monitoring`.\nOptional. Defines whether instances have integrity monitoring enabled. Integrity monitoring compares the most recent boot measurements to the integrity policy baseline and returns a pair of pass/fail results depending on whether they match or not."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_secure_boot`.\nOptional. Defines whether the instances have Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_vtpm`.\nOptional. Defines whether the instance have the vTPM enabled. Virtual Trusted Platform Module protects objects like keys, certificates and enables Measured Boot by performing the measurements needed to create a known good boot baseline, called the integrity policy baseline."]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nOptional. Defines whether instances have integrity monitoring enabled. Integrity monitoring compares the most recent boot measurements to the integrity policy baseline and returns a pair of pass/fail results depending on whether they match or not."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nOptional. Defines whether the instances have Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_vtpm` after provisioning.\nOptional. Defines whether the instance have the vTPM enabled. Virtual Trusted Platform Module protects objects like keys, certificates and enables Measured Boot by performing the measurements needed to create a known good boot baseline, called the integrity policy baseline."]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElDynamic {
    node_group_affinity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl,
        >,
    >,
    reservation_affinity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl,
        >,
    >,
    shielded_instance_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ipv6_google_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_group_affinity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl>,
    >,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {
    #[doc =
        "Set the field `internal_ip_only`.\nOptional. If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This `internal_ip_only` restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses."]
    pub fn set_internal_ip_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_ip_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata`.\nThe Compute Engine metadata entries to add to all instances (see [Project and instance metadata](https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata))."]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nOptional. The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither `network_uri` nor `subnetwork_uri` is specified, the \"default\" network of the project is used, if it exists. Cannot be a \"Custom Subnet Network\" (see [Using Subnetworks](https://cloud.google.com/compute/docs/subnetworks) for more information). A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/regions/global/default` * `projects/[project_id]/regions/global/default` * `default`"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_ipv6_google_access`.\nOptional. The type of IPv6 access for a cluster. Possible values: PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED, INHERIT_FROM_SUBNETWORK, OUTBOUND, BIDIRECTIONAL"]
    pub fn set_private_ipv6_google_access(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ipv6_google_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nOptional. The [Dataproc service account](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/service-accounts#service_accounts_in_dataproc) (also see [VM Data Plane identity](https://cloud.google.com/dataproc/docs/concepts/iam/dataproc-principals#vm_service_account_data_plane_identity)) used by Dataproc cluster VM instances to access Google Cloud Platform services. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_scopes`.\nOptional. The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included: * https://www.googleapis.com/auth/cloud.useraccounts.readonly * https://www.googleapis.com/auth/devstorage.read_write * https://www.googleapis.com/auth/logging.write If no scopes are specified, the following defaults are also provided: * https://www.googleapis.com/auth/bigquery * https://www.googleapis.com/auth/bigtable.admin.table * https://www.googleapis.com/auth/bigtable.data * https://www.googleapis.com/auth/devstorage.full_control"]
    pub fn set_service_account_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nOptional. The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/regions/us-east1/subnetworks/sub0` * `projects/[project_id]/regions/us-east1/subnetworks/sub0` * `sub0`"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nThe Compute Engine tags to add to all instances (see [Tagging instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nOptional. The zone where the Compute Engine cluster will be located. On a create request, it is required in the \"global\" region. If omitted in a non-global Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/[zone]` * `projects/[project_id]/zones/[zone]` * `us-central1-f`"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_group_affinity`.\n"]
    pub fn set_node_group_affinity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_group_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_group_affinity = std::option::Option::Some(d);
            },
        }
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
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityEl,
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

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl {
            internal_ip_only: core::default::Default::default(),
            metadata: core::default::Default::default(),
            network: core::default::Default::default(),
            private_ipv6_google_access: core::default::Default::default(),
            service_account: core::default::Default::default(),
            service_account_scopes: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
            zone: core::default::Default::default(),
            node_group_affinity: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `internal_ip_only` after provisioning.\nOptional. If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This `internal_ip_only` restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses."]
    pub fn internal_ip_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nThe Compute Engine metadata entries to add to all instances (see [Project and instance metadata](https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata))."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nOptional. The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither `network_uri` nor `subnetwork_uri` is specified, the \"default\" network of the project is used, if it exists. Cannot be a \"Custom Subnet Network\" (see [Using Subnetworks](https://cloud.google.com/compute/docs/subnetworks) for more information). A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/regions/global/default` * `projects/[project_id]/regions/global/default` * `default`"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_ipv6_google_access` after provisioning.\nOptional. The type of IPv6 access for a cluster. Possible values: PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED, INHERIT_FROM_SUBNETWORK, OUTBOUND, BIDIRECTIONAL"]
    pub fn private_ipv6_google_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ipv6_google_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. The [Dataproc service account](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/service-accounts#service_accounts_in_dataproc) (also see [VM Data Plane identity](https://cloud.google.com/dataproc/docs/concepts/iam/dataproc-principals#vm_service_account_data_plane_identity)) used by Dataproc cluster VM instances to access Google Cloud Platform services. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_scopes` after provisioning.\nOptional. The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included: * https://www.googleapis.com/auth/cloud.useraccounts.readonly * https://www.googleapis.com/auth/devstorage.read_write * https://www.googleapis.com/auth/logging.write If no scopes are specified, the following defaults are also provided: * https://www.googleapis.com/auth/bigquery * https://www.googleapis.com/auth/bigtable.admin.table * https://www.googleapis.com/auth/bigtable.data * https://www.googleapis.com/auth/devstorage.full_control"]
    pub fn service_account_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nOptional. The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/regions/us-east1/subnetworks/sub0` * `projects/[project_id]/regions/us-east1/subnetworks/sub0` * `sub0`"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe Compute Engine tags to add to all instances (see [Tagging instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nOptional. The zone where the Compute Engine cluster will be located. On a create request, it is required in the \"global\" region. If omitted in a non-global Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/[zone]` * `projects/[project_id]/zones/[zone]` * `us-central1-f`"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc = "Get a reference to the value of field `node_group_affinity` after provisioning.\n"]
    pub fn node_group_affinity(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElNodeGroupAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_group_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElShieldedInstanceConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executable_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {
    #[doc = "Set the field `executable_file`.\nRequired. Cloud Storage URI of executable file."]
    pub fn set_executable_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.executable_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `execution_timeout`.\nOptional. Amount of time executable has to complete. Default is 10 minutes (see JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). Cluster creation fails with an explanatory error message (the name of the executable that caused the error and the exceeded timeout period) if the executable is not completed at end of the timeout period."]
    pub fn set_execution_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl {
            executable_file: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `executable_file` after provisioning.\nRequired. Cloud Storage URI of executable file."]
    pub fn executable_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.executable_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_timeout` after provisioning.\nOptional. Amount of time executable has to complete. Default is 10 minutes (see JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). Cluster creation fails with an explanatory error message (the name of the executable that caused the error and the exceeded timeout period) if the executable is not completed at end of the timeout period."]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_delete_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {
    #[doc =
        "Set the field `auto_delete_time`.\nOptional. The time when cluster will be auto-deleted (see JSON representation of [Timestamp](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn set_auto_delete_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_delete_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_delete_ttl`.\nOptional. The lifetime duration of cluster. The cluster will be auto-deleted at the end of this period. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn set_auto_delete_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_delete_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `idle_delete_ttl`.\nOptional. The duration to keep the cluster alive while idling (when no jobs are running). Passing this threshold will cause the cluster to be deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn set_idle_delete_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_delete_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl {
            auto_delete_time: core::default::Default::default(),
            auto_delete_ttl: core::default::Default::default(),
            idle_delete_ttl: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_delete_time` after provisioning.\nOptional. The time when cluster will be auto-deleted (see JSON representation of [Timestamp](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn auto_delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_delete_ttl` after provisioning.\nOptional. The lifetime duration of cluster. The cluster will be auto-deleted at the end of this period. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn auto_delete_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_delete_ttl` after provisioning.\nOptional. The duration to keep the cluster alive while idling (when no jobs are running). Passing this threshold will cause the cluster to be deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn idle_delete_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_delete_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_start_time` after provisioning.\nOutput only. The time when cluster became idle (most recent job finished) and became eligible for deletion due to idleness (see JSON representation of [Timestamp](https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub fn idle_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_manager_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {
    #[doc = "Set the field `instance_group_manager_name`.\n"]
    pub fn set_instance_group_manager_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_group_manager_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_template_name`.\n"]
    pub fn set_instance_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_template_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigEl {
            instance_group_manager_name: core::default::Default::default(),
            instance_template_name: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_group_manager_name` after provisioning.\n"]
    pub fn instance_group_manager_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_template_name` after provisioning.\n"]
    pub fn instance_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_template_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {
    #[doc =
        "Set the field `accelerator_count`.\nThe number of the accelerator cards of this type exposed to this instance."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nFull URL, partial URI, or short name of the accelerator type resource to expose to this instance. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `nvidia-tesla-k80` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {
    type O =
        terrars
        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nFull URL, partial URI, or short name of the accelerator type resource to expose to this instance. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `nvidia-tesla-k80` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nOptional. Size in GB of the boot disk (default is 500GB)."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nOptional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See [Disk types](https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nOptional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {
    type O =
        terrars
        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl {
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nOptional. Size in GB of the boot disk (default is 500GB)."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nOptional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See [Disk types](https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nOptional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDynamic {
    accelerators: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl>,
    >,
    disk_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl>>,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {
    #[doc =
        "Set the field `image`.\nOptional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nOptional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nOptional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc -> Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_instances`.\nOptional. The number of VM instances in the instance group. For [HA cluster](/dataproc/docs/concepts/configuring-clusters/high-availability) [master_config](#FIELDS.master_config) groups, **must be set to 3**. For standard cluster [master_config](#FIELDS.master_config) groups, **must be set to 1**."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preemptibility`.\nOptional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE"]
    pub fn set_preemptibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preemptibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl {
            image: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nOptional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nOutput only. The list of instance names. Dataproc derives the names from `cluster_name`, `num_instances`, and the instance group."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_preemptible` after provisioning.\nOutput only. Specifies that this instance group contains preemptible instances."]
    pub fn is_preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_group_config` after provisioning.\nOutput only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups."]
    pub fn managed_group_config(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElManagedGroupConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_group_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nOptional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc -> Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nOptional. The number of VM instances in the instance group. For [HA cluster](/dataproc/docs/concepts/configuring-clusters/high-availability) [master_config](#FIELDS.master_config) groups, **must be set to 3**. For standard cluster [master_config](#FIELDS.master_config) groups, **must be set to 1**."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptibility` after provisioning.\nOptional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE"]
    pub fn preemptibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptibility", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElAcceleratorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElDiskConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_manager_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {
    #[doc = "Set the field `instance_group_manager_name`.\n"]
    pub fn set_instance_group_manager_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_group_manager_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_template_name`.\n"]
    pub fn set_instance_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_template_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigEl {
            instance_group_manager_name: core::default::Default::default(),
            instance_template_name: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_group_manager_name` after provisioning.\n"]
    pub fn instance_group_manager_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_template_name` after provisioning.\n"]
    pub fn instance_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_template_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {
    #[doc =
        "Set the field `accelerator_count`.\nThe number of the accelerator cards of this type exposed to this instance."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nFull URL, partial URI, or short name of the accelerator type resource to expose to this instance. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `nvidia-tesla-k80` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nFull URL, partial URI, or short name of the accelerator type resource to expose to this instance. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `nvidia-tesla-k80` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nOptional. Size in GB of the boot disk (default is 500GB)."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nOptional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See [Disk types](https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nOptional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl {
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nOptional. Size in GB of the boot disk (default is 500GB)."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nOptional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See [Disk types](https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nOptional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDynamic {
    accelerators: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl,
        >,
    >,
    disk_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<
        std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl>,
    >,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {
    #[doc =
        "Set the field `image`.\nOptional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nOptional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nOptional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc -> Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_instances`.\nOptional. The number of VM instances in the instance group. For [HA cluster](/dataproc/docs/concepts/configuring-clusters/high-availability) [master_config](#FIELDS.master_config) groups, **must be set to 3**. For standard cluster [master_config](#FIELDS.master_config) groups, **must be set to 1**."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preemptibility`.\nOptional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE"]
    pub fn set_preemptibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preemptibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl {
            image: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nOptional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nOutput only. The list of instance names. Dataproc derives the names from `cluster_name`, `num_instances`, and the instance group."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_preemptible` after provisioning.\nOutput only. Specifies that this instance group contains preemptible instances."]
    pub fn is_preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_group_config` after provisioning.\nOutput only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups."]
    pub fn managed_group_config(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElManagedGroupConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_group_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nOptional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc -> Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nOptional. The number of VM instances in the instance group. For [HA cluster](/dataproc/docs/concepts/configuring-clusters/high-availability) [master_config](#FIELDS.master_config) groups, **must be set to 3**. For standard cluster [master_config](#FIELDS.master_config) groups, **must be set to 1**."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptibility` after provisioning.\nOptional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE"]
    pub fn preemptibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptibility", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElAcceleratorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElDiskConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_admin_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_kdc: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_realm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_shared_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_kerberos: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kdc_db_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keystore: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keystore_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_principal_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tgt_lifetime_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    truststore: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    truststore_password: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {
    #[doc =
        "Set the field `cross_realm_trust_admin_server`.\nOptional. The admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_admin_server(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_admin_server = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_realm_trust_kdc`.\nOptional. The KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_kdc(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_kdc = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_realm_trust_realm`.\nOptional. The remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub fn set_cross_realm_trust_realm(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_realm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cross_realm_trust_shared_password`.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster Kerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub fn set_cross_realm_trust_shared_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_shared_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_kerberos`.\nOptional. Flag to indicate whether to Kerberize the cluster (default: false). Set this field to true to enable Kerberos on a cluster."]
    pub fn set_enable_kerberos(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_kerberos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kdc_db_key`.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub fn set_kdc_db_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kdc_db_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_password`.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_key_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keystore`.\nOptional. The Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn set_keystore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keystore = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `keystore_password`.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided keystore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_keystore_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keystore_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\nOptional. The uri of the KMS key used to encrypt various sensitive files."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `realm`.\nOptional. The name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub fn set_realm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.realm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `root_principal_password`.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub fn set_root_principal_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.root_principal_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tgt_lifetime_hours`.\nOptional. The lifetime of the ticket granting ticket, in hours. If not specified, or user specifies 0, then default value 10 will be used."]
    pub fn set_tgt_lifetime_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tgt_lifetime_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `truststore`.\nOptional. The Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn set_truststore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.truststore = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `truststore_password`.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn set_truststore_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.truststore_password = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl {
            cross_realm_trust_admin_server: core::default::Default::default(),
            cross_realm_trust_kdc: core::default::Default::default(),
            cross_realm_trust_realm: core::default::Default::default(),
            cross_realm_trust_shared_password: core::default::Default::default(),
            enable_kerberos: core::default::Default::default(),
            kdc_db_key: core::default::Default::default(),
            key_password: core::default::Default::default(),
            keystore: core::default::Default::default(),
            keystore_password: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            realm: core::default::Default::default(),
            root_principal_password: core::default::Default::default(),
            tgt_lifetime_hours: core::default::Default::default(),
            truststore: core::default::Default::default(),
            truststore_password: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_admin_server` after provisioning.\nOptional. The admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn cross_realm_trust_admin_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_admin_server", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_kdc` after provisioning.\nOptional. The KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub fn cross_realm_trust_kdc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_kdc", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_realm` after provisioning.\nOptional. The remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub fn cross_realm_trust_realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_realm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cross_realm_trust_shared_password` after provisioning.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster Kerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub fn cross_realm_trust_shared_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_shared_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_kerberos` after provisioning.\nOptional. Flag to indicate whether to Kerberize the cluster (default: false). Set this field to true to enable Kerberos on a cluster."]
    pub fn enable_kerberos(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_kerberos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kdc_db_key` after provisioning.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub fn kdc_db_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kdc_db_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_password` after provisioning.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn key_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keystore` after provisioning.\nOptional. The Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn keystore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.base))
    }

    #[doc =
        "Get a reference to the value of field `keystore_password` after provisioning.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided keystore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn keystore_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nOptional. The uri of the KMS key used to encrypt various sensitive files."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `realm` after provisioning.\nOptional. The name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub fn realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `root_principal_password` after provisioning.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub fn root_principal_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_principal_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tgt_lifetime_hours` after provisioning.\nOptional. The lifetime of the ticket granting ticket, in hours. If not specified, or user specifies 0, then default value 10 will be used."]
    pub fn tgt_lifetime_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tgt_lifetime_hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `truststore` after provisioning.\nOptional. The Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub fn truststore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.truststore", self.base))
    }

    #[doc =
        "Get a reference to the value of field `truststore_password` after provisioning.\nOptional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub fn truststore_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.truststore_password", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElDynamic {
    kerberos_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std
    ::option
    ::Option<
        std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl>,
    >,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {
    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kerberos_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kerberos_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl {
            kerberos_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElKerberosConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_components: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {
    #[doc =
        "Set the field `image_version`.\nOptional. The version of software inside the cluster. It must be one of the supported [Dataproc Versions](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions), such as \"1.2\" (including a subminor version, such as \"1.2.29\"), or the [\"preview\" version](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions). If unspecified, it defaults to the latest Debian version."]
    pub fn set_image_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `optional_components`.\nOptional. The set of components to activate on the cluster."]
    pub fn set_optional_components(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.optional_components = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. The properties to set on daemon config files. Property keys are specified in `prefix:property` format, for example `core:hadoop.tmp.dir`. The following are supported prefixes and their mappings: * capacity-scheduler: `capacity-scheduler.xml` * core: `core-site.xml` * distcp: `distcp-default.xml` * hdfs: `hdfs-site.xml` * hive: `hive-site.xml` * mapred: `mapred-site.xml` * pig: `pig.properties` * spark: `spark-defaults.conf` * yarn: `yarn-site.xml` For more information, see [Cluster properties](https://cloud.google.com/dataproc/docs/concepts/cluster-properties)."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl {
            image_version: core::default::Default::default(),
            optional_components: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image_version` after provisioning.\nOptional. The version of software inside the cluster. It must be one of the supported [Dataproc Versions](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions), such as \"1.2\" (including a subminor version, such as \"1.2.29\"), or the [\"preview\" version](https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions). If unspecified, it defaults to the latest Debian version."]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `optional_components` after provisioning.\nOptional. The set of components to activate on the cluster."]
    pub fn optional_components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optional_components", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. The properties to set on daemon config files. Property keys are specified in `prefix:property` format, for example `core:hadoop.tmp.dir`. The following are supported prefixes and their mappings: * capacity-scheduler: `capacity-scheduler.xml` * core: `core-site.xml` * distcp: `distcp-default.xml` * hdfs: `hdfs-site.xml` * hive: `hive-site.xml` * mapred: `mapred-site.xml` * pig: `pig.properties` * spark: `spark-defaults.conf` * yarn: `yarn-site.xml` For more information, see [Cluster properties](https://cloud.google.com/dataproc/docs/concepts/cluster-properties)."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_manager_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {
    #[doc = "Set the field `instance_group_manager_name`.\n"]
    pub fn set_instance_group_manager_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_group_manager_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_template_name`.\n"]
    pub fn set_instance_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_template_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigEl {
            instance_group_manager_name: core::default::Default::default(),
            instance_template_name: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_group_manager_name` after provisioning.\n"]
    pub fn instance_group_manager_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_template_name` after provisioning.\n"]
    pub fn instance_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_template_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {
    #[doc =
        "Set the field `accelerator_count`.\nThe number of the accelerator cards of this type exposed to this instance."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nFull URL, partial URI, or short name of the accelerator type resource to expose to this instance. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `nvidia-tesla-k80` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {
    type O =
        terrars
        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the accelerator cards of this type exposed to this instance."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nFull URL, partial URI, or short name of the accelerator type resource to expose to this instance. See [Compute Engine AcceleratorTypes](https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes). Examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80` * `nvidia-tesla-k80` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {
    #[doc = "Set the field `boot_disk_size_gb`.\nOptional. Size in GB of the boot disk (default is 500GB)."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nOptional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See [Disk types](https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nOptional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {
    type O =
        terrars
        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl {
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nOptional. Size in GB of the boot disk (default is 500GB)."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nOptional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See [Disk types](https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nOptional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDynamic {
    accelerators: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl>,
    >,
    disk_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl>>,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {
    #[doc =
        "Set the field `image`.\nOptional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nOptional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_cpu_platform`.\nOptional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc -> Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_instances`.\nOptional. The number of VM instances in the instance group. For [HA cluster](/dataproc/docs/concepts/configuring-clusters/high-availability) [master_config](#FIELDS.master_config) groups, **must be set to 3**. For standard cluster [master_config](#FIELDS.master_config) groups, **must be set to 1**."]
    pub fn set_num_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preemptibility`.\nOptional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE"]
    pub fn set_preemptibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preemptibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_config`.\n"]
    pub fn set_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl {
            image: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nOptional. The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id]` * `projects/[project_id]/global/images/[image-id]` * `image-id` Image family examples. Dataproc will use the most recent image from the family: * `https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name]` * `projects/[project_id]/global/images/family/[custom-image-family-name]` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_names` after provisioning.\nOutput only. The list of instance names. Dataproc derives the names from `cluster_name`, `num_instances`, and the instance group."]
    pub fn instance_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_preemptible` after provisioning.\nOutput only. Specifies that this instance group contains preemptible instances."]
    pub fn is_preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2` * `n1-standard-2` **Auto Zone Exception**: If you are using the Dataproc [Auto Zone Placement](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `managed_group_config` after provisioning.\nOutput only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups."]
    pub fn managed_group_config(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElManagedGroupConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_group_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nOptional. Specifies the minimum cpu platform for the Instance Group. See [Dataproc -> Minimum CPU Platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_instances` after provisioning.\nOptional. The number of VM instances in the instance group. For [HA cluster](/dataproc/docs/concepts/configuring-clusters/high-availability) [master_config](#FIELDS.master_config) groups, **must be set to 3**. For standard cluster [master_config](#FIELDS.master_config) groups, **must be set to 1**."]
    pub fn num_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptibility` after provisioning.\nOptional. Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE"]
    pub fn preemptibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptibility", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElAcceleratorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElDiskConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElDynamic {
    autoscaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl>>,
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl>>,
    endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl>>,
    gce_cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl>>,
    initialization_actions: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl>,
    >,
    lifecycle_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl>>,
    master_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl>>,
    secondary_worker_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl>,
    >,
    security_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl>>,
    software_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl>>,
    worker_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temp_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_cluster_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialization_actions: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_worker_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl>>,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElConfigElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {
    #[doc =
        "Set the field `staging_bucket`.\nOptional. A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see [Dataproc staging bucket](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)). **This field requires a Cloud Storage bucket name, not a URI to a Cloud Storage bucket.**"]
    pub fn set_staging_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.staging_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temp_bucket`.\nOptional. A Cloud Storage bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. If you do not specify a temp bucket, Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's temp bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket. The default bucket has a TTL of 90 days, but you can use any TTL (or none) if you specify a bucket. **This field requires a Cloud Storage bucket name, not a URI to a Cloud Storage bucket.**"]
    pub fn set_temp_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.temp_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_config`.\n"]
    pub fn set_endpoint_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.endpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gce_cluster_config`.\n"]
    pub fn set_gce_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initialization_actions`.\n"]
    pub fn set_initialization_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initialization_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initialization_actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_config`.\n"]
    pub fn set_lifecycle_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lifecycle_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_config`.\n"]
    pub fn set_master_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.master_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.master_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_worker_config`.\n"]
    pub fn set_secondary_worker_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secondary_worker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secondary_worker_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_config`.\n"]
    pub fn set_security_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.software_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.software_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker_config`.\n"]
    pub fn set_worker_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl {
            staging_bucket: core::default::Default::default(),
            temp_bucket: core::default::Default::default(),
            autoscaling_config: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            endpoint_config: core::default::Default::default(),
            gce_cluster_config: core::default::Default::default(),
            initialization_actions: core::default::Default::default(),
            lifecycle_config: core::default::Default::default(),
            master_config: core::default::Default::default(),
            secondary_worker_config: core::default::Default::default(),
            security_config: core::default::Default::default(),
            software_config: core::default::Default::default(),
            worker_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElConfigElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `staging_bucket` after provisioning.\nOptional. A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see [Dataproc staging bucket](https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)). **This field requires a Cloud Storage bucket name, not a URI to a Cloud Storage bucket.**"]
    pub fn staging_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temp_bucket` after provisioning.\nOptional. A Cloud Storage bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. If you do not specify a temp bucket, Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's temp bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket. The default bucket has a TTL of 90 days, but you can use any TTL (or none) if you specify a bucket. **This field requires a Cloud Storage bucket name, not a URI to a Cloud Storage bucket.**"]
    pub fn temp_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temp_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElAutoscalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElEndpointConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_cluster_config` after provisioning.\n"]
    pub fn gce_cluster_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElGceClusterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_cluster_config", self.base))
    }

    #[doc = "Get a reference to the value of field `initialization_actions` after provisioning.\n"]
    pub fn initialization_actions(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElInitializationActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initialization_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config` after provisioning.\n"]
    pub fn lifecycle_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElLifecycleConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_config", self.base))
    }

    #[doc = "Get a reference to the value of field `master_config` after provisioning.\n"]
    pub fn master_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElMasterConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_config", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_worker_config` after provisioning.\n"]
    pub fn secondary_worker_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecondaryWorkerConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_worker_config", self.base))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.base))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElSoftwareConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElWorkerConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElManagedClusterElDynamic {
    config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementElManagedClusterEl {
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl>>,
    dynamic: DataprocWorkflowTemplatePlacementElManagedClusterElDynamic,
}

impl DataprocWorkflowTemplatePlacementElManagedClusterEl {
    #[doc =
        "Set the field `labels`.\nOptional. The labels to associate with this cluster. Label keys must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: p{Ll}p{Lo}{0,62} Label values must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: [p{Ll}p{Lo}p{N}_-]{0,63} No more than 32 labels can be associated with a given cluster."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterElConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementElManagedClusterEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementElManagedClusterEl {
    #[doc =
        "Required. The cluster name prefix. A unique cluster name will be formed by appending a random suffix. The name must contain only lower-case letters (a-z), numbers (0-9), and hyphens (-). Must begin with a letter. Cannot begin or end with hyphen. Must consist of between 2 and 35 characters."]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplatePlacementElManagedClusterEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementElManagedClusterEl {
        DataprocWorkflowTemplatePlacementElManagedClusterEl {
            cluster_name: self.cluster_name,
            labels: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElManagedClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElManagedClusterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplatePlacementElManagedClusterElRef {
        DataprocWorkflowTemplatePlacementElManagedClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElManagedClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nRequired. The cluster name prefix. A unique cluster name will be formed by appending a random suffix. The name must contain only lower-case letters (a-z), numbers (0-9), and hyphens (-). Must begin with a letter. Cannot begin or end with hyphen. Must consist of between 2 and 35 characters."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this cluster. Label keys must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: p{Ll}p{Lo}{0,62} Label values must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: [p{Ll}p{Lo}p{N}_-]{0,63} No more than 32 labels can be associated with a given cluster."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplatePlacementElDynamic {
    cluster_selector: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElClusterSelectorEl>>,
    managed_cluster: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementElManagedClusterEl>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplatePlacementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_selector: std::option::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElClusterSelectorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_cluster: std::option::Option<std::vec::Vec<DataprocWorkflowTemplatePlacementElManagedClusterEl>>,
    dynamic: DataprocWorkflowTemplatePlacementElDynamic,
}

impl DataprocWorkflowTemplatePlacementEl {
    #[doc = "Set the field `cluster_selector`.\n"]
    pub fn set_cluster_selector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElClusterSelectorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cluster_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_cluster`.\n"]
    pub fn set_managed_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplatePlacementElManagedClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_cluster = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplatePlacementEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplatePlacementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplatePlacementEl {}

impl BuildDataprocWorkflowTemplatePlacementEl {
    pub fn build(self) -> DataprocWorkflowTemplatePlacementEl {
        DataprocWorkflowTemplatePlacementEl {
            cluster_selector: core::default::Default::default(),
            managed_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplatePlacementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplatePlacementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplatePlacementElRef {
        DataprocWorkflowTemplatePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplatePlacementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_selector` after provisioning.\n"]
    pub fn cluster_selector(&self) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElClusterSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_cluster` after provisioning.\n"]
    pub fn managed_cluster(&self) -> terrars::ListRef<DataprocWorkflowTemplatePlacementElManagedClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_cluster", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplateTimeoutsEl {
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
}

impl terrars::ToListMappable for DataprocWorkflowTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplateTimeoutsEl {}

impl BuildDataprocWorkflowTemplateTimeoutsEl {
    pub fn build(self) -> DataprocWorkflowTemplateTimeoutsEl {
        DataprocWorkflowTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DataprocWorkflowTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplateTimeoutsElRef {
        DataprocWorkflowTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplateTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateDynamic {
    jobs: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateJobsEl>>,
    parameters: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplateParametersEl>>,
    placement: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplatePlacementEl>>,
}
