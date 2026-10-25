#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocSessionTemplateData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_config: std::option::Option<std::vec::Vec<DataprocSessionTemplate_EnvironmentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jupyter_session: std::option::Option<std::vec::Vec<DataprocSessionTemplate_JupyterSession>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_config: std::option::Option<std::vec::Vec<DataprocSessionTemplate_RuntimeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_connect_session: std::option::Option<std::vec::Vec<DataprocSessionTemplate_SparkConnectSession>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocSessionTemplate_Timeouts>,
    dynamic: DataprocSessionTemplateDynamic,
}

struct DataprocSessionTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocSessionTemplateData_>,
}

#[derive(Clone)]
pub struct DataprocSessionTemplate(std::rc::Rc<DataprocSessionTemplate_>);

impl DataprocSessionTemplate {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels to associate with this session template.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location in which the session template will be created in."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_config`.\n"]
    pub fn set_environment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jupyter_session`.\n"]
    pub fn set_jupyter_session(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocSessionTemplate_JupyterSession>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jupyter_session = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jupyter_session = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocSessionTemplate_RuntimeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_connect_session`.\n"]
    pub fn set_spark_connect_session(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocSessionTemplate_SparkConnectSession>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_connect_session = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_connect_session = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocSessionTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the session template was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the session template."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this session template.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location in which the session template will be created in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the session template in the following format:\nprojects/{project}/locations/{location}/sessionTemplates/{template_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the session template was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uuid` after provisioning.\nA session template UUID (Unique Universal Identifier). The service generates this value when it creates the session template."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> terrars::ListRef<DataprocSessionTemplate_EnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jupyter_session` after provisioning.\n"]
    pub fn jupyter_session(&self) -> terrars::ListRef<DataprocSessionTemplate_JupyterSessionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_session", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> terrars::ListRef<DataprocSessionTemplate_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_connect_session` after provisioning.\n"]
    pub fn spark_connect_session(&self) -> terrars::ListRef<DataprocSessionTemplate_SparkConnectSessionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_connect_session", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocSessionTemplate_TimeoutsRef {
        DataprocSessionTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocSessionTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocSessionTemplate { }

impl terrars::ToListMappable for DataprocSessionTemplate {
    type O = terrars::ListRef<DataprocSessionTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocSessionTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_session_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocSessionTemplate {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of the session template in the following format:\nprojects/{project}/locations/{location}/sessionTemplates/{template_id}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocSessionTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocSessionTemplate {
        let out = DataprocSessionTemplate(std::rc::Rc::new(DataprocSessionTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocSessionTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                environment_config: core::default::Default::default(),
                jupyter_session: core::default::Default::default(),
                runtime_config: core::default::Default::default(),
                spark_connect_session: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocSessionTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocSessionTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the session template was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the session template."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this session template.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location in which the session template will be created in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the session template in the following format:\nprojects/{project}/locations/{location}/sessionTemplates/{template_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the session template was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uuid` after provisioning.\nA session template UUID (Unique Universal Identifier). The service generates this value when it creates the session template."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> terrars::ListRef<DataprocSessionTemplate_EnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `jupyter_session` after provisioning.\n"]
    pub fn jupyter_session(&self) -> terrars::ListRef<DataprocSessionTemplate_JupyterSessionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jupyter_session", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> terrars::ListRef<DataprocSessionTemplate_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_connect_session` after provisioning.\n"]
    pub fn spark_connect_session(&self) -> terrars::ListRef<DataprocSessionTemplate_SparkConnectSessionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_connect_session", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocSessionTemplate_TimeoutsRef {
        DataprocSessionTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocSessionTemplateDynamic {
    environment_config: std::option::Option<terrars::DynamicBlock<DataprocSessionTemplate_EnvironmentConfig>>,
    jupyter_session: std::option::Option<terrars::DynamicBlock<DataprocSessionTemplate_JupyterSession>>,
    runtime_config: std::option::Option<terrars::DynamicBlock<DataprocSessionTemplate_RuntimeConfig>>,
    spark_connect_session: std::option::Option<terrars::DynamicBlock<DataprocSessionTemplate_SparkConnectSession>>,
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_workload_authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    #[doc =
        "Set the field `user_workload_authentication_type`.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn set_user_workload_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_workload_authentication_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    type O =
        terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {}

impl BuildDataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    pub fn build(self) -> DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
        DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
            user_workload_authentication_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
        DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_workload_authentication_type` after provisioning.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn user_workload_authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_workload_authentication_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigDynamic {
    authentication_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>>,
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_config: std
    ::option
    ::Option<std::vec::Vec<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>>,
    dynamic: DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigDynamic,
}

impl DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {
    #[doc =
        "Set the field `idle_ttl`.\nThe duration to keep the session alive while it's idling.\nExceeding this threshold causes the session to terminate. Minimum value is 10 minutes; maximum value is 14 day.\nDefaults to 1 hour if not set. If both ttl and idleTtl are specified for an interactive session, the conditions\nare treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or when ttl has\nbeen exceeded, whichever occurs first."]
    pub fn set_idle_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key`.\nThe Cloud KMS key to use for encryption."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tags`.\nTags used for network traffic control."]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.network_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nService account that used to execute workload."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `staging_bucket`.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn set_staging_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.staging_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork_uri`.\nSubnetwork configuration for workload execution."]
    pub fn set_subnetwork_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a session workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {}

impl BuildDataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {
    pub fn build(self) -> DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {
        DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig {
            idle_ttl: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            service_account: core::default::Default::default(),
            staging_bucket: core::default::Default::default(),
            subnetwork_uri: core::default::Default::default(),
            ttl: core::default::Default::default(),
            authentication_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigRef {
        DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `idle_ttl` after provisioning.\nThe duration to keep the session alive while it's idling.\nExceeding this threshold causes the session to terminate. Minimum value is 10 minutes; maximum value is 14 day.\nDefaults to 1 hour if not set. If both ttl and idleTtl are specified for an interactive session, the conditions\nare treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or when ttl has\nbeen exceeded, whichever occurs first."]
    pub fn idle_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key to use for encryption."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tags` after provisioning.\nTags used for network traffic control."]
    pub fn network_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account that used to execute workload."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `staging_bucket` after provisioning.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn staging_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork_uri` after provisioning.\nSubnetwork configuration for workload execution."]
    pub fn subnetwork_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a session workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(
        &self,
    ) -> terrars::ListRef<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    #[doc =
        "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn set_dataproc_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataproc_cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    type O =
        terrars
        ::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {}

impl BuildDataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    pub fn build(self) -> DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
        DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
            dataproc_cluster: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
        DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn dataproc_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigDynamic {
    spark_history_server_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_history_server_config: std
    ::option
    ::Option<std::vec::Vec<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>>,
    dynamic: DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigDynamic,
}

impl DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {
    #[doc = "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service."]
    pub fn set_metastore_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metastore_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_history_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {}

impl BuildDataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {
    pub fn build(self) -> DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {
        DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig {
            metastore_service: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigRef {
        DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service."]
    pub fn metastore_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metastore_service", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> terrars::ListRef<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_history_server_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocSessionTemplate_EnvironmentConfigDynamic {
    execution_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig>>,
    peripherals_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig>>,
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_EnvironmentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_config: std::option::Option<std::vec::Vec<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peripherals_config: std::option::Option<std::vec::Vec<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig>>,
    dynamic: DataprocSessionTemplate_EnvironmentConfigDynamic,
}

impl DataprocSessionTemplate_EnvironmentConfig {
    #[doc = "Set the field `execution_config`.\n"]
    pub fn set_execution_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `peripherals_config`.\n"]
    pub fn set_peripherals_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.peripherals_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.peripherals_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_EnvironmentConfig {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_EnvironmentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_EnvironmentConfig {}

impl BuildDataprocSessionTemplate_EnvironmentConfig {
    pub fn build(self) -> DataprocSessionTemplate_EnvironmentConfig {
        DataprocSessionTemplate_EnvironmentConfig {
            execution_config: core::default::Default::default(),
            peripherals_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_EnvironmentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_EnvironmentConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocSessionTemplate_EnvironmentConfigRef {
        DataprocSessionTemplate_EnvironmentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_EnvironmentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_config` after provisioning.\n"]
    pub fn execution_config(&self) -> terrars::ListRef<DataprocSessionTemplate_EnvironmentConfig_ExecutionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_config", self.base))
    }

    #[doc = "Get a reference to the value of field `peripherals_config` after provisioning.\n"]
    pub fn peripherals_config(&self) -> terrars::ListRef<DataprocSessionTemplate_EnvironmentConfig_PeripheralsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peripherals_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_JupyterSession {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocSessionTemplate_JupyterSession {
    #[doc = "Set the field `display_name`.\nDisplay name, shown in the Jupyter kernelspec card."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kernel`.\nKernel to be used with Jupyter interactive session. Possible values: [\"PYTHON\", \"SCALA\"]"]
    pub fn set_kernel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kernel = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_JupyterSession {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_JupyterSession>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_JupyterSession {}

impl BuildDataprocSessionTemplate_JupyterSession {
    pub fn build(self) -> DataprocSessionTemplate_JupyterSession {
        DataprocSessionTemplate_JupyterSession {
            display_name: core::default::Default::default(),
            kernel: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_JupyterSessionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_JupyterSessionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocSessionTemplate_JupyterSessionRef {
        DataprocSessionTemplate_JupyterSessionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_JupyterSessionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay name, shown in the Jupyter kernelspec card."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kernel` after provisioning.\nKernel to be used with Jupyter interactive session. Possible values: [\"PYTHON\", \"SCALA\"]"]
    pub fn kernel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kernel", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_RuntimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocSessionTemplate_RuntimeConfig {
    #[doc =
        "Set the field `container_image`.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn set_container_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the session runtime."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocSessionTemplate_RuntimeConfig {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_RuntimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_RuntimeConfig {}

impl BuildDataprocSessionTemplate_RuntimeConfig {
    pub fn build(self) -> DataprocSessionTemplate_RuntimeConfig {
        DataprocSessionTemplate_RuntimeConfig {
            container_image: core::default::Default::default(),
            properties: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_RuntimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_RuntimeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocSessionTemplate_RuntimeConfigRef {
        DataprocSessionTemplate_RuntimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_RuntimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_image` after provisioning.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn container_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn effective_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the session runtime."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_SparkConnectSession {}

impl DataprocSessionTemplate_SparkConnectSession { }

impl terrars::ToListMappable for DataprocSessionTemplate_SparkConnectSession {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_SparkConnectSession>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_SparkConnectSession {}

impl BuildDataprocSessionTemplate_SparkConnectSession {
    pub fn build(self) -> DataprocSessionTemplate_SparkConnectSession {
        DataprocSessionTemplate_SparkConnectSession {}
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_SparkConnectSessionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_SparkConnectSessionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocSessionTemplate_SparkConnectSessionRef {
        DataprocSessionTemplate_SparkConnectSessionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_SparkConnectSessionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataprocSessionTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocSessionTemplate_Timeouts {
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

impl terrars::ToListMappable for DataprocSessionTemplate_Timeouts {
    type O = terrars::BlockAssignable<DataprocSessionTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocSessionTemplate_Timeouts {}

impl BuildDataprocSessionTemplate_Timeouts {
    pub fn build(self) -> DataprocSessionTemplate_Timeouts {
        DataprocSessionTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocSessionTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocSessionTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocSessionTemplate_TimeoutsRef {
        DataprocSessionTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocSessionTemplate_TimeoutsRef {
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
