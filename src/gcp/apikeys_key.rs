#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApikeysKeyData_ {
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
    check_existing_usage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<ApikeysKey_Restrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApikeysKey_Timeouts>,
    dynamic: ApikeysKeyDynamic,
}

struct ApikeysKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApikeysKeyData_>,
}

#[derive(Clone)]
pub struct ApikeysKey(std::rc::Rc<ApikeysKey_>);

impl ApikeysKey {
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
        "Set the field `check_existing_usage`.\nDefines the behavior for checking existing usage when updating a key. Possible values: 'SKIP', 'CHECK'."]
    pub fn set_check_existing_usage(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().check_existing_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the resource. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the resource,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed, and existing traffic usage will be checked. If active usage was detected in the last 7 days, the request fails.\nWhen set to \"FORCE\", deleting the resource will bypass the active traffic usage check."]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nHuman-readable display name of this API key. Modifiable by user."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe email of the service account the key is bound to. If this field is specified, the key is a service account bound key and auth enabled. See [Documentation](https://cloud.google.com/docs/authentication/api-keys?#api-keys-bound-sa) for more details."]
    pub fn set_service_account_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKey_Restrictions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApikeysKey_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `check_existing_usage` after provisioning.\nDefines the behavior for checking existing usage when updating a key. Possible values: 'SKIP', 'CHECK'."]
    pub fn check_existing_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_existing_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the resource. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the resource,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed, and existing traffic usage will be checked. If active usage was detected in the last 7 days, the request fails.\nWhen set to \"FORCE\", deleting the resource will bypass the active traffic usage check."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this API key. Modifiable by user."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_string` after provisioning.\nOutput only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method."]
    pub fn key_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe email of the service account the key is bound to. If this field is specified, the key is a service account bound key and auth enabled. See [Documentation](https://cloud.google.com/docs/authentication/api-keys?#api-keys-bound-sa) for more details."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique id in UUID4 format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<ApikeysKey_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApikeysKey_TimeoutsRef {
        ApikeysKey_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApikeysKey {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApikeysKey { }

impl terrars::ToListMappable for ApikeysKey {
    type O = terrars::ListRef<ApikeysKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApikeysKey_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apikeys_key".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApikeysKey {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApikeysKey {
    pub fn build(self, stack: &mut terrars::Stack) -> ApikeysKey {
        let out = ApikeysKey(std::rc::Rc::new(ApikeysKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApikeysKeyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                check_existing_usage: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                service_account_email: core::default::Default::default(),
                restrictions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApikeysKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApikeysKeyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_existing_usage` after provisioning.\nDefines the behavior for checking existing usage when updating a key. Possible values: 'SKIP', 'CHECK'."]
    pub fn check_existing_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_existing_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the resource. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the resource,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed, and existing traffic usage will be checked. If active usage was detected in the last 7 days, the request fails.\nWhen set to \"FORCE\", deleting the resource will bypass the active traffic usage check."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this API key. Modifiable by user."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_string` after provisioning.\nOutput only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method."]
    pub fn key_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the key. The name must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. In another word, the name must match the regular expression: `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe email of the service account the key is bound to. If this field is specified, the key is a service account bound key and auth enabled. See [Documentation](https://cloud.google.com/docs/authentication/api-keys?#api-keys-bound-sa) for more details."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique id in UUID4 format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<ApikeysKey_RestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApikeysKey_TimeoutsRef {
        ApikeysKey_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApikeysKeyDynamic {
    restrictions: std::option::Option<terrars::DynamicBlock<ApikeysKey_Restrictions>>,
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications {
    package_name: terrars::PrimField<std::string::String>,
    sha1_fingerprint: terrars::PrimField<std::string::String>,
}

impl ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications { }

impl terrars::ToListMappable for ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications {
    #[doc = "The package name of the application."]
    pub package_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter."]
    pub sha1_fingerprint: terrars::PrimField<std::string::String>,
}

impl BuildApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications {
    pub fn build(self) -> ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications {
        ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications {
            package_name: self.package_name,
            sha1_fingerprint: self.sha1_fingerprint,
        }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplicationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplicationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplicationsRef {
        ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplicationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplicationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `package_name` after provisioning.\nThe package name of the application."]
    pub fn package_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha1_fingerprint` after provisioning.\nThe SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter."]
    pub fn sha1_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha1_fingerprint", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApikeysKey_Restrictions_AndroidKeyRestrictionsDynamic {
    allowed_applications: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications>>,
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions_AndroidKeyRestrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_applications: std
    ::option
    ::Option<std::vec::Vec<ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications>>,
    dynamic: ApikeysKey_Restrictions_AndroidKeyRestrictionsDynamic,
}

impl ApikeysKey_Restrictions_AndroidKeyRestrictions {
    #[doc = "Set the field `allowed_applications`.\n"]
    pub fn set_allowed_applications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplications>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_applications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_applications = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApikeysKey_Restrictions_AndroidKeyRestrictions {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions_AndroidKeyRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions_AndroidKeyRestrictions {}

impl BuildApikeysKey_Restrictions_AndroidKeyRestrictions {
    pub fn build(self) -> ApikeysKey_Restrictions_AndroidKeyRestrictions {
        ApikeysKey_Restrictions_AndroidKeyRestrictions {
            allowed_applications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_Restrictions_AndroidKeyRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_Restrictions_AndroidKeyRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKey_Restrictions_AndroidKeyRestrictionsRef {
        ApikeysKey_Restrictions_AndroidKeyRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_Restrictions_AndroidKeyRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_applications` after provisioning.\n"]
    pub fn allowed_applications(
        &self,
    ) -> terrars::ListRef<ApikeysKey_Restrictions_AndroidKeyRestrictions_AllowedApplicationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_applications", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions_ApiTargets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    service: terrars::PrimField<std::string::String>,
}

impl ApikeysKey_Restrictions_ApiTargets {
    #[doc =
        "Set the field `methods`.\nOptional. List of one or more methods that can be called. If empty, all methods for the service are allowed. A wildcard (*) can be used as the last symbol. Valid examples: `google.cloud.translate.v2.TranslateService.GetSupportedLanguage` `TranslateText` `Get*` `translate.googleapis.com.Get*`"]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApikeysKey_Restrictions_ApiTargets {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions_ApiTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions_ApiTargets {
    #[doc =
        "The service for this restriction. It should be the canonical service name, for example: `translate.googleapis.com`. You can use `gcloud services list` to get a list of services that are enabled in the project."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildApikeysKey_Restrictions_ApiTargets {
    pub fn build(self) -> ApikeysKey_Restrictions_ApiTargets {
        ApikeysKey_Restrictions_ApiTargets {
            methods: core::default::Default::default(),
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_Restrictions_ApiTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_Restrictions_ApiTargetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKey_Restrictions_ApiTargetsRef {
        ApikeysKey_Restrictions_ApiTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_Restrictions_ApiTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nOptional. List of one or more methods that can be called. If empty, all methods for the service are allowed. A wildcard (*) can be used as the last symbol. Valid examples: `google.cloud.translate.v2.TranslateService.GetSupportedLanguage` `TranslateText` `Get*` `translate.googleapis.com.Get*`"]
    pub fn methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe service for this restriction. It should be the canonical service name, for example: `translate.googleapis.com`. You can use `gcloud services list` to get a list of services that are enabled in the project."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions_BrowserKeyRestrictions {
    allowed_referrers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ApikeysKey_Restrictions_BrowserKeyRestrictions { }

impl terrars::ToListMappable for ApikeysKey_Restrictions_BrowserKeyRestrictions {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions_BrowserKeyRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions_BrowserKeyRestrictions {
    #[doc = "A list of regular expressions for the referrer URLs that are allowed to make API calls with this key."]
    pub allowed_referrers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApikeysKey_Restrictions_BrowserKeyRestrictions {
    pub fn build(self) -> ApikeysKey_Restrictions_BrowserKeyRestrictions {
        ApikeysKey_Restrictions_BrowserKeyRestrictions { allowed_referrers: self.allowed_referrers }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_Restrictions_BrowserKeyRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_Restrictions_BrowserKeyRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKey_Restrictions_BrowserKeyRestrictionsRef {
        ApikeysKey_Restrictions_BrowserKeyRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_Restrictions_BrowserKeyRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_referrers` after provisioning.\nA list of regular expressions for the referrer URLs that are allowed to make API calls with this key."]
    pub fn allowed_referrers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_referrers", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions_IosKeyRestrictions {
    allowed_bundle_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ApikeysKey_Restrictions_IosKeyRestrictions { }

impl terrars::ToListMappable for ApikeysKey_Restrictions_IosKeyRestrictions {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions_IosKeyRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions_IosKeyRestrictions {
    #[doc = "A list of bundle IDs that are allowed when making API calls with this key."]
    pub allowed_bundle_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApikeysKey_Restrictions_IosKeyRestrictions {
    pub fn build(self) -> ApikeysKey_Restrictions_IosKeyRestrictions {
        ApikeysKey_Restrictions_IosKeyRestrictions { allowed_bundle_ids: self.allowed_bundle_ids }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_Restrictions_IosKeyRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_Restrictions_IosKeyRestrictionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKey_Restrictions_IosKeyRestrictionsRef {
        ApikeysKey_Restrictions_IosKeyRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_Restrictions_IosKeyRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_bundle_ids` after provisioning.\nA list of bundle IDs that are allowed when making API calls with this key."]
    pub fn allowed_bundle_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_bundle_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions_ServerKeyRestrictions {
    allowed_ips: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ApikeysKey_Restrictions_ServerKeyRestrictions { }

impl terrars::ToListMappable for ApikeysKey_Restrictions_ServerKeyRestrictions {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions_ServerKeyRestrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions_ServerKeyRestrictions {
    #[doc = "A list of the caller IP addresses that are allowed to make API calls with this key."]
    pub allowed_ips: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApikeysKey_Restrictions_ServerKeyRestrictions {
    pub fn build(self) -> ApikeysKey_Restrictions_ServerKeyRestrictions {
        ApikeysKey_Restrictions_ServerKeyRestrictions { allowed_ips: self.allowed_ips }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_Restrictions_ServerKeyRestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_Restrictions_ServerKeyRestrictionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKey_Restrictions_ServerKeyRestrictionsRef {
        ApikeysKey_Restrictions_ServerKeyRestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_Restrictions_ServerKeyRestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_ips` after provisioning.\nA list of the caller IP addresses that are allowed to make API calls with this key."]
    pub fn allowed_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApikeysKey_RestrictionsDynamic {
    android_key_restrictions: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKey_Restrictions_AndroidKeyRestrictions>>,
    api_targets: std::option::Option<terrars::DynamicBlock<ApikeysKey_Restrictions_ApiTargets>>,
    browser_key_restrictions: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKey_Restrictions_BrowserKeyRestrictions>>,
    ios_key_restrictions: std::option::Option<terrars::DynamicBlock<ApikeysKey_Restrictions_IosKeyRestrictions>>,
    server_key_restrictions: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKey_Restrictions_ServerKeyRestrictions>>,
}

#[derive(Serialize)]
pub struct ApikeysKey_Restrictions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    android_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKey_Restrictions_AndroidKeyRestrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_targets: std::option::Option<std::vec::Vec<ApikeysKey_Restrictions_ApiTargets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKey_Restrictions_BrowserKeyRestrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ios_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKey_Restrictions_IosKeyRestrictions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKey_Restrictions_ServerKeyRestrictions>>,
    dynamic: ApikeysKey_RestrictionsDynamic,
}

impl ApikeysKey_Restrictions {
    #[doc = "Set the field `android_key_restrictions`.\n"]
    pub fn set_android_key_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKey_Restrictions_AndroidKeyRestrictions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.android_key_restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.android_key_restrictions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `api_targets`.\n"]
    pub fn set_api_targets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKey_Restrictions_ApiTargets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `browser_key_restrictions`.\n"]
    pub fn set_browser_key_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKey_Restrictions_BrowserKeyRestrictions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.browser_key_restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.browser_key_restrictions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ios_key_restrictions`.\n"]
    pub fn set_ios_key_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKey_Restrictions_IosKeyRestrictions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ios_key_restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ios_key_restrictions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_key_restrictions`.\n"]
    pub fn set_server_key_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKey_Restrictions_ServerKeyRestrictions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_key_restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_key_restrictions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApikeysKey_Restrictions {
    type O = terrars::BlockAssignable<ApikeysKey_Restrictions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Restrictions {}

impl BuildApikeysKey_Restrictions {
    pub fn build(self) -> ApikeysKey_Restrictions {
        ApikeysKey_Restrictions {
            android_key_restrictions: core::default::Default::default(),
            api_targets: core::default::Default::default(),
            browser_key_restrictions: core::default::Default::default(),
            ios_key_restrictions: core::default::Default::default(),
            server_key_restrictions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_RestrictionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_RestrictionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKey_RestrictionsRef {
        ApikeysKey_RestrictionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_RestrictionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `android_key_restrictions` after provisioning.\n"]
    pub fn android_key_restrictions(&self) -> terrars::ListRef<ApikeysKey_Restrictions_AndroidKeyRestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.android_key_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `api_targets` after provisioning.\n"]
    pub fn api_targets(&self) -> terrars::ListRef<ApikeysKey_Restrictions_ApiTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_targets", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_key_restrictions` after provisioning.\n"]
    pub fn browser_key_restrictions(&self) -> terrars::ListRef<ApikeysKey_Restrictions_BrowserKeyRestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.browser_key_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `ios_key_restrictions` after provisioning.\n"]
    pub fn ios_key_restrictions(&self) -> terrars::ListRef<ApikeysKey_Restrictions_IosKeyRestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ios_key_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `server_key_restrictions` after provisioning.\n"]
    pub fn server_key_restrictions(&self) -> terrars::ListRef<ApikeysKey_Restrictions_ServerKeyRestrictionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_key_restrictions", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKey_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApikeysKey_Timeouts {
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

impl terrars::ToListMappable for ApikeysKey_Timeouts {
    type O = terrars::BlockAssignable<ApikeysKey_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKey_Timeouts {}

impl BuildApikeysKey_Timeouts {
    pub fn build(self) -> ApikeysKey_Timeouts {
        ApikeysKey_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApikeysKey_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKey_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKey_TimeoutsRef {
        ApikeysKey_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKey_TimeoutsRef {
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
