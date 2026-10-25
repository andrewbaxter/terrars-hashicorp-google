#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocWorkflowTemplateData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    encryption_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jobs: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocWorkflowTemplate_Timeouts>,
    dynamic: DataprocWorkflowTemplateDynamic,
}

struct DataprocWorkflowTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocWorkflowTemplateData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jobs`.\n"]
    pub fn set_jobs(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Placement>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocWorkflowTemplate_Timeouts>) -> Self {
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jobs` after provisioning.\n"]
    pub fn jobs(&self) -> terrars::ListRef<DataprocWorkflowTemplate_JobsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DataprocWorkflowTemplate_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocWorkflowTemplate_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocWorkflowTemplate_TimeoutsRef {
        DataprocWorkflowTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataprocWorkflowTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dag_timeout: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                version: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jobs` after provisioning.\n"]
    pub fn jobs(&self) -> terrars::ListRef<DataprocWorkflowTemplate_JobsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DataprocWorkflowTemplate_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocWorkflowTemplate_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocWorkflowTemplate_TimeoutsRef {
        DataprocWorkflowTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplateDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_EncryptionConfig>>,
    jobs: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs>>,
    parameters: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Parameters>>,
    placement: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_EncryptionConfig {
    #[doc = "Set the field `kms_key`.\nOptional. The Cloud KMS key name to use for encryption."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_EncryptionConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_EncryptionConfig {}

impl BuildDataprocWorkflowTemplate_EncryptionConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_EncryptionConfig {
        DataprocWorkflowTemplate_EncryptionConfig { kms_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_EncryptionConfigRef {
        DataprocWorkflowTemplate_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nOptional. The Cloud KMS key name to use for encryption."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_log_levels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig {}

impl BuildDataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig {
        DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig { driver_log_levels: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef {
        DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef {
    #[allow(dead_code)]
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
struct DataprocWorkflowTemplate_Jobs_HadoopJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_HadoopJob {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    dynamic: DataprocWorkflowTemplate_Jobs_HadoopJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_HadoopJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_HadoopJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_HadoopJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_HadoopJob {}

impl BuildDataprocWorkflowTemplate_Jobs_HadoopJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_HadoopJob {
        DataprocWorkflowTemplate_Jobs_HadoopJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_HadoopJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_HadoopJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_HadoopJobRef {
        DataprocWorkflowTemplate_Jobs_HadoopJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_HadoopJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_PigJob_QueryList {
    queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Jobs_PigJob_QueryList { }

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_PigJob_QueryList {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_PigJob_QueryList {
    #[doc =
        "Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } }"]
    pub queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplate_Jobs_PigJob_QueryList {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_PigJob_QueryList {
        DataprocWorkflowTemplate_Jobs_PigJob_QueryList { queries: self.queries }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef {
        DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef {
    #[allow(dead_code)]
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
struct DataprocWorkflowTemplate_Jobs_HiveJobDynamic {
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_HiveJob {
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
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
    dynamic: DataprocWorkflowTemplate_Jobs_HiveJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_HiveJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_HiveJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_HiveJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_HiveJob {}

impl BuildDataprocWorkflowTemplate_Jobs_HiveJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_HiveJob {
        DataprocWorkflowTemplate_Jobs_HiveJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_HiveJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_HiveJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_HiveJobRef {
        DataprocWorkflowTemplate_Jobs_HiveJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_HiveJobRef {
    #[allow(dead_code)]
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
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Jobs_PigJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_PigJob {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
    dynamic: DataprocWorkflowTemplate_Jobs_PigJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_PigJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_PigJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_PigJob {}

impl BuildDataprocWorkflowTemplate_Jobs_PigJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_PigJob {
        DataprocWorkflowTemplate_Jobs_PigJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_PigJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_PigJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_PigJobRef {
        DataprocWorkflowTemplate_Jobs_PigJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_PigJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Jobs_PrestoJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_PrestoJob {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
    dynamic: DataprocWorkflowTemplate_Jobs_PrestoJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_PrestoJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_PrestoJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PrestoJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_PrestoJob {}

impl BuildDataprocWorkflowTemplate_Jobs_PrestoJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_PrestoJob {
        DataprocWorkflowTemplate_Jobs_PrestoJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_PrestoJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_PrestoJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_PrestoJobRef {
        DataprocWorkflowTemplate_Jobs_PrestoJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_PrestoJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Jobs_PysparkJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_PysparkJob {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    dynamic: DataprocWorkflowTemplate_Jobs_PysparkJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_PysparkJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_PysparkJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PysparkJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_PysparkJob {
    #[doc = "Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub main_python_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate_Jobs_PysparkJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_PysparkJob {
        DataprocWorkflowTemplate_Jobs_PysparkJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_PysparkJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_PysparkJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_PysparkJobRef {
        DataprocWorkflowTemplate_Jobs_PysparkJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_PysparkJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_Scheduling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_failures_per_hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_failures_total: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplate_Jobs_Scheduling {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_Scheduling {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_Scheduling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_Scheduling {}

impl BuildDataprocWorkflowTemplate_Jobs_Scheduling {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_Scheduling {
        DataprocWorkflowTemplate_Jobs_Scheduling {
            max_failures_per_hour: core::default::Default::default(),
            max_failures_total: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_SchedulingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_SchedulingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_SchedulingRef {
        DataprocWorkflowTemplate_Jobs_SchedulingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_SchedulingRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Jobs_SparkJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_SparkJob {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    dynamic: DataprocWorkflowTemplate_Jobs_SparkJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_SparkJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_SparkJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_SparkJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_SparkJob {}

impl BuildDataprocWorkflowTemplate_Jobs_SparkJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_SparkJob {
        DataprocWorkflowTemplate_Jobs_SparkJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_SparkJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_SparkJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_SparkJobRef {
        DataprocWorkflowTemplate_Jobs_SparkJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_SparkJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Jobs_SparkRJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_SparkRJob {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    dynamic: DataprocWorkflowTemplate_Jobs_SparkRJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_SparkRJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_SparkRJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_SparkRJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_SparkRJob {
    #[doc = "Required. The HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub main_r_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate_Jobs_SparkRJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_SparkRJob {
        DataprocWorkflowTemplate_Jobs_SparkRJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_SparkRJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_SparkRJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_SparkRJobRef {
        DataprocWorkflowTemplate_Jobs_SparkRJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_SparkRJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Jobs_SparkSqlJobDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    query_list: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs_SparkSqlJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
    dynamic: DataprocWorkflowTemplate_Jobs_SparkSqlJobDynamic,
}

impl DataprocWorkflowTemplate_Jobs_SparkSqlJob {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob_QueryList>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs_SparkSqlJob {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_SparkSqlJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs_SparkSqlJob {}

impl BuildDataprocWorkflowTemplate_Jobs_SparkSqlJob {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs_SparkSqlJob {
        DataprocWorkflowTemplate_Jobs_SparkSqlJob {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Jobs_SparkSqlJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Jobs_SparkSqlJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_Jobs_SparkSqlJobRef {
        DataprocWorkflowTemplate_Jobs_SparkSqlJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Jobs_SparkSqlJobRef {
    #[allow(dead_code)]
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJob_QueryListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_JobsDynamic {
    hadoop_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_HadoopJob>>,
    hive_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_HiveJob>>,
    pig_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PigJob>>,
    presto_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PrestoJob>>,
    pyspark_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_PysparkJob>>,
    scheduling: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_Scheduling>>,
    spark_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_SparkJob>>,
    spark_r_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_SparkRJob>>,
    spark_sql_job: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Jobs_SparkSqlJob>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Jobs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prerequisite_step_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    step_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hadoop_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_HadoopJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_HiveJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pig_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PigJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    presto_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PrestoJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pyspark_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_PysparkJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_Scheduling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_SparkJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_r_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_SparkRJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_sql_job: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Jobs_SparkSqlJob>>,
    dynamic: DataprocWorkflowTemplate_JobsDynamic,
}

impl DataprocWorkflowTemplate_Jobs {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_HadoopJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_HiveJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PigJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PrestoJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_PysparkJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_Scheduling>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_SparkJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_SparkRJob>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs_SparkSqlJob>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Jobs {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Jobs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Jobs {
    #[doc =
        "Required. The step id. The id must be unique among all jobs within the template. The step id is used as prefix for job id, as job `goog-dataproc-workflow-step-id` label, and in prerequisiteStepIds field from other steps. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters."]
    pub step_id: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate_Jobs {
    pub fn build(self) -> DataprocWorkflowTemplate_Jobs {
        DataprocWorkflowTemplate_Jobs {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_JobsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_JobsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_JobsRef {
        DataprocWorkflowTemplate_JobsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_JobsRef {
    #[allow(dead_code)]
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
    pub fn hadoop_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_HadoopJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_job", self.base))
    }

    #[doc = "Get a reference to the value of field `hive_job` after provisioning.\n"]
    pub fn hive_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_HiveJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_job", self.base))
    }

    #[doc = "Get a reference to the value of field `pig_job` after provisioning.\n"]
    pub fn pig_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PigJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pig_job", self.base))
    }

    #[doc = "Get a reference to the value of field `presto_job` after provisioning.\n"]
    pub fn presto_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PrestoJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto_job", self.base))
    }

    #[doc = "Get a reference to the value of field `pyspark_job` after provisioning.\n"]
    pub fn pyspark_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_PysparkJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_job", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_SchedulingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_job` after provisioning.\n"]
    pub fn spark_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_SparkJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_job", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_r_job` after provisioning.\n"]
    pub fn spark_r_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_SparkRJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_r_job", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_sql_job` after provisioning.\n"]
    pub fn spark_sql_job(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Jobs_SparkSqlJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_job", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Parameters_Validation_Regex {
    regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Parameters_Validation_Regex { }

impl terrars::ToListMappable for DataprocWorkflowTemplate_Parameters_Validation_Regex {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters_Validation_Regex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Parameters_Validation_Regex {
    #[doc =
        "Required. RE2 regular expressions used to validate the parameter's value. The value must match the regex in its entirety (substring matches are not sufficient)."]
    pub regexes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplate_Parameters_Validation_Regex {
    pub fn build(self) -> DataprocWorkflowTemplate_Parameters_Validation_Regex {
        DataprocWorkflowTemplate_Parameters_Validation_Regex { regexes: self.regexes }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Parameters_Validation_RegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Parameters_Validation_RegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Parameters_Validation_RegexRef {
        DataprocWorkflowTemplate_Parameters_Validation_RegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Parameters_Validation_RegexRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Parameters_Validation_Values {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Parameters_Validation_Values { }

impl terrars::ToListMappable for DataprocWorkflowTemplate_Parameters_Validation_Values {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters_Validation_Values>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Parameters_Validation_Values {
    #[doc = "Required. List of allowed values for the parameter."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplate_Parameters_Validation_Values {
    pub fn build(self) -> DataprocWorkflowTemplate_Parameters_Validation_Values {
        DataprocWorkflowTemplate_Parameters_Validation_Values { values: self.values }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Parameters_Validation_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Parameters_Validation_ValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Parameters_Validation_ValuesRef {
        DataprocWorkflowTemplate_Parameters_Validation_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Parameters_Validation_ValuesRef {
    #[allow(dead_code)]
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
struct DataprocWorkflowTemplate_Parameters_ValidationDynamic {
    regex: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Parameters_Validation_Regex>>,
    values: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Parameters_Validation_Values>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Parameters_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Parameters_Validation_Regex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Parameters_Validation_Values>>,
    dynamic: DataprocWorkflowTemplate_Parameters_ValidationDynamic,
}

impl DataprocWorkflowTemplate_Parameters_Validation {
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters_Validation_Regex>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters_Validation_Values>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Parameters_Validation {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Parameters_Validation {}

impl BuildDataprocWorkflowTemplate_Parameters_Validation {
    pub fn build(self) -> DataprocWorkflowTemplate_Parameters_Validation {
        DataprocWorkflowTemplate_Parameters_Validation {
            regex: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Parameters_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Parameters_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Parameters_ValidationRef {
        DataprocWorkflowTemplate_Parameters_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Parameters_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Parameters_Validation_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Parameters_Validation_ValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_ParametersDynamic {
    validation: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Parameters_Validation>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Parameters_Validation>>,
    dynamic: DataprocWorkflowTemplate_ParametersDynamic,
}

impl DataprocWorkflowTemplate_Parameters {
    #[doc =
        "Set the field `description`.\nOptional. Brief description of the parameter. Must not exceed 1024 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters_Validation>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Parameters {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Parameters {
    #[doc =
        "Required. Paths to all fields that the parameter replaces. A field is allowed to appear in at most one parameter's list of field paths. A field path is similar in syntax to a google.protobuf.FieldMask. For example, a field path that references the zone field of a workflow template's cluster selector would be specified as `placement.clusterSelector.zone`. Also, field paths can reference fields using the following syntax: * Values in maps can be referenced by key: * labels['key'] * placement.clusterSelector.clusterLabels['key'] * placement.managedCluster.labels['key'] * placement.clusterSelector.clusterLabels['key'] * jobs['step-id'].labels['key'] * Jobs in the jobs list can be referenced by step-id: * jobs['step-id'].hadoopJob.mainJarFileUri * jobs['step-id'].hiveJob.queryFileUri * jobs['step-id'].pySparkJob.mainPythonFileUri * jobs['step-id'].hadoopJob.jarFileUris[0] * jobs['step-id'].hadoopJob.archiveUris[0] * jobs['step-id'].hadoopJob.fileUris[0] * jobs['step-id'].pySparkJob.pythonFileUris[0] * Items in repeated fields can be referenced by a zero-based index: * jobs['step-id'].sparkJob.args[0] * Other examples: * jobs['step-id'].hadoopJob.properties['key'] * jobs['step-id'].hadoopJob.args[0] * jobs['step-id'].hiveJob.scriptVariables['key'] * jobs['step-id'].hadoopJob.mainJarFileUri * placement.clusterSelector.zone It may not be possible to parameterize maps and repeated fields in their entirety since only individual map values and individual items in repeated fields can be referenced. For example, the following field paths are invalid: - placement.clusterSelector.clusterLabels - jobs['step-id'].sparkJob.args"]
    pub fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "Required. Parameter name. The parameter name is used as the key, and paired with the parameter value, which are passed to the template when the template is instantiated. The name must contain only capital letters (A-Z), numbers (0-9), and underscores (_), and must not start with a number. The maximum length is 40 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate_Parameters {
    pub fn build(self) -> DataprocWorkflowTemplate_Parameters {
        DataprocWorkflowTemplate_Parameters {
            description: core::default::Default::default(),
            fields: self.fields,
            name: self.name,
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_ParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_ParametersRef {
        DataprocWorkflowTemplate_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_ParametersRef {
    #[allow(dead_code)]
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
    pub fn validation(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Parameters_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ClusterSelector {
    cluster_labels: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ClusterSelector {
    #[doc =
        "Set the field `zone`.\nOptional. The zone where workflow process executes. This parameter does not affect the selection of the cluster. If unspecified, the zone of the first cluster matching the selector is used."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ClusterSelector {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ClusterSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ClusterSelector {
    #[doc = "Required. The cluster labels. Cluster must have all labels to match."]
    pub cluster_labels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocWorkflowTemplate_Placement_ClusterSelector {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ClusterSelector {
        DataprocWorkflowTemplate_Placement_ClusterSelector {
            cluster_labels: self.cluster_labels,
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ClusterSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ClusterSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ClusterSelectorRef {
        DataprocWorkflowTemplate_Placement_ClusterSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ClusterSelectorRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {
    #[doc =
        "Set the field `policy`.\nOptional. The autoscaling policy used by the cluster. Only resource names including projectid and location (region) are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]` * `projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]` Note that the policy must be in the same project and Dataproc region."]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig {
            policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfigRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_pd_kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig {
            gce_pd_kms_key_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfigRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_http_port_access: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {
    #[doc =
        "Set the field `enable_http_port_access`.\nOptional. If true, enable http access to specific ports on the cluster from external sources. Defaults to false."]
    pub fn set_enable_http_port_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_http_port_access = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig {
            enable_http_port_access: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfigRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity {
    node_group: terrars::PrimField<std::string::String>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity { }

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity {
    #[doc =
        "Required. The URI of a sole-tenant [node group resource](https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups) that the cluster will be created on. A full URL, partial URI, or node group name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1` * `projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1` * `node-group-1`"]
    pub node_group: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity {
            node_group: self.node_group,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinityRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinityRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consume_reservation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {
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
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinityRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinityRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {
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
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
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
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigDynamic {
    node_group_affinity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity>,
    >,
    reservation_affinity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity>,
    >,
    shielded_instance_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {
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
        std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std
    ::option
    ::Option<
        std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig>,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinity,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinity,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfig,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigRef {
    #[allow(dead_code)]
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
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_NodeGroupAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_group_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    executable_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions {
            executable_file: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActionsRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActionsRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_delete_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig {
            auto_delete_time: core::default::Default::default(),
            auto_delete_ttl: core::default::Default::default(),
            idle_delete_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfigRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_group_manager_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {
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
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {
    type O =
        terrars
        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfig {
            instance_group_manager_name: core::default::Default::default(),
            instance_template_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef {
    #[allow(dead_code)]
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
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {
    #[doc = "Set the field `disk_size_gb`.\nOptional. Size of the attached disk, specified in GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nOptional. The disk type of the attached disk. Currently only supports Hyperdisks: `hyperdisk-balanced`, `hyperdisk-extreme`, `hyperdisk-ml`, `hyperdisk-throughput`."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_iops`.\nOptional. Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_throughput`.\nOptional. Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. Attached disk type. Currently only supports Hyperdisks: `hyperdisk-balanced`, `hyperdisk-extreme`, `hyperdisk-ml`, `hyperdisk-throughput`."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. Size of the attached disk, specified in GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nOptional. The disk type of the attached disk. Currently only supports Hyperdisks: `hyperdisk-balanced`, `hyperdisk-extreme`, `hyperdisk-ml`, `hyperdisk-throughput`."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_iops` after provisioning.\nOptional. Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_throughput` after provisioning.\nOptional. Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. Attached disk type. Currently only supports Hyperdisks: `hyperdisk-balanced`, `hyperdisk-extreme`, `hyperdisk-ml`, `hyperdisk-throughput`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigDynamic {
    attached_disk_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_local_ssds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_disk_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig>,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {
    #[doc =
        "Set the field `boot_disk_provisioned_iops`.\nOptional. Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle. This field is supported only if boot_disk_type is hyperdisk-balanced."]
    pub fn set_boot_disk_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_provisioned_throughput`.\nOptional. Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle. Values must be greater than or equal to 1. This field is supported only if boot_disk_type is hyperdisk-balanced."]
    pub fn set_boot_disk_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

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
        "Set the field `local_ssd_interface`.\nOptional. Interface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn set_local_ssd_interface(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.local_ssd_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `num_local_ssds`.\nOptional. Number of attached SSDs, from 0 to 8 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn set_num_local_ssds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_local_ssds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_disk_config`.\n"]
    pub fn set_attached_disk_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attached_disk_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attached_disk_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig {
            boot_disk_provisioned_iops: core::default::Default::default(),
            boot_disk_provisioned_throughput: core::default::Default::default(),
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
            local_ssd_interface: core::default::Default::default(),
            num_local_ssds: core::default::Default::default(),
            attached_disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_iops` after provisioning.\nOptional. Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle. This field is supported only if boot_disk_type is hyperdisk-balanced."]
    pub fn boot_disk_provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_provisioned_throughput` after provisioning.\nOptional. Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle. Values must be greater than or equal to 1. This field is supported only if boot_disk_type is hyperdisk-balanced."]
    pub fn boot_disk_provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_provisioned_throughput", self.base))
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
        "Get a reference to the value of field `local_ssd_interface` after provisioning.\nOptional. Interface type of local SSDs (default is \"scsi\"). Valid values: \"scsi\" (Small Computer System Interface), \"nvme\" (Non-Volatile Memory Express)."]
    pub fn local_ssd_interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd_interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_local_ssds` after provisioning.\nOptional. Number of attached SSDs, from 0 to 8 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and [HDFS](https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub fn num_local_ssds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_local_ssds", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_disk_config` after provisioning.\n"]
    pub fn attached_disk_config(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig_AttachedDiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_count`.\n"]
    pub fn set_vm_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResults {
            machine_type: core::default::Default::default(),
            vm_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_count` after provisioning.\n"]
    pub fn vm_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListDynamic {
    disk_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rank: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    #[doc = "Set the field `machine_types`.\nOptional. Full machine-type names, e.g. \"n1-standard-16\"."]
    pub fn set_machine_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.machine_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rank`.\nOptional. Preference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."]
    pub fn set_rank(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rank = std::option::Option::Some(v.into());
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig,
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

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList {
            machine_types: core::default::Default::default(),
            rank: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_types` after provisioning.\nOptional. Full machine-type names, e.g. \"n1-standard-16\"."]
    pub fn machine_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rank` after provisioning.\nOptional. Preference of this instance selection. Lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference."]
    pub fn rank(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyDynamic {
    instance_selection_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selection_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
        >,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_selection_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy {
            instance_selection_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_machine_types` after provisioning.\nOutput only. A map of instance names to their machine type."]
    pub fn instance_machine_types(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instance_machine_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_selection_results` after provisioning.\nOutput only. A list of instance selection results that were successfully allocated."]
    pub fn instance_selection_results(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_results", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]
    pub fn instance_selection_list(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigDynamic {
    accelerators: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>,
    >,
    disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    instance_flexibility_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {
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
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<
        std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy>,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig,
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

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig {
            image: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigRef {
    #[allow(dead_code)]
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
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef> {
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
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_capacity_base: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_capacity_percent_above_base: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    #[doc =
        "Set the field `standard_capacity_base`.\nOptional. The base capacity that will always use Standard VMs to avoid risk of premature allocation."]
    pub fn set_standard_capacity_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.standard_capacity_base = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `standard_capacity_percent_above_base`.\nOptional. The percentage of target capacity that will use Standard VMs above standardCapacityBase."]
    pub fn set_standard_capacity_percent_above_base(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.standard_capacity_percent_above_base = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix {
            standard_capacity_base: core::default::Default::default(),
            standard_capacity_percent_above_base: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `standard_capacity_base` after provisioning.\nOptional. The base capacity that will always use Standard VMs to avoid risk of premature allocation."]
    pub fn standard_capacity_base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standard_capacity_base", self.base))
    }

    #[doc =
        "Get a reference to the value of field `standard_capacity_percent_above_base` after provisioning.\nOptional. The percentage of target capacity that will use Standard VMs above standardCapacityBase."]
    pub fn standard_capacity_percent_above_base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standard_capacity_percent_above_base", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyDynamic {
    instance_selection_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
        >,
    >,
    provisioning_model_mix: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selection_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_model_mix: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
        >,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {
    #[doc = "Set the field `instance_selection_list`.\n"]
    pub fn set_instance_selection_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_selection_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selection_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provisioning_model_mix`.\n"]
    pub fn set_provisioning_model_mix(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMix,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provisioning_model_mix = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioning_model_mix = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {
    type O =
        terrars
        ::BlockAssignable<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {
    pub fn build(
        self,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy {
            instance_selection_list: core::default::Default::default(),
            provisioning_model_mix: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_machine_types` after provisioning.\nOutput only. A map of instance names to their machine type."]
    pub fn instance_machine_types(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instance_machine_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_selection_results` after provisioning.\nOutput only. A list of instance selection results that were successfully allocated."]
    pub fn instance_selection_results(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionResultsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_results", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_selection_list` after provisioning.\n"]
    pub fn instance_selection_list(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy_InstanceSelectionListRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selection_list", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioning_model_mix` after provisioning.\n"]
    pub fn provisioning_model_mix(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy_ProvisioningModelMixRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_model_mix", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigDynamic {
    accelerators: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>,
    >,
    disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    instance_flexibility_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {
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
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy,
        >,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig,
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

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {
    type O =
        terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig {
            image: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigRef {
    #[allow(dead_code)]
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
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef> {
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
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars
    ::ListRef<
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig_InstanceFlexibilityPolicyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {
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

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {
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
::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {
    type O =
        terrars
        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfigRef {
    #[allow(dead_code)]
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
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigDynamic {
    kerberos_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig>>,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfig,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig {
            kerberos_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig_KerberosConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional_components: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig {
            image_version: core::default::Default::default(),
            optional_components: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfigRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigDynamic {
    accelerators: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>,
    >,
    disk_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    instance_flexibility_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {
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
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<
        std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy>,
    >,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_Accelerators,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfig,
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

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig {
            image: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            num_instances: core::default::Default::default(),
            preemptibility: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            disk_config: core::default::Default::default(),
            instance_flexibility_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigRef {
    #[allow(dead_code)]
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
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_ManagedGroupConfigRef> {
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
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_config` after provisioning.\n"]
    pub fn disk_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_DiskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars
    ::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigDynamic {
    autoscaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig>>,
    encryption_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig>>,
    endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig>>,
    gce_cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig>>,
    initialization_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions>>,
    lifecycle_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig>>,
    master_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig>>,
    secondary_worker_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig>>,
    security_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig>>,
    software_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig>>,
    worker_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temp_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_cluster_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialization_actions: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_worker_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_config: std
    ::option
    ::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig>>,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_Config {
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfig>,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfig>,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfig>,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfig>,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActions,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfig>,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfig>,
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
                            DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfig,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfig>,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfig>,
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
                        ::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfig>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster_Config {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config {}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster_Config {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster_Config {
        DataprocWorkflowTemplate_Placement_ManagedCluster_Config {
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

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigRef {
        DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_config` after provisioning.\n"]
    pub fn endpoint_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_EndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_cluster_config` after provisioning.\n"]
    pub fn gce_cluster_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_GceClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_cluster_config", self.base))
    }

    #[doc = "Get a reference to the value of field `initialization_actions` after provisioning.\n"]
    pub fn initialization_actions(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_InitializationActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initialization_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_config` after provisioning.\n"]
    pub fn lifecycle_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_LifecycleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_config", self.base))
    }

    #[doc = "Get a reference to the value of field `master_config` after provisioning.\n"]
    pub fn master_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_MasterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_config", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_worker_config` after provisioning.\n"]
    pub fn secondary_worker_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecondaryWorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_worker_config", self.base))
    }

    #[doc = "Get a reference to the value of field `security_config` after provisioning.\n"]
    pub fn security_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_config", self.base))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_SoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(
        &self,
    ) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_Config_WorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_Placement_ManagedClusterDynamic {
    config: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster_Config>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement_ManagedCluster {
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster_Config>>,
    dynamic: DataprocWorkflowTemplate_Placement_ManagedClusterDynamic,
}

impl DataprocWorkflowTemplate_Placement_ManagedCluster {
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
                    ::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster_Config>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement_ManagedCluster {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement_ManagedCluster {
    #[doc =
        "Required. The cluster name prefix. A unique cluster name will be formed by appending a random suffix. The name must contain only lower-case letters (a-z), numbers (0-9), and hyphens (-). Must begin with a letter. Cannot begin or end with hyphen. Must consist of between 2 and 35 characters."]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocWorkflowTemplate_Placement_ManagedCluster {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement_ManagedCluster {
        DataprocWorkflowTemplate_Placement_ManagedCluster {
            cluster_name: self.cluster_name,
            labels: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_Placement_ManagedClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_Placement_ManagedClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocWorkflowTemplate_Placement_ManagedClusterRef {
        DataprocWorkflowTemplate_Placement_ManagedClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_Placement_ManagedClusterRef {
    #[allow(dead_code)]
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
    pub fn config(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedCluster_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocWorkflowTemplate_PlacementDynamic {
    cluster_selector: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ClusterSelector>>,
    managed_cluster: std::option::Option<terrars::DynamicBlock<DataprocWorkflowTemplate_Placement_ManagedCluster>>,
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_selector: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ClusterSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_cluster: std::option::Option<std::vec::Vec<DataprocWorkflowTemplate_Placement_ManagedCluster>>,
    dynamic: DataprocWorkflowTemplate_PlacementDynamic,
}

impl DataprocWorkflowTemplate_Placement {
    #[doc = "Set the field `cluster_selector`.\n"]
    pub fn set_cluster_selector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ClusterSelector>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocWorkflowTemplate_Placement_ManagedCluster>>,
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Placement {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Placement {}

impl BuildDataprocWorkflowTemplate_Placement {
    pub fn build(self) -> DataprocWorkflowTemplate_Placement {
        DataprocWorkflowTemplate_Placement {
            cluster_selector: core::default::Default::default(),
            managed_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_PlacementRef {
        DataprocWorkflowTemplate_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_PlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_selector` after provisioning.\n"]
    pub fn cluster_selector(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ClusterSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_cluster` after provisioning.\n"]
    pub fn managed_cluster(&self) -> terrars::ListRef<DataprocWorkflowTemplate_Placement_ManagedClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_cluster", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocWorkflowTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocWorkflowTemplate_Timeouts {
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

impl terrars::ToListMappable for DataprocWorkflowTemplate_Timeouts {
    type O = terrars::BlockAssignable<DataprocWorkflowTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocWorkflowTemplate_Timeouts {}

impl BuildDataprocWorkflowTemplate_Timeouts {
    pub fn build(self) -> DataprocWorkflowTemplate_Timeouts {
        DataprocWorkflowTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocWorkflowTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocWorkflowTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocWorkflowTemplate_TimeoutsRef {
        DataprocWorkflowTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocWorkflowTemplate_TimeoutsRef {
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
