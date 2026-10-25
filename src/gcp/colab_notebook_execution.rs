#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ColabNotebookExecutionData_ {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_user: std::option::Option<terrars::PrimField<std::string::String>>,
    gcs_output_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_execution_job_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notebook_runtime_template_resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_environment_spec: std::option::Option<std::vec::Vec<ColabNotebookExecution_CustomEnvironmentSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataform_repository_source: std::option::Option<std::vec::Vec<ColabNotebookExecution_DataformRepositorySource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_notebook_source: std::option::Option<std::vec::Vec<ColabNotebookExecution_DirectNotebookSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_notebook_source: std::option::Option<std::vec::Vec<ColabNotebookExecution_GcsNotebookSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ColabNotebookExecution_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workbench_runtime: std::option::Option<std::vec::Vec<ColabNotebookExecution_WorkbenchRuntime>>,
    dynamic: ColabNotebookExecutionDynamic,
}

struct ColabNotebookExecution_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ColabNotebookExecutionData_>,
}

#[derive(Clone)]
pub struct ColabNotebookExecution(std::rc::Rc<ColabNotebookExecution_>);

impl ColabNotebookExecution {
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
        "Set the field `execution_timeout`.\nMax running time of the execution job in seconds (default 86400s / 24 hrs)."]
    pub fn set_execution_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_user`.\nThe user email to run the execution as."]
    pub fn set_execution_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notebook_execution_job_id`.\nUser specified ID for the Notebook Execution Job"]
    pub fn set_notebook_execution_job_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().notebook_execution_job_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notebook_runtime_template_resource_name`.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn set_notebook_runtime_template_resource_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().notebook_runtime_template_resource_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nThe service account to run the execution as."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_environment_spec`.\n"]
    pub fn set_custom_environment_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_environment_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_environment_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dataform_repository_source`.\n"]
    pub fn set_dataform_repository_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabNotebookExecution_DataformRepositorySource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dataform_repository_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dataform_repository_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `direct_notebook_source`.\n"]
    pub fn set_direct_notebook_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabNotebookExecution_DirectNotebookSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().direct_notebook_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.direct_notebook_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_notebook_source`.\n"]
    pub fn set_gcs_notebook_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabNotebookExecution_GcsNotebookSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_notebook_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_notebook_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ColabNotebookExecution_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workbench_runtime`.\n"]
    pub fn set_workbench_runtime(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabNotebookExecution_WorkbenchRuntime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workbench_runtime = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workbench_runtime = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Notebook Execution."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_timeout` after provisioning.\nMax running time of the execution job in seconds (default 86400s / 24 hrs)."]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_user` after provisioning.\nThe user email to run the execution as."]
    pub fn execution_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcs_output_uri` after provisioning.\nThe Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub fn gcs_output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_output_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notebook_execution_job_id` after provisioning.\nUser specified ID for the Notebook Execution Job"]
    pub fn notebook_execution_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notebook_execution_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notebook_runtime_template_resource_name` after provisioning.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn notebook_runtime_template_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template_resource_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run the execution as."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_environment_spec` after provisioning.\n"]
    pub fn custom_environment_spec(&self) -> terrars::ListRef<ColabNotebookExecution_CustomEnvironmentSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_environment_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataform_repository_source` after provisioning.\n"]
    pub fn dataform_repository_source(&self) -> terrars::ListRef<ColabNotebookExecution_DataformRepositorySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataform_repository_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `direct_notebook_source` after provisioning.\n"]
    pub fn direct_notebook_source(&self) -> terrars::ListRef<ColabNotebookExecution_DirectNotebookSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.direct_notebook_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_notebook_source` after provisioning.\n"]
    pub fn gcs_notebook_source(&self) -> terrars::ListRef<ColabNotebookExecution_GcsNotebookSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_notebook_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabNotebookExecution_TimeoutsRef {
        ColabNotebookExecution_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workbench_runtime` after provisioning.\n"]
    pub fn workbench_runtime(&self) -> terrars::ListRef<ColabNotebookExecution_WorkbenchRuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workbench_runtime", self.extract_ref()))
    }
}

impl terrars::Referable for ColabNotebookExecution {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ColabNotebookExecution { }

impl terrars::ToListMappable for ColabNotebookExecution {
    type O = terrars::ListRef<ColabNotebookExecutionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ColabNotebookExecution_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_colab_notebook_execution".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildColabNotebookExecution {
    pub tf_id: std::string::String,
    #[doc = "Required. The display name of the Notebook Execution."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub gcs_output_uri: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildColabNotebookExecution {
    pub fn build(self, stack: &mut terrars::Stack) -> ColabNotebookExecution {
        let out = ColabNotebookExecution(std::rc::Rc::new(ColabNotebookExecution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ColabNotebookExecutionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                execution_timeout: core::default::Default::default(),
                execution_user: core::default::Default::default(),
                gcs_output_uri: self.gcs_output_uri,
                id: core::default::Default::default(),
                location: self.location,
                notebook_execution_job_id: core::default::Default::default(),
                notebook_runtime_template_resource_name: core::default::Default::default(),
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                custom_environment_spec: core::default::Default::default(),
                dataform_repository_source: core::default::Default::default(),
                direct_notebook_source: core::default::Default::default(),
                gcs_notebook_source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workbench_runtime: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ColabNotebookExecutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecutionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ColabNotebookExecutionRef {
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
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Notebook Execution."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_timeout` after provisioning.\nMax running time of the execution job in seconds (default 86400s / 24 hrs)."]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_user` after provisioning.\nThe user email to run the execution as."]
    pub fn execution_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_user", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcs_output_uri` after provisioning.\nThe Cloud Storage location to upload the result to. Format:'gs://bucket-name'"]
    pub fn gcs_output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_output_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notebook_execution_job_id` after provisioning.\nUser specified ID for the Notebook Execution Job"]
    pub fn notebook_execution_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notebook_execution_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notebook_runtime_template_resource_name` after provisioning.\nThe NotebookRuntimeTemplate to source compute configuration from."]
    pub fn notebook_runtime_template_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_runtime_template_resource_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run the execution as."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_environment_spec` after provisioning.\n"]
    pub fn custom_environment_spec(&self) -> terrars::ListRef<ColabNotebookExecution_CustomEnvironmentSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_environment_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dataform_repository_source` after provisioning.\n"]
    pub fn dataform_repository_source(&self) -> terrars::ListRef<ColabNotebookExecution_DataformRepositorySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataform_repository_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `direct_notebook_source` after provisioning.\n"]
    pub fn direct_notebook_source(&self) -> terrars::ListRef<ColabNotebookExecution_DirectNotebookSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.direct_notebook_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_notebook_source` after provisioning.\n"]
    pub fn gcs_notebook_source(&self) -> terrars::ListRef<ColabNotebookExecution_GcsNotebookSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_notebook_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabNotebookExecution_TimeoutsRef {
        ColabNotebookExecution_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workbench_runtime` after provisioning.\n"]
    pub fn workbench_runtime(&self) -> terrars::ListRef<ColabNotebookExecution_WorkbenchRuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workbench_runtime", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ColabNotebookExecutionDynamic {
    custom_environment_spec: std::option::Option<terrars::DynamicBlock<ColabNotebookExecution_CustomEnvironmentSpec>>,
    dataform_repository_source: std
    ::option
    ::Option<terrars::DynamicBlock<ColabNotebookExecution_DataformRepositorySource>>,
    direct_notebook_source: std::option::Option<terrars::DynamicBlock<ColabNotebookExecution_DirectNotebookSource>>,
    gcs_notebook_source: std::option::Option<terrars::DynamicBlock<ColabNotebookExecution_GcsNotebookSource>>,
    workbench_runtime: std::option::Option<terrars::DynamicBlock<ColabNotebookExecution_WorkbenchRuntime>>,
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators used by the runtime."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe Compute Engine machine type selected for the runtime."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {
    type O = terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {}

impl BuildColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {
    pub fn build(self) -> ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {
        ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_CustomEnvironmentSpec_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_CustomEnvironmentSpec_MachineSpecRef {
        ColabNotebookExecution_CustomEnvironmentSpec_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_CustomEnvironmentSpec_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators used by the runtime."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type selected for the runtime."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_internet_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {
    #[doc = "Set the field `enable_internet_access`.\nEnable public internet access for the runtime."]
    pub fn set_enable_internet_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_internet_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nThe name of the VPC that this runtime is in."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\nThe name of the subnetwork that this runtime is in."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {
    type O = terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {}

impl BuildColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {
    pub fn build(self) -> ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {
        ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec {
            enable_internet_access: core::default::Default::default(),
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpecRef {
        ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_internet_access` after provisioning.\nEnable public internet access for the runtime."]
    pub fn enable_internet_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_internet_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC that this runtime is in."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnetwork that this runtime is in."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {
    #[doc =
        "Set the field `disk_size_gb`.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\nThe type of the persistent disk."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {
    type O = terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {}

impl BuildColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {
    pub fn build(self) -> ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {
        ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpecRef {
        ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the persistent disk."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {
    #[doc =
        "Set the field `enable_integrity_monitoring`.\nDefines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created. Enabled by default."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_secure_boot`.\nDefines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails. Disabled by default."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_vtpm`.\nDefines whether the instance has the vTPM enabled. Enabled by default."]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {}

impl BuildColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {
    pub fn build(self) -> ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {
        ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfigRef {
        ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created. Enabled by default."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails. Disabled by default."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_vtpm` after provisioning.\nDefines whether the instance has the vTPM enabled. Enabled by default."]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabNotebookExecution_CustomEnvironmentSpecDynamic {
    machine_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec>>,
    network_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec>>,
    persistent_disk_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig>>,
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_CustomEnvironmentSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std::option::Option<std::vec::Vec<ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_spec: std::option::Option<std::vec::Vec<ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_disk_spec: std
    ::option
    ::Option<std::vec::Vec<ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<std::vec::Vec<ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig>>,
    dynamic: ColabNotebookExecution_CustomEnvironmentSpecDynamic,
}

impl ColabNotebookExecution_CustomEnvironmentSpec {
    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_MachineSpec>>,
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
                    ::Into<terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpec>>,
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
                    ::Into<terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpec>>,
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

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfig>,
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

impl terrars::ToListMappable for ColabNotebookExecution_CustomEnvironmentSpec {
    type O = terrars::BlockAssignable<ColabNotebookExecution_CustomEnvironmentSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_CustomEnvironmentSpec {}

impl BuildColabNotebookExecution_CustomEnvironmentSpec {
    pub fn build(self) -> ColabNotebookExecution_CustomEnvironmentSpec {
        ColabNotebookExecution_CustomEnvironmentSpec {
            machine_spec: core::default::Default::default(),
            network_spec: core::default::Default::default(),
            persistent_disk_spec: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_CustomEnvironmentSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_CustomEnvironmentSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_CustomEnvironmentSpecRef {
        ColabNotebookExecution_CustomEnvironmentSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_CustomEnvironmentSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<ColabNotebookExecution_CustomEnvironmentSpec_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(&self) -> terrars::ListRef<ColabNotebookExecution_CustomEnvironmentSpec_NetworkSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `persistent_disk_spec` after provisioning.\n"]
    pub fn persistent_disk_spec(
        &self,
    ) -> terrars::ListRef<ColabNotebookExecution_CustomEnvironmentSpec_PersistentDiskSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_disk_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<ColabNotebookExecution_CustomEnvironmentSpec_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_DataformRepositorySource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    dataform_repository_resource_name: terrars::PrimField<std::string::String>,
}

impl ColabNotebookExecution_DataformRepositorySource {
    #[doc =
        "Set the field `commit_sha`.\nThe commit SHA to read repository with. If unset, the file will be read at HEAD."]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_DataformRepositorySource {
    type O = terrars::BlockAssignable<ColabNotebookExecution_DataformRepositorySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_DataformRepositorySource {
    #[doc = "The resource name of the Dataform Repository."]
    pub dataform_repository_resource_name: terrars::PrimField<std::string::String>,
}

impl BuildColabNotebookExecution_DataformRepositorySource {
    pub fn build(self) -> ColabNotebookExecution_DataformRepositorySource {
        ColabNotebookExecution_DataformRepositorySource {
            commit_sha: core::default::Default::default(),
            dataform_repository_resource_name: self.dataform_repository_resource_name,
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_DataformRepositorySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_DataformRepositorySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_DataformRepositorySourceRef {
        ColabNotebookExecution_DataformRepositorySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_DataformRepositorySourceRef {
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
        "Get a reference to the value of field `dataform_repository_resource_name` after provisioning.\nThe resource name of the Dataform Repository."]
    pub fn dataform_repository_resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataform_repository_resource_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_DirectNotebookSource {
    content: terrars::PrimField<std::string::String>,
}

impl ColabNotebookExecution_DirectNotebookSource { }

impl terrars::ToListMappable for ColabNotebookExecution_DirectNotebookSource {
    type O = terrars::BlockAssignable<ColabNotebookExecution_DirectNotebookSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_DirectNotebookSource {
    #[doc = "The base64-encoded contents of the input notebook file."]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildColabNotebookExecution_DirectNotebookSource {
    pub fn build(self) -> ColabNotebookExecution_DirectNotebookSource {
        ColabNotebookExecution_DirectNotebookSource { content: self.content }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_DirectNotebookSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_DirectNotebookSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_DirectNotebookSourceRef {
        ColabNotebookExecution_DirectNotebookSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_DirectNotebookSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe base64-encoded contents of the input notebook file."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_GcsNotebookSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl ColabNotebookExecution_GcsNotebookSource {
    #[doc =
        "Set the field `generation`.\nThe version of the Cloud Storage object to read. If unset, the current version of the object is read. See https://cloud.google.com/storage/docs/metadata#generation-number."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_GcsNotebookSource {
    type O = terrars::BlockAssignable<ColabNotebookExecution_GcsNotebookSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_GcsNotebookSource {
    #[doc = "The Cloud Storage uri pointing to the ipynb file."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildColabNotebookExecution_GcsNotebookSource {
    pub fn build(self) -> ColabNotebookExecution_GcsNotebookSource {
        ColabNotebookExecution_GcsNotebookSource {
            generation: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_GcsNotebookSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_GcsNotebookSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabNotebookExecution_GcsNotebookSourceRef {
        ColabNotebookExecution_GcsNotebookSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_GcsNotebookSourceRef {
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
        "Get a reference to the value of field `uri` after provisioning.\nThe Cloud Storage uri pointing to the ipynb file."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabNotebookExecution_Timeouts {
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

impl terrars::ToListMappable for ColabNotebookExecution_Timeouts {
    type O = terrars::BlockAssignable<ColabNotebookExecution_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_Timeouts {}

impl BuildColabNotebookExecution_Timeouts {
    pub fn build(self) -> ColabNotebookExecution_Timeouts {
        ColabNotebookExecution_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabNotebookExecution_TimeoutsRef {
        ColabNotebookExecution_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_TimeoutsRef {
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
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_WorkbenchRuntime_VmImage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabNotebookExecution_WorkbenchRuntime_VmImage {
    #[doc =
        "Set the field `family`.\nUse this VM image family to find the image; the newest image in this family will be used."]
    pub fn set_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nUse VM image name to find the image."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe name of the Google Cloud project that this VM image belongs to. Format: {project_id}"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_WorkbenchRuntime_VmImage {
    type O = terrars::BlockAssignable<ColabNotebookExecution_WorkbenchRuntime_VmImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_WorkbenchRuntime_VmImage {}

impl BuildColabNotebookExecution_WorkbenchRuntime_VmImage {
    pub fn build(self) -> ColabNotebookExecution_WorkbenchRuntime_VmImage {
        ColabNotebookExecution_WorkbenchRuntime_VmImage {
            family: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_WorkbenchRuntime_VmImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_WorkbenchRuntime_VmImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabNotebookExecution_WorkbenchRuntime_VmImageRef {
        ColabNotebookExecution_WorkbenchRuntime_VmImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_WorkbenchRuntime_VmImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `family` after provisioning.\nUse this VM image family to find the image; the newest image in this family will be used."]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUse VM image name to find the image."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe name of the Google Cloud project that this VM image belongs to. Format: {project_id}"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabNotebookExecution_WorkbenchRuntimeDynamic {
    vm_image: std::option::Option<terrars::DynamicBlock<ColabNotebookExecution_WorkbenchRuntime_VmImage>>,
}

#[derive(Serialize)]
pub struct ColabNotebookExecution_WorkbenchRuntime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_image: std::option::Option<std::vec::Vec<ColabNotebookExecution_WorkbenchRuntime_VmImage>>,
    dynamic: ColabNotebookExecution_WorkbenchRuntimeDynamic,
}

impl ColabNotebookExecution_WorkbenchRuntime {
    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabNotebookExecution_WorkbenchRuntime_VmImage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vm_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_image = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabNotebookExecution_WorkbenchRuntime {
    type O = terrars::BlockAssignable<ColabNotebookExecution_WorkbenchRuntime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabNotebookExecution_WorkbenchRuntime {}

impl BuildColabNotebookExecution_WorkbenchRuntime {
    pub fn build(self) -> ColabNotebookExecution_WorkbenchRuntime {
        ColabNotebookExecution_WorkbenchRuntime {
            vm_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabNotebookExecution_WorkbenchRuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabNotebookExecution_WorkbenchRuntimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabNotebookExecution_WorkbenchRuntimeRef {
        ColabNotebookExecution_WorkbenchRuntimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabNotebookExecution_WorkbenchRuntimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> terrars::ListRef<ColabNotebookExecution_WorkbenchRuntime_VmImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_image", self.base))
    }
}
