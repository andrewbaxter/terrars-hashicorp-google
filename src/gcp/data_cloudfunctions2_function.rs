#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudfunctions2FunctionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudfunctions2Function_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfunctions2FunctionData_>,
}

#[derive(Clone)]
pub struct DataCloudfunctions2Function(std::rc::Rc<DataCloudfunctions2Function_>);

impl DataCloudfunctions2Function {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nDescribes the Build step of the function that builds a container\nfrom the given source."]
    pub fn build_config(&self) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of a function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nThe environment the function is hosted on."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_trigger` after provisioning.\nAn Eventarc trigger managed by Google Cloud Functions that fires events in\nresponse to a condition in another service."]
    pub fn event_trigger(&self) -> terrars::ListRef<DataCloudfunctions2Function_EventTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of this cloud function."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_config` after provisioning.\nDescribes the Service being deployed."]
    pub fn service_config(&self) -> terrars::ListRef<DataCloudfunctions2Function_ServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDescribes the current state of the function."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a Cloud Function."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nOutput only. The deployed url for the function."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudfunctions2Function {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfunctions2Function { }

impl terrars::ToListMappable for DataCloudfunctions2Function {
    type O = terrars::ListRef<DataCloudfunctions2FunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfunctions2Function_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloudfunctions2_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfunctions2Function {
    pub tf_id: std::string::String,
    #[doc = "The location of this cloud function."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "A user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudfunctions2Function {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfunctions2Function {
        let out = DataCloudfunctions2Function(std::rc::Rc::new(DataCloudfunctions2Function_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfunctions2FunctionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfunctions2FunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2FunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfunctions2FunctionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `build_config` after provisioning.\nDescribes the Build step of the function that builds a container\nfrom the given source."]
    pub fn build_config(&self) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of a function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nThe environment the function is hosted on."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_trigger` after provisioning.\nAn Eventarc trigger managed by Google Cloud Functions that fires events in\nresponse to a condition in another service."]
    pub fn event_trigger(&self) -> terrars::ListRef<DataCloudfunctions2Function_EventTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of this cloud function."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_config` after provisioning.\nDescribes the Service being deployed."]
    pub fn service_config(&self) -> terrars::ListRef<DataCloudfunctions2Function_ServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDescribes the current state of the function."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a Cloud Function."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nOutput only. The deployed url for the function."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {}

impl DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy { }

impl terrars::ToListMappable for DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {}

impl BuildDataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {
    pub fn build(self) -> DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {
        DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {}
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
        DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
    #[doc = "Set the field `runtime_version`.\n"]
    pub fn set_runtime_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {}

impl BuildDataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
    pub fn build(self) -> DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
        DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
            runtime_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
        DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_BuildConfig_Source_RepoSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_BuildConfig_Source_RepoSource {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dir`.\n"]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invert_regex`.\n"]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_name`.\n"]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_name`.\n"]
    pub fn set_tag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_BuildConfig_Source_RepoSource {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_BuildConfig_Source_RepoSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_BuildConfig_Source_RepoSource {}

impl BuildDataCloudfunctions2Function_BuildConfig_Source_RepoSource {
    pub fn build(self) -> DataCloudfunctions2Function_BuildConfig_Source_RepoSource {
        DataCloudfunctions2Function_BuildConfig_Source_RepoSource {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
        DataCloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc = "Get a reference to the value of field `dir` after provisioning.\n"]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc = "Get a reference to the value of field `invert_regex` after provisioning.\n"]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_name` after provisioning.\n"]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\n"]
    pub fn tag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_BuildConfig_Source_StorageSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_BuildConfig_Source_StorageSource {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generation`.\n"]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_BuildConfig_Source_StorageSource {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_BuildConfig_Source_StorageSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_BuildConfig_Source_StorageSource {}

impl BuildDataCloudfunctions2Function_BuildConfig_Source_StorageSource {
    pub fn build(self) -> DataCloudfunctions2Function_BuildConfig_Source_StorageSource {
        DataCloudfunctions2Function_BuildConfig_Source_StorageSource {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
        DataCloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `generation` after provisioning.\n"]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_BuildConfig_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_source: std::option::Option<terrars::ListField<DataCloudfunctions2Function_BuildConfig_Source_RepoSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_source: std
    ::option
    ::Option<terrars::ListField<DataCloudfunctions2Function_BuildConfig_Source_StorageSource>>,
}

impl DataCloudfunctions2Function_BuildConfig_Source {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctions2Function_BuildConfig_Source_RepoSource>>,
    ) -> Self {
        self.repo_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctions2Function_BuildConfig_Source_StorageSource>>,
    ) -> Self {
        self.storage_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_BuildConfig_Source {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_BuildConfig_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_BuildConfig_Source {}

impl BuildDataCloudfunctions2Function_BuildConfig_Source {
    pub fn build(self) -> DataCloudfunctions2Function_BuildConfig_Source {
        DataCloudfunctions2Function_BuildConfig_Source {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_BuildConfig_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_BuildConfig_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_BuildConfig_SourceRef {
        DataCloudfunctions2Function_BuildConfig_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_BuildConfig_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfig_Source_RepoSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(&self) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfig_Source_StorageSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_BuildConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_update_policy: std
    ::option
    ::Option<terrars::ListField<DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_deploy_update_policy: std
    ::option
    ::Option<terrars::ListField<DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::ListField<DataCloudfunctions2Function_BuildConfig_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_BuildConfig {
    #[doc = "Set the field `automatic_update_policy`.\n"]
    pub fn set_automatic_update_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>>,
    ) -> Self {
        self.automatic_update_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build`.\n"]
    pub fn set_build(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_repository`.\n"]
    pub fn set_docker_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.docker_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_point`.\n"]
    pub fn set_entry_point(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entry_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_deploy_update_policy`.\n"]
    pub fn set_on_deploy_update_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>>,
    ) -> Self {
        self.on_deploy_update_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctions2Function_BuildConfig_Source>>,
    ) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker_pool`.\n"]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_BuildConfig {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_BuildConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_BuildConfig {}

impl BuildDataCloudfunctions2Function_BuildConfig {
    pub fn build(self) -> DataCloudfunctions2Function_BuildConfig {
        DataCloudfunctions2Function_BuildConfig {
            automatic_update_policy: core::default::Default::default(),
            build: core::default::Default::default(),
            docker_repository: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            on_deploy_update_policy: core::default::Default::default(),
            runtime: core::default::Default::default(),
            service_account: core::default::Default::default(),
            source: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_BuildConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_BuildConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfunctions2Function_BuildConfigRef {
        DataCloudfunctions2Function_BuildConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_BuildConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(
        &self,
    ) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_update_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_repository` after provisioning.\n"]
    pub fn docker_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docker_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_point` after provisioning.\n"]
    pub fn entry_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(
        &self,
    ) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_deploy_update_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataCloudfunctions2Function_BuildConfig_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_pool` after provisioning.\n"]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_EventTrigger_EventFilters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_EventTrigger_EventFilters {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_EventTrigger_EventFilters {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_EventTrigger_EventFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_EventTrigger_EventFilters {}

impl BuildDataCloudfunctions2Function_EventTrigger_EventFilters {
    pub fn build(self) -> DataCloudfunctions2Function_EventTrigger_EventFilters {
        DataCloudfunctions2Function_EventTrigger_EventFilters {
            attribute: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_EventTrigger_EventFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_EventTrigger_EventFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_EventTrigger_EventFiltersRef {
        DataCloudfunctions2Function_EventTrigger_EventFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_EventTrigger_EventFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_EventTrigger {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_filters: std::option::Option<terrars::SetField<DataCloudfunctions2Function_EventTrigger_EventFilters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_EventTrigger {
    #[doc = "Set the field `event_filters`.\n"]
    pub fn set_event_filters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCloudfunctions2Function_EventTrigger_EventFilters>>,
    ) -> Self {
        self.event_filters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_type`.\n"]
    pub fn set_event_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pubsub_topic`.\n"]
    pub fn set_pubsub_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pubsub_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retry_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger`.\n"]
    pub fn set_trigger(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trigger = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_region`.\n"]
    pub fn set_trigger_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trigger_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_EventTrigger {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_EventTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_EventTrigger {}

impl BuildDataCloudfunctions2Function_EventTrigger {
    pub fn build(self) -> DataCloudfunctions2Function_EventTrigger {
        DataCloudfunctions2Function_EventTrigger {
            event_filters: core::default::Default::default(),
            event_type: core::default::Default::default(),
            pubsub_topic: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            trigger: core::default::Default::default(),
            trigger_region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_EventTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_EventTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfunctions2Function_EventTriggerRef {
        DataCloudfunctions2Function_EventTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_EventTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_filters` after provisioning.\n"]
    pub fn event_filters(&self) -> terrars::SetRef<DataCloudfunctions2Function_EventTrigger_EventFiltersRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.event_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\n"]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\n"]
    pub fn trigger(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_region` after provisioning.\n"]
    pub fn trigger_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {}

impl BuildDataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    pub fn build(self) -> DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
        DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
        DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {}

impl BuildDataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    pub fn build(self) -> DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
        DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
            key: core::default::Default::default(),
            project_id: core::default::Default::default(),
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
        DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {}

impl BuildDataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    pub fn build(self) -> DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
        DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
        DataCloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_ServiceConfig_SecretVolumes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>>,
}

impl DataCloudfunctions2Function_ServiceConfig_SecretVolumes {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>>,
    ) -> Self {
        self.versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_ServiceConfig_SecretVolumes {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_ServiceConfig_SecretVolumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_ServiceConfig_SecretVolumes {}

impl BuildDataCloudfunctions2Function_ServiceConfig_SecretVolumes {
    pub fn build(self) -> DataCloudfunctions2Function_ServiceConfig_SecretVolumes {
        DataCloudfunctions2Function_ServiceConfig_SecretVolumes {
            mount_path: core::default::Default::default(),
            project_id: core::default::Default::default(),
            secret: core::default::Default::default(),
            versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_ServiceConfig_SecretVolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_ServiceConfig_SecretVolumesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctions2Function_ServiceConfig_SecretVolumesRef {
        DataCloudfunctions2Function_ServiceConfig_SecretVolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_ServiceConfig_SecretVolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<DataCloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctions2Function_ServiceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_traffic_on_latest_revision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_vpc_egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_vpc_network_interface: std
    ::option
    ::Option<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcf_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_request_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_environment_variables: std
    ::option
    ::Option<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_volumes: std::option::Option<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_SecretVolumes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector_egress_settings: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctions2Function_ServiceConfig {
    #[doc = "Set the field `all_traffic_on_latest_revision`.\n"]
    pub fn set_all_traffic_on_latest_revision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_traffic_on_latest_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_cpu`.\n"]
    pub fn set_available_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.available_cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_memory`.\n"]
    pub fn set_available_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.available_memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `binary_authorization_policy`.\n"]
    pub fn set_binary_authorization_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.binary_authorization_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direct_vpc_egress`.\n"]
    pub fn set_direct_vpc_egress(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direct_vpc_egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direct_vpc_network_interface`.\n"]
    pub fn set_direct_vpc_network_interface(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>>,
    ) -> Self {
        self.direct_vpc_network_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\n"]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcf_uri`.\n"]
    pub fn set_gcf_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcf_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_settings`.\n"]
    pub fn set_ingress_settings(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ingress_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_instance_request_concurrency`.\n"]
    pub fn set_max_instance_request_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_environment_variables`.\n"]
    pub fn set_secret_environment_variables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>>,
    ) -> Self {
        self.secret_environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_volumes`.\n"]
    pub fn set_secret_volumes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctions2Function_ServiceConfig_SecretVolumes>>,
    ) -> Self {
        self.secret_volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_connector`.\n"]
    pub fn set_vpc_connector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_connector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_connector_egress_settings`.\n"]
    pub fn set_vpc_connector_egress_settings(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vpc_connector_egress_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctions2Function_ServiceConfig {
    type O = terrars::BlockAssignable<DataCloudfunctions2Function_ServiceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctions2Function_ServiceConfig {}

impl BuildDataCloudfunctions2Function_ServiceConfig {
    pub fn build(self) -> DataCloudfunctions2Function_ServiceConfig {
        DataCloudfunctions2Function_ServiceConfig {
            all_traffic_on_latest_revision: core::default::Default::default(),
            available_cpu: core::default::Default::default(),
            available_memory: core::default::Default::default(),
            binary_authorization_policy: core::default::Default::default(),
            direct_vpc_egress: core::default::Default::default(),
            direct_vpc_network_interface: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            gcf_uri: core::default::Default::default(),
            ingress_settings: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            secret_environment_variables: core::default::Default::default(),
            secret_volumes: core::default::Default::default(),
            service: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            uri: core::default::Default::default(),
            vpc_connector: core::default::Default::default(),
            vpc_connector_egress_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudfunctions2Function_ServiceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctions2Function_ServiceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfunctions2Function_ServiceConfigRef {
        DataCloudfunctions2Function_ServiceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctions2Function_ServiceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_traffic_on_latest_revision` after provisioning.\n"]
    pub fn all_traffic_on_latest_revision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_traffic_on_latest_revision", self.base))
    }

    #[doc = "Get a reference to the value of field `available_cpu` after provisioning.\n"]
    pub fn available_cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `available_memory` after provisioning.\n"]
    pub fn available_memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_memory", self.base))
    }

    #[doc = "Get a reference to the value of field `binary_authorization_policy` after provisioning.\n"]
    pub fn binary_authorization_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binary_authorization_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_vpc_egress` after provisioning.\n"]
    pub fn direct_vpc_egress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direct_vpc_egress", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_vpc_network_interface` after provisioning.\n"]
    pub fn direct_vpc_network_interface(
        &self,
    ) -> terrars::ListRef<DataCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.direct_vpc_network_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_variables` after provisioning.\n"]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `gcf_uri` after provisioning.\n"]
    pub fn gcf_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcf_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_settings` after provisioning.\n"]
    pub fn ingress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\n"]
    pub fn max_instance_request_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_request_concurrency", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(
        &self,
    ) -> terrars::ListRef<DataCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(&self) -> terrars::ListRef<DataCloudfunctions2Function_ServiceConfig_SecretVolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_connector` after provisioning.\n"]
    pub fn vpc_connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\n"]
    pub fn vpc_connector_egress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector_egress_settings", self.base))
    }
}
