#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataformRepositoryData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    npmrc_environment_variables_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_remote_settings: std::option::Option<std::vec::Vec<DataformRepository_GitRemoteSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataformRepository_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_compilation_overrides: std::option::Option<std::vec::Vec<DataformRepository_WorkspaceCompilationOverrides>>,
    dynamic: DataformRepositoryDynamic,
}

struct DataformRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataformRepositoryData_>,
}

#[derive(Clone)]
pub struct DataformRepository(std::rc::Rc<DataformRepository_>);

impl DataformRepository {
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
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/dataform_repository.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nOptional. The repository's user-friendly name."]
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
        "Set the field `kms_key_name`.\nOptional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources.\nIt is not possible to add or update the encryption key after the repository is created. Example projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]"]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Repository user labels.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `npmrc_environment_variables_secret_version`.\nOptional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format projects/*/secrets/*/versions/*. The file itself must be in a JSON format."]
    pub fn set_npmrc_environment_variables_secret_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().npmrc_environment_variables_secret_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `service_account`.\nThe service account to run workflow invocations under."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_remote_settings`.\n"]
    pub fn set_git_remote_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataformRepository_GitRemoteSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().git_remote_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.git_remote_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataformRepository_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workspace_compilation_overrides`.\n"]
    pub fn set_workspace_compilation_overrides(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataformRepository_WorkspaceCompilationOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workspace_compilation_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workspace_compilation_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/dataform_repository.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. The repository's user-friendly name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources.\nIt is not possible to add or update the encryption key after the repository is created. Example projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Repository user labels.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe repository's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `npmrc_environment_variables_secret_version` after provisioning.\nOptional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format projects/*/secrets/*/versions/*. The file itself must be in a JSON format."]
    pub fn npmrc_environment_variables_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.npmrc_environment_variables_secret_version", self.extract_ref()),
        )
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
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run workflow invocations under."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `git_remote_settings` after provisioning.\n"]
    pub fn git_remote_settings(&self) -> terrars::ListRef<DataformRepository_GitRemoteSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_remote_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataformRepository_TimeoutsRef {
        DataformRepository_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_compilation_overrides` after provisioning.\n"]
    pub fn workspace_compilation_overrides(
        &self,
    ) -> terrars::ListRef<DataformRepository_WorkspaceCompilationOverridesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_compilation_overrides", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataformRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataformRepository { }

impl terrars::ToListMappable for DataformRepository {
    type O = terrars::ListRef<DataformRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataformRepository_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataform_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataformRepository {
    pub tf_id: std::string::String,
    #[doc = "The repository's name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataformRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> DataformRepository {
        let out = DataformRepository(std::rc::Rc::new(DataformRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataformRepositoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                npmrc_environment_variables_secret_version: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                service_account: core::default::Default::default(),
                git_remote_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workspace_compilation_overrides: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataformRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataformRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataformRepositoryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/dataform_repository.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. The repository's user-friendly name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. The reference to a KMS encryption key. If provided, it will be used to encrypt user data in the repository and all child resources.\nIt is not possible to add or update the encryption key after the repository is created. Example projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Repository user labels.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe repository's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `npmrc_environment_variables_secret_version` after provisioning.\nOptional. The name of the Secret Manager secret version to be used to interpolate variables into the .npmrc file for package installation operations. Must be in the format projects/*/secrets/*/versions/*. The file itself must be in a JSON format."]
    pub fn npmrc_environment_variables_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.npmrc_environment_variables_secret_version", self.extract_ref()),
        )
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
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to run workflow invocations under."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `git_remote_settings` after provisioning.\n"]
    pub fn git_remote_settings(&self) -> terrars::ListRef<DataformRepository_GitRemoteSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git_remote_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataformRepository_TimeoutsRef {
        DataformRepository_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workspace_compilation_overrides` after provisioning.\n"]
    pub fn workspace_compilation_overrides(
        &self,
    ) -> terrars::ListRef<DataformRepository_WorkspaceCompilationOverridesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.workspace_compilation_overrides", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DataformRepositoryDynamic {
    git_remote_settings: std::option::Option<terrars::DynamicBlock<DataformRepository_GitRemoteSettings>>,
    workspace_compilation_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<DataformRepository_WorkspaceCompilationOverrides>>,
}

#[derive(Serialize)]
pub struct DataformRepository_GitRemoteSettings_SshAuthenticationConfig {
    host_public_key: terrars::PrimField<std::string::String>,
    user_private_key_secret_version: terrars::PrimField<std::string::String>,
}

impl DataformRepository_GitRemoteSettings_SshAuthenticationConfig { }

impl terrars::ToListMappable for DataformRepository_GitRemoteSettings_SshAuthenticationConfig {
    type O = terrars::BlockAssignable<DataformRepository_GitRemoteSettings_SshAuthenticationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataformRepository_GitRemoteSettings_SshAuthenticationConfig {
    #[doc = "Content of a public SSH key to verify an identity of a remote Git host."]
    pub host_public_key: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the Secret Manager secret version to use as a ssh private key for Git operations. Must be in the format projects/*/secrets/*/versions/*."]
    pub user_private_key_secret_version: terrars::PrimField<std::string::String>,
}

impl BuildDataformRepository_GitRemoteSettings_SshAuthenticationConfig {
    pub fn build(self) -> DataformRepository_GitRemoteSettings_SshAuthenticationConfig {
        DataformRepository_GitRemoteSettings_SshAuthenticationConfig {
            host_public_key: self.host_public_key,
            user_private_key_secret_version: self.user_private_key_secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct DataformRepository_GitRemoteSettings_SshAuthenticationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataformRepository_GitRemoteSettings_SshAuthenticationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataformRepository_GitRemoteSettings_SshAuthenticationConfigRef {
        DataformRepository_GitRemoteSettings_SshAuthenticationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataformRepository_GitRemoteSettings_SshAuthenticationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_public_key` after provisioning.\nContent of a public SSH key to verify an identity of a remote Git host."]
    pub fn host_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_public_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_private_key_secret_version` after provisioning.\nThe name of the Secret Manager secret version to use as a ssh private key for Git operations. Must be in the format projects/*/secrets/*/versions/*."]
    pub fn user_private_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_private_key_secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataformRepository_GitRemoteSettingsDynamic {
    ssh_authentication_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataformRepository_GitRemoteSettings_SshAuthenticationConfig>>,
}

#[derive(Serialize)]
pub struct DataformRepository_GitRemoteSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_token_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    default_branch: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_repository_link: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_authentication_config: std
    ::option
    ::Option<std::vec::Vec<DataformRepository_GitRemoteSettings_SshAuthenticationConfig>>,
    dynamic: DataformRepository_GitRemoteSettingsDynamic,
}

impl DataformRepository_GitRemoteSettings {
    #[doc =
        "Set the field `authentication_token_secret_version`.\nThe name of the Secret Manager secret version to use as an authentication token for Git operations. This secret is for assigning with HTTPS only(for SSH use 'ssh_authentication_config'). Must be in the format projects/*/secrets/*/versions/*."]
    pub fn set_authentication_token_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_token_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `git_repository_link`.\nThe name of the Developer Connect GitRepositoryLink to use for machine credentials. Must be in the format projects/*/locations/*/connections/*/gitRepositoryLinks/*."]
    pub fn set_git_repository_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.git_repository_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_authentication_config`.\n"]
    pub fn set_ssh_authentication_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataformRepository_GitRemoteSettings_SshAuthenticationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssh_authentication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_authentication_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataformRepository_GitRemoteSettings {
    type O = terrars::BlockAssignable<DataformRepository_GitRemoteSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataformRepository_GitRemoteSettings {
    #[doc = "The Git remote's default branch name."]
    pub default_branch: terrars::PrimField<std::string::String>,
    #[doc = "The Git remote's URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildDataformRepository_GitRemoteSettings {
    pub fn build(self) -> DataformRepository_GitRemoteSettings {
        DataformRepository_GitRemoteSettings {
            authentication_token_secret_version: core::default::Default::default(),
            default_branch: self.default_branch,
            git_repository_link: core::default::Default::default(),
            url: self.url,
            ssh_authentication_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataformRepository_GitRemoteSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataformRepository_GitRemoteSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataformRepository_GitRemoteSettingsRef {
        DataformRepository_GitRemoteSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataformRepository_GitRemoteSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_token_secret_version` after provisioning.\nThe name of the Secret Manager secret version to use as an authentication token for Git operations. This secret is for assigning with HTTPS only(for SSH use 'ssh_authentication_config'). Must be in the format projects/*/secrets/*/versions/*."]
    pub fn authentication_token_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_token_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_branch` after provisioning.\nThe Git remote's default branch name."]
    pub fn default_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `git_repository_link` after provisioning.\nThe name of the Developer Connect GitRepositoryLink to use for machine credentials. Must be in the format projects/*/locations/*/connections/*/gitRepositoryLinks/*."]
    pub fn git_repository_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_repository_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_status` after provisioning.\nIndicates the status of the Git access token. https://cloud.google.com/dataform/reference/rest/v1/projects.locations.repositories#TokenStatus"]
    pub fn token_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_status", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Git remote's URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_authentication_config` after provisioning.\n"]
    pub fn ssh_authentication_config(
        &self,
    ) -> terrars::ListRef<DataformRepository_GitRemoteSettings_SshAuthenticationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_authentication_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataformRepository_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataformRepository_Timeouts {
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

impl terrars::ToListMappable for DataformRepository_Timeouts {
    type O = terrars::BlockAssignable<DataformRepository_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataformRepository_Timeouts {}

impl BuildDataformRepository_Timeouts {
    pub fn build(self) -> DataformRepository_Timeouts {
        DataformRepository_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataformRepository_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataformRepository_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataformRepository_TimeoutsRef {
        DataformRepository_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataformRepository_TimeoutsRef {
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
pub struct DataformRepository_WorkspaceCompilationOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataformRepository_WorkspaceCompilationOverrides {
    #[doc = "Set the field `default_database`.\nThe default database (Google Cloud project ID)."]
    pub fn set_default_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_database = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema_suffix`.\nThe suffix that should be appended to all schema (BigQuery dataset ID) names."]
    pub fn set_schema_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_prefix`.\nThe prefix that should be prepended to all table names."]
    pub fn set_table_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataformRepository_WorkspaceCompilationOverrides {
    type O = terrars::BlockAssignable<DataformRepository_WorkspaceCompilationOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataformRepository_WorkspaceCompilationOverrides {}

impl BuildDataformRepository_WorkspaceCompilationOverrides {
    pub fn build(self) -> DataformRepository_WorkspaceCompilationOverrides {
        DataformRepository_WorkspaceCompilationOverrides {
            default_database: core::default::Default::default(),
            schema_suffix: core::default::Default::default(),
            table_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataformRepository_WorkspaceCompilationOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataformRepository_WorkspaceCompilationOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataformRepository_WorkspaceCompilationOverridesRef {
        DataformRepository_WorkspaceCompilationOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataformRepository_WorkspaceCompilationOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_database` after provisioning.\nThe default database (Google Cloud project ID)."]
    pub fn default_database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_suffix` after provisioning.\nThe suffix that should be appended to all schema (BigQuery dataset ID) names."]
    pub fn schema_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_prefix` after provisioning.\nThe prefix that should be prepended to all table names."]
    pub fn table_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.base))
    }
}
