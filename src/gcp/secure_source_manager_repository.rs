#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SecureSourceManagerRepositoryData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    repository_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_config: std::option::Option<std::vec::Vec<SecureSourceManagerRepository_InitialConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_config: std::option::Option<std::vec::Vec<SecureSourceManagerRepository_ScanConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SecureSourceManagerRepository_Timeouts>,
    dynamic: SecureSourceManagerRepositoryDynamic,
}

struct SecureSourceManagerRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecureSourceManagerRepositoryData_>,
}

#[derive(Clone)]
pub struct SecureSourceManagerRepository(std::rc::Rc<SecureSourceManagerRepository_>);

impl SecureSourceManagerRepository {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the repository, which cannot exceed 500 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `service_account`.\nRepository level service account."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_config`.\n"]
    pub fn set_initial_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecureSourceManagerRepository_InitialConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scan_config`.\n"]
    pub fn set_scan_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecureSourceManagerRepository_ScanConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scan_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scan_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SecureSourceManagerRepository_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the repository was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the repository, which cannot exceed 500 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance in which the repository is hosted."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Repository."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nRepository level service account."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the repository."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the repository was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\nURIs for the repository."]
    pub fn uris(&self) -> terrars::ListRef<SecureSourceManagerRepository_UrisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uris", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_config` after provisioning.\n"]
    pub fn initial_config(&self) -> terrars::ListRef<SecureSourceManagerRepository_InitialConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_config` after provisioning.\n"]
    pub fn scan_config(&self) -> terrars::ListRef<SecureSourceManagerRepository_ScanConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerRepository_TimeoutsRef {
        SecureSourceManagerRepository_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SecureSourceManagerRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecureSourceManagerRepository { }

impl terrars::ToListMappable for SecureSourceManagerRepository {
    type O = terrars::ListRef<SecureSourceManagerRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecureSourceManagerRepository_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_secure_source_manager_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecureSourceManagerRepository {
    pub tf_id: std::string::String,
    #[doc = "The name of the instance in which the repository is hosted."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc = "The location for the Repository."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The ID for the Repository."]
    pub repository_id: terrars::PrimField<std::string::String>,
}

impl BuildSecureSourceManagerRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> SecureSourceManagerRepository {
        let out = SecureSourceManagerRepository(std::rc::Rc::new(SecureSourceManagerRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecureSourceManagerRepositoryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                service_account: core::default::Default::default(),
                initial_config: core::default::Default::default(),
                scan_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecureSourceManagerRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecureSourceManagerRepositoryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the repository was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the repository, which cannot exceed 500 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance in which the repository is hosted."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Repository."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nRepository level service account."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the repository."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the repository was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\nURIs for the repository."]
    pub fn uris(&self) -> terrars::ListRef<SecureSourceManagerRepository_UrisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uris", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `initial_config` after provisioning.\n"]
    pub fn initial_config(&self) -> terrars::ListRef<SecureSourceManagerRepository_InitialConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_config` after provisioning.\n"]
    pub fn scan_config(&self) -> terrars::ListRef<SecureSourceManagerRepository_ScanConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerRepository_TimeoutsRef {
        SecureSourceManagerRepository_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SecureSourceManagerRepositoryDynamic {
    initial_config: std::option::Option<terrars::DynamicBlock<SecureSourceManagerRepository_InitialConfig>>,
    scan_config: std::option::Option<terrars::DynamicBlock<SecureSourceManagerRepository_ScanConfig>>,
}

#[derive(Serialize)]
pub struct SecureSourceManagerRepository_Uris {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_https: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecureSourceManagerRepository_Uris {
    #[doc = "Set the field `api`.\n"]
    pub fn set_api(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_https`.\n"]
    pub fn set_git_https(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.git_https = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.html = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecureSourceManagerRepository_Uris {
    type O = terrars::BlockAssignable<SecureSourceManagerRepository_Uris>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerRepository_Uris {}

impl BuildSecureSourceManagerRepository_Uris {
    pub fn build(self) -> SecureSourceManagerRepository_Uris {
        SecureSourceManagerRepository_Uris {
            api: core::default::Default::default(),
            git_https: core::default::Default::default(),
            html: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerRepository_UrisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerRepository_UrisRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecureSourceManagerRepository_UrisRef {
        SecureSourceManagerRepository_UrisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerRepository_UrisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }

    #[doc = "Get a reference to the value of field `git_https` after provisioning.\n"]
    pub fn git_https(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_https", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }
}

#[derive(Serialize)]
pub struct SecureSourceManagerRepository_InitialConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitignores: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readme: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecureSourceManagerRepository_InitialConfig {
    #[doc = "Set the field `default_branch`.\nDefault branch name of the repository."]
    pub fn set_default_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gitignores`.\nList of gitignore template names user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn set_gitignores(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gitignores = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `license`.\nLicense template name user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn set_license(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `readme`.\nREADME template name.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn set_readme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.readme = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecureSourceManagerRepository_InitialConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerRepository_InitialConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerRepository_InitialConfig {}

impl BuildSecureSourceManagerRepository_InitialConfig {
    pub fn build(self) -> SecureSourceManagerRepository_InitialConfig {
        SecureSourceManagerRepository_InitialConfig {
            default_branch: core::default::Default::default(),
            gitignores: core::default::Default::default(),
            license: core::default::Default::default(),
            readme: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerRepository_InitialConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerRepository_InitialConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecureSourceManagerRepository_InitialConfigRef {
        SecureSourceManagerRepository_InitialConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerRepository_InitialConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_branch` after provisioning.\nDefault branch name of the repository."]
    pub fn default_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gitignores` after provisioning.\nList of gitignore template names user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn gitignores(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitignores", self.base))
    }

    #[doc =
        "Get a reference to the value of field `license` after provisioning.\nLicense template name user can choose from.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn license(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license", self.base))
    }

    #[doc =
        "Get a reference to the value of field `readme` after provisioning.\nREADME template name.\nValid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig."]
    pub fn readme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.readme", self.base))
    }
}

#[derive(Serialize)]
pub struct SecureSourceManagerRepository_ScanConfig_SecretScanConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecureSourceManagerRepository_ScanConfig_SecretScanConfig {
    #[doc = "Set the field `enabled`.\nEnables secret scanning for the repository."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_template`.\nThe DLP inspect template to use for secret scanning."]
    pub fn set_inspect_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inspect_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecureSourceManagerRepository_ScanConfig_SecretScanConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerRepository_ScanConfig_SecretScanConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerRepository_ScanConfig_SecretScanConfig {}

impl BuildSecureSourceManagerRepository_ScanConfig_SecretScanConfig {
    pub fn build(self) -> SecureSourceManagerRepository_ScanConfig_SecretScanConfig {
        SecureSourceManagerRepository_ScanConfig_SecretScanConfig {
            enabled: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerRepository_ScanConfig_SecretScanConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerRepository_ScanConfig_SecretScanConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecureSourceManagerRepository_ScanConfig_SecretScanConfigRef {
        SecureSourceManagerRepository_ScanConfig_SecretScanConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerRepository_ScanConfig_SecretScanConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables secret scanning for the repository."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_template` after provisioning.\nThe DLP inspect template to use for secret scanning."]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecureSourceManagerRepository_ScanConfigDynamic {
    secret_scan_config: std
    ::option
    ::Option<terrars::DynamicBlock<SecureSourceManagerRepository_ScanConfig_SecretScanConfig>>,
}

#[derive(Serialize)]
pub struct SecureSourceManagerRepository_ScanConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_scan_config: std::option::Option<std::vec::Vec<SecureSourceManagerRepository_ScanConfig_SecretScanConfig>>,
    dynamic: SecureSourceManagerRepository_ScanConfigDynamic,
}

impl SecureSourceManagerRepository_ScanConfig {
    #[doc = "Set the field `secret_scan_config`.\n"]
    pub fn set_secret_scan_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecureSourceManagerRepository_ScanConfig_SecretScanConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_scan_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_scan_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecureSourceManagerRepository_ScanConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerRepository_ScanConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerRepository_ScanConfig {}

impl BuildSecureSourceManagerRepository_ScanConfig {
    pub fn build(self) -> SecureSourceManagerRepository_ScanConfig {
        SecureSourceManagerRepository_ScanConfig {
            secret_scan_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerRepository_ScanConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerRepository_ScanConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecureSourceManagerRepository_ScanConfigRef {
        SecureSourceManagerRepository_ScanConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerRepository_ScanConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_scan_config` after provisioning.\n"]
    pub fn secret_scan_config(&self) -> terrars::ListRef<SecureSourceManagerRepository_ScanConfig_SecretScanConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_scan_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SecureSourceManagerRepository_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecureSourceManagerRepository_Timeouts {
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

impl terrars::ToListMappable for SecureSourceManagerRepository_Timeouts {
    type O = terrars::BlockAssignable<SecureSourceManagerRepository_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerRepository_Timeouts {}

impl BuildSecureSourceManagerRepository_Timeouts {
    pub fn build(self) -> SecureSourceManagerRepository_Timeouts {
        SecureSourceManagerRepository_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerRepository_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerRepository_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecureSourceManagerRepository_TimeoutsRef {
        SecureSourceManagerRepository_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerRepository_TimeoutsRef {
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
