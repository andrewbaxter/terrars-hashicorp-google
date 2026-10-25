#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeDeveloperAppData_ {
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
    api_products: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_family: std::option::Option<terrars::PrimField<std::string::String>>,
    callback_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    developer_email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_expires_in: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    org_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<ApigeeDeveloperApp_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeDeveloperApp_Timeouts>,
    dynamic: ApigeeDeveloperAppDynamic,
}

struct ApigeeDeveloperApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeDeveloperAppData_>,
}

#[derive(Clone)]
pub struct ApigeeDeveloperApp(std::rc::Rc<ApigeeDeveloperApp_>);

impl ApigeeDeveloperApp {
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

    #[doc = "Set the field `api_products`.\nList of API products associated with the developer app."]
    pub fn set_api_products(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().api_products = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_family`.\nDeveloper app family."]
    pub fn set_app_family(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().app_family = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consumer_key`.\nOptionally specify a static consumer key for the developer app's credential.\nIf not set, the API auto-generates a key. The consumer key must be unique\nacross all developer apps in an organization. Changing this field forces the\nresource to be recreated.\n\nThis is a write-only input used at create time: the provider creates the\ncredential with this key via the keys API and removes the auto-generated\none. The effective key is exposed in the 'credentials' output."]
    pub fn set_consumer_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().consumer_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consumer_secret`.\nOptionally specify a static consumer secret for the developer app's\ncredential. Required if 'consumer_key' is specified. If not set, the API\nauto-generates a secret. Changing this field forces the resource to be\nrecreated.\n\nThis is a write-only input used at create time; the effective secret is\nexposed in the 'credentials' output."]
    pub fn set_consumer_secret(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().consumer_secret = std::option::Option::Some(v.into());
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
        "Set the field `key_expires_in`.\nExpiration time, in milliseconds, for the consumer key that is generated\nfor the developer app. If not set or left to the default value of -1,\nthe API key never expires. The expiration time can't be updated after it is set."]
    pub fn set_key_expires_in(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_expires_in = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scopes`.\nScopes to apply to the developer app.\nThe specified scopes must already exist for the API product that\nyou associate with the developer app."]
    pub fn set_scopes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\nStatus of the credential. Valid values include approved or revoked."]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeDeveloperApp_Attributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeDeveloperApp_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `api_products` after provisioning.\nList of API products associated with the developer app."]
    pub fn api_products(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.api_products", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_family` after provisioning.\nDeveloper app family."]
    pub fn app_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_family", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nID of the developer app. This ID is not user specified but is\nautomatically generated on app creation. appId is a UUID."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `callback_url` after provisioning.\nCallback URL used by OAuth 2.0 authorization servers to communicate\nauthorization codes back to developer apps."]
    pub fn callback_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.callback_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_key` after provisioning.\nOptionally specify a static consumer key for the developer app's credential.\nIf not set, the API auto-generates a key. The consumer key must be unique\nacross all developer apps in an organization. Changing this field forces the\nresource to be recreated.\n\nThis is a write-only input used at create time: the provider creates the\ncredential with this key via the keys API and removes the auto-generated\none. The effective key is exposed in the 'credentials' output."]
    pub fn consumer_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_secret` after provisioning.\nOptionally specify a static consumer secret for the developer app's\ncredential. Required if 'consumer_key' is specified. If not set, the API\nauto-generates a secret. Changing this field forces the resource to be\nrecreated.\n\nThis is a write-only input used at create time; the effective secret is\nexposed in the 'credentials' output."]
    pub fn consumer_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTime at which the developer was created in milliseconds since epoch."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credentials` after provisioning.\nOutput only. Set of credentials for the developer app consisting of\nthe consumer key/secret pairs associated with the API products."]
    pub fn credentials(&self) -> terrars::ListRef<ApigeeDeveloperApp_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `developer_email` after provisioning.\nEmail address of the developer.\nThis value is used to uniquely identify the developer in Apigee hybrid.\nNote that the email address has to be in lowercase only."]
    pub fn developer_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_id` after provisioning.\nID of the developer."]
    pub fn developer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_expires_in` after provisioning.\nExpiration time, in milliseconds, for the consumer key that is generated\nfor the developer app. If not set or left to the default value of -1,\nthe API key never expires. The expiration time can't be updated after it is set."]
    pub fn key_expires_in(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_expires_in", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_at` after provisioning.\nTime at which the developer was last modified in milliseconds since epoch."]
    pub fn last_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the developer app."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nScopes to apply to the developer app.\nThe specified scopes must already exist for the API product that\nyou associate with the developer app."]
    pub fn scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the credential. Valid values include approved or revoked."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<ApigeeDeveloperApp_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeDeveloperApp_TimeoutsRef {
        ApigeeDeveloperApp_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeDeveloperApp {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeDeveloperApp { }

impl terrars::ToListMappable for ApigeeDeveloperApp {
    type O = terrars::ListRef<ApigeeDeveloperAppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeDeveloperApp_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_developer_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeDeveloperApp {
    pub tf_id: std::string::String,
    #[doc =
        "Callback URL used by OAuth 2.0 authorization servers to communicate\nauthorization codes back to developer apps."]
    pub callback_url: terrars::PrimField<std::string::String>,
    #[doc =
        "Email address of the developer.\nThis value is used to uniquely identify the developer in Apigee hybrid.\nNote that the email address has to be in lowercase only."]
    pub developer_email: terrars::PrimField<std::string::String>,
    #[doc = "Name of the developer app."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: terrars::PrimField<std::string::String>,
}

impl BuildApigeeDeveloperApp {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeDeveloperApp {
        let out = ApigeeDeveloperApp(std::rc::Rc::new(ApigeeDeveloperApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeDeveloperAppData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_products: core::default::Default::default(),
                app_family: core::default::Default::default(),
                callback_url: self.callback_url,
                consumer_key: core::default::Default::default(),
                consumer_secret: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                developer_email: self.developer_email,
                id: core::default::Default::default(),
                key_expires_in: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                scopes: core::default::Default::default(),
                status: core::default::Default::default(),
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeDeveloperAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeDeveloperAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeDeveloperAppRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_products` after provisioning.\nList of API products associated with the developer app."]
    pub fn api_products(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.api_products", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_family` after provisioning.\nDeveloper app family."]
    pub fn app_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_family", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nID of the developer app. This ID is not user specified but is\nautomatically generated on app creation. appId is a UUID."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `callback_url` after provisioning.\nCallback URL used by OAuth 2.0 authorization servers to communicate\nauthorization codes back to developer apps."]
    pub fn callback_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.callback_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_key` after provisioning.\nOptionally specify a static consumer key for the developer app's credential.\nIf not set, the API auto-generates a key. The consumer key must be unique\nacross all developer apps in an organization. Changing this field forces the\nresource to be recreated.\n\nThis is a write-only input used at create time: the provider creates the\ncredential with this key via the keys API and removes the auto-generated\none. The effective key is exposed in the 'credentials' output."]
    pub fn consumer_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consumer_secret` after provisioning.\nOptionally specify a static consumer secret for the developer app's\ncredential. Required if 'consumer_key' is specified. If not set, the API\nauto-generates a secret. Changing this field forces the resource to be\nrecreated.\n\nThis is a write-only input used at create time; the effective secret is\nexposed in the 'credentials' output."]
    pub fn consumer_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTime at which the developer was created in milliseconds since epoch."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credentials` after provisioning.\nOutput only. Set of credentials for the developer app consisting of\nthe consumer key/secret pairs associated with the API products."]
    pub fn credentials(&self) -> terrars::ListRef<ApigeeDeveloperApp_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `developer_email` after provisioning.\nEmail address of the developer.\nThis value is used to uniquely identify the developer in Apigee hybrid.\nNote that the email address has to be in lowercase only."]
    pub fn developer_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `developer_id` after provisioning.\nID of the developer."]
    pub fn developer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.developer_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_expires_in` after provisioning.\nExpiration time, in milliseconds, for the consumer key that is generated\nfor the developer app. If not set or left to the default value of -1,\nthe API key never expires. The expiration time can't be updated after it is set."]
    pub fn key_expires_in(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_expires_in", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_at` after provisioning.\nTime at which the developer was last modified in milliseconds since epoch."]
    pub fn last_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the developer app."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nScopes to apply to the developer app.\nThe specified scopes must already exist for the API product that\nyou associate with the developer app."]
    pub fn scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nStatus of the credential. Valid values include approved or revoked."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<ApigeeDeveloperApp_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeDeveloperApp_TimeoutsRef {
        ApigeeDeveloperApp_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApigeeDeveloperAppDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<ApigeeDeveloperApp_Attributes>>,
}

#[derive(Serialize)]
pub struct ApigeeDeveloperApp_Credentials_ApiProducts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apiproduct: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeDeveloperApp_Credentials_ApiProducts {
    #[doc = "Set the field `apiproduct`.\n"]
    pub fn set_apiproduct(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.apiproduct = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeDeveloperApp_Credentials_ApiProducts {
    type O = terrars::BlockAssignable<ApigeeDeveloperApp_Credentials_ApiProducts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeDeveloperApp_Credentials_ApiProducts {}

impl BuildApigeeDeveloperApp_Credentials_ApiProducts {
    pub fn build(self) -> ApigeeDeveloperApp_Credentials_ApiProducts {
        ApigeeDeveloperApp_Credentials_ApiProducts {
            apiproduct: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeDeveloperApp_Credentials_ApiProductsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeDeveloperApp_Credentials_ApiProductsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeDeveloperApp_Credentials_ApiProductsRef {
        ApigeeDeveloperApp_Credentials_ApiProductsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeDeveloperApp_Credentials_ApiProductsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apiproduct` after provisioning.\n"]
    pub fn apiproduct(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apiproduct", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeDeveloperApp_Credentials_Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeDeveloperApp_Credentials_Attributes {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeDeveloperApp_Credentials_Attributes {
    type O = terrars::BlockAssignable<ApigeeDeveloperApp_Credentials_Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeDeveloperApp_Credentials_Attributes {}

impl BuildApigeeDeveloperApp_Credentials_Attributes {
    pub fn build(self) -> ApigeeDeveloperApp_Credentials_Attributes {
        ApigeeDeveloperApp_Credentials_Attributes {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeDeveloperApp_Credentials_AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeDeveloperApp_Credentials_AttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeDeveloperApp_Credentials_AttributesRef {
        ApigeeDeveloperApp_Credentials_AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeDeveloperApp_Credentials_AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeDeveloperApp_Credentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_products: std::option::Option<terrars::ListField<ApigeeDeveloperApp_Credentials_ApiProducts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::ListField<ApigeeDeveloperApp_Credentials_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expires_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issued_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeDeveloperApp_Credentials {
    #[doc = "Set the field `api_products`.\n"]
    pub fn set_api_products(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApigeeDeveloperApp_Credentials_ApiProducts>>,
    ) -> Self {
        self.api_products = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApigeeDeveloperApp_Credentials_Attributes>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_key`.\n"]
    pub fn set_consumer_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_secret`.\n"]
    pub fn set_consumer_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expires_at`.\n"]
    pub fn set_expires_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expires_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issued_at`.\n"]
    pub fn set_issued_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issued_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeDeveloperApp_Credentials {
    type O = terrars::BlockAssignable<ApigeeDeveloperApp_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeDeveloperApp_Credentials {}

impl BuildApigeeDeveloperApp_Credentials {
    pub fn build(self) -> ApigeeDeveloperApp_Credentials {
        ApigeeDeveloperApp_Credentials {
            api_products: core::default::Default::default(),
            attributes: core::default::Default::default(),
            consumer_key: core::default::Default::default(),
            consumer_secret: core::default::Default::default(),
            expires_at: core::default::Default::default(),
            issued_at: core::default::Default::default(),
            scopes: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeDeveloperApp_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeDeveloperApp_CredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeDeveloperApp_CredentialsRef {
        ApigeeDeveloperApp_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeDeveloperApp_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_products` after provisioning.\n"]
    pub fn api_products(&self) -> terrars::ListRef<ApigeeDeveloperApp_Credentials_ApiProductsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_products", self.base))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<ApigeeDeveloperApp_Credentials_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_key` after provisioning.\n"]
    pub fn consumer_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_key", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_secret` after provisioning.\n"]
    pub fn consumer_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `expires_at` after provisioning.\n"]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }

    #[doc = "Get a reference to the value of field `issued_at` after provisioning.\n"]
    pub fn issued_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issued_at", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeDeveloperApp_Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeDeveloperApp_Attributes {
    #[doc = "Set the field `name`.\nKey of the attribute"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue of the attribute"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeDeveloperApp_Attributes {
    type O = terrars::BlockAssignable<ApigeeDeveloperApp_Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeDeveloperApp_Attributes {}

impl BuildApigeeDeveloperApp_Attributes {
    pub fn build(self) -> ApigeeDeveloperApp_Attributes {
        ApigeeDeveloperApp_Attributes {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeDeveloperApp_AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeDeveloperApp_AttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeDeveloperApp_AttributesRef {
        ApigeeDeveloperApp_AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeDeveloperApp_AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeDeveloperApp_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeDeveloperApp_Timeouts {
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

impl terrars::ToListMappable for ApigeeDeveloperApp_Timeouts {
    type O = terrars::BlockAssignable<ApigeeDeveloperApp_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeDeveloperApp_Timeouts {}

impl BuildApigeeDeveloperApp_Timeouts {
    pub fn build(self) -> ApigeeDeveloperApp_Timeouts {
        ApigeeDeveloperApp_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeDeveloperApp_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeDeveloperApp_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeDeveloperApp_TimeoutsRef {
        ApigeeDeveloperApp_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeDeveloperApp_TimeoutsRef {
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
