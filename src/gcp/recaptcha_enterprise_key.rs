#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct RecaptchaEnterpriseKeyData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    android_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKey_AndroidSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ios_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKey_IosSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_options: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKey_TestingOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RecaptchaEnterpriseKey_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waf_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKey_WafSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKey_WebSettings>>,
    dynamic: RecaptchaEnterpriseKeyDynamic,
}

struct RecaptchaEnterpriseKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RecaptchaEnterpriseKeyData_>,
}

#[derive(Clone)]
pub struct RecaptchaEnterpriseKey(std::rc::Rc<RecaptchaEnterpriseKey_>);

impl RecaptchaEnterpriseKey {
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
        "Set the field `labels`.\nSee [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
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

    #[doc = "Set the field `android_settings`.\n"]
    pub fn set_android_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKey_AndroidSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().android_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.android_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ios_settings`.\n"]
    pub fn set_ios_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKey_IosSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ios_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ios_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `testing_options`.\n"]
    pub fn set_testing_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKey_TestingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().testing_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.testing_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RecaptchaEnterpriseKey_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waf_settings`.\n"]
    pub fn set_waf_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKey_WafSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().waf_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.waf_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_settings`.\n"]
    pub fn set_web_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().web_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.web_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp corresponding to the creation of this Key."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this key. Modifiable by user."]
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
        "Get a reference to the value of field `labels` after provisioning.\nSee [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource id for the Key, which is the same as the Site Key itself."]
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

    #[doc = "Get a reference to the value of field `android_settings` after provisioning.\n"]
    pub fn android_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_AndroidSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.android_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ios_settings` after provisioning.\n"]
    pub fn ios_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_IosSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ios_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `testing_options` after provisioning.\n"]
    pub fn testing_options(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_TestingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.testing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecaptchaEnterpriseKey_TimeoutsRef {
        RecaptchaEnterpriseKey_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waf_settings` after provisioning.\n"]
    pub fn waf_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_WafSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waf_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_settings` after provisioning.\n"]
    pub fn web_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_WebSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_settings", self.extract_ref()))
    }
}

impl terrars::Referable for RecaptchaEnterpriseKey {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RecaptchaEnterpriseKey { }

impl terrars::ToListMappable for RecaptchaEnterpriseKey {
    type O = terrars::ListRef<RecaptchaEnterpriseKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RecaptchaEnterpriseKey_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_recaptcha_enterprise_key".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRecaptchaEnterpriseKey {
    pub tf_id: std::string::String,
    #[doc = "Human-readable display name of this key. Modifiable by user."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildRecaptchaEnterpriseKey {
    pub fn build(self, stack: &mut terrars::Stack) -> RecaptchaEnterpriseKey {
        let out = RecaptchaEnterpriseKey(std::rc::Rc::new(RecaptchaEnterpriseKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RecaptchaEnterpriseKeyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                android_settings: core::default::Default::default(),
                ios_settings: core::default::Default::default(),
                testing_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                waf_settings: core::default::Default::default(),
                web_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RecaptchaEnterpriseKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RecaptchaEnterpriseKeyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp corresponding to the creation of this Key."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this key. Modifiable by user."]
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
        "Get a reference to the value of field `labels` after provisioning.\nSee [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource id for the Key, which is the same as the Site Key itself."]
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

    #[doc = "Get a reference to the value of field `android_settings` after provisioning.\n"]
    pub fn android_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_AndroidSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.android_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ios_settings` after provisioning.\n"]
    pub fn ios_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_IosSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ios_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `testing_options` after provisioning.\n"]
    pub fn testing_options(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_TestingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.testing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecaptchaEnterpriseKey_TimeoutsRef {
        RecaptchaEnterpriseKey_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waf_settings` after provisioning.\n"]
    pub fn waf_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_WafSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waf_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_settings` after provisioning.\n"]
    pub fn web_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_WebSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKeyDynamic {
    android_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_AndroidSettings>>,
    ios_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_IosSettings>>,
    testing_options: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_TestingOptions>>,
    waf_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_WafSettings>>,
    web_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_WebSettings>>,
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_AndroidSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_package_names: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_package_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl RecaptchaEnterpriseKey_AndroidSettings {
    #[doc =
        "Set the field `allow_all_package_names`.\nIf set to true, it means allowed_package_names will not be enforced."]
    pub fn set_allow_all_package_names(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_all_package_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_package_names`.\nAndroid package names of apps allowed to use the key. Example: 'com.companyname.appname'"]
    pub fn set_allowed_package_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_package_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RecaptchaEnterpriseKey_AndroidSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_AndroidSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_AndroidSettings {}

impl BuildRecaptchaEnterpriseKey_AndroidSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_AndroidSettings {
        RecaptchaEnterpriseKey_AndroidSettings {
            allow_all_package_names: core::default::Default::default(),
            allowed_package_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_AndroidSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_AndroidSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKey_AndroidSettingsRef {
        RecaptchaEnterpriseKey_AndroidSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_AndroidSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all_package_names` after provisioning.\nIf set to true, it means allowed_package_names will not be enforced."]
    pub fn allow_all_package_names(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_package_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_package_names` after provisioning.\nAndroid package names of apps allowed to use the key. Example: 'com.companyname.appname'"]
    pub fn allowed_package_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_package_names", self.base))
    }
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_IosSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_bundle_ids: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_bundle_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl RecaptchaEnterpriseKey_IosSettings {
    #[doc = "Set the field `allow_all_bundle_ids`.\nIf set to true, it means allowed_bundle_ids will not be enforced."]
    pub fn set_allow_all_bundle_ids(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_all_bundle_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_bundle_ids`.\niOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'"]
    pub fn set_allowed_bundle_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_bundle_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RecaptchaEnterpriseKey_IosSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_IosSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_IosSettings {}

impl BuildRecaptchaEnterpriseKey_IosSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_IosSettings {
        RecaptchaEnterpriseKey_IosSettings {
            allow_all_bundle_ids: core::default::Default::default(),
            allowed_bundle_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_IosSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_IosSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKey_IosSettingsRef {
        RecaptchaEnterpriseKey_IosSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_IosSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all_bundle_ids` after provisioning.\nIf set to true, it means allowed_bundle_ids will not be enforced."]
    pub fn allow_all_bundle_ids(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_bundle_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_bundle_ids` after provisioning.\niOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'"]
    pub fn allowed_bundle_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_bundle_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_TestingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_challenge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_score: std::option::Option<terrars::PrimField<f64>>,
}

impl RecaptchaEnterpriseKey_TestingOptions {
    #[doc =
        "Set the field `testing_challenge`.\nFor challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if UNSOLVABLE_CHALLENGE. Possible values: TESTING_CHALLENGE_UNSPECIFIED, NOCAPTCHA, UNSOLVABLE_CHALLENGE"]
    pub fn set_testing_challenge(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.testing_challenge = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `testing_score`.\nAll assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive."]
    pub fn set_testing_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.testing_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RecaptchaEnterpriseKey_TestingOptions {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_TestingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_TestingOptions {}

impl BuildRecaptchaEnterpriseKey_TestingOptions {
    pub fn build(self) -> RecaptchaEnterpriseKey_TestingOptions {
        RecaptchaEnterpriseKey_TestingOptions {
            testing_challenge: core::default::Default::default(),
            testing_score: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_TestingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_TestingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKey_TestingOptionsRef {
        RecaptchaEnterpriseKey_TestingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_TestingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `testing_challenge` after provisioning.\nFor challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if UNSOLVABLE_CHALLENGE. Possible values: TESTING_CHALLENGE_UNSPECIFIED, NOCAPTCHA, UNSOLVABLE_CHALLENGE"]
    pub fn testing_challenge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.testing_challenge", self.base))
    }

    #[doc =
        "Get a reference to the value of field `testing_score` after provisioning.\nAll assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive."]
    pub fn testing_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.testing_score", self.base))
    }
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RecaptchaEnterpriseKey_Timeouts {
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

impl terrars::ToListMappable for RecaptchaEnterpriseKey_Timeouts {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_Timeouts {}

impl BuildRecaptchaEnterpriseKey_Timeouts {
    pub fn build(self) -> RecaptchaEnterpriseKey_Timeouts {
        RecaptchaEnterpriseKey_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKey_TimeoutsRef {
        RecaptchaEnterpriseKey_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_TimeoutsRef {
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
pub struct RecaptchaEnterpriseKey_WafSettings {
    waf_feature: terrars::PrimField<std::string::String>,
    waf_service: terrars::PrimField<std::string::String>,
}

impl RecaptchaEnterpriseKey_WafSettings { }

impl terrars::ToListMappable for RecaptchaEnterpriseKey_WafSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_WafSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_WafSettings {
    #[doc =
        "Supported WAF features. For more information, see https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features. Possible values: CHALLENGE_PAGE, SESSION_TOKEN, ACTION_TOKEN, EXPRESS"]
    pub waf_feature: terrars::PrimField<std::string::String>,
    #[doc = "The WAF service that uses this key. Possible values: CA, FASTLY"]
    pub waf_service: terrars::PrimField<std::string::String>,
}

impl BuildRecaptchaEnterpriseKey_WafSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_WafSettings {
        RecaptchaEnterpriseKey_WafSettings {
            waf_feature: self.waf_feature,
            waf_service: self.waf_service,
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_WafSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_WafSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKey_WafSettingsRef {
        RecaptchaEnterpriseKey_WafSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_WafSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `waf_feature` after provisioning.\nSupported WAF features. For more information, see https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features. Possible values: CHALLENGE_PAGE, SESSION_TOKEN, ACTION_TOKEN, EXPRESS"]
    pub fn waf_feature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf_feature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `waf_service` after provisioning.\nThe WAF service that uses this key. Possible values: CA, FASTLY"]
    pub fn waf_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waf_service", self.base))
    }
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings {
    action: terrars::PrimField<std::string::String>,
    score_threshold: terrars::PrimField<f64>,
}

impl RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings { }

impl terrars::ToListMappable for RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings {
    #[doc = "The action name."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "A challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub score_threshold: terrars::PrimField<f64>,
}

impl BuildRecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings {
        RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings {
            action: self.action,
            score_threshold: self.score_threshold,
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettingsRef {
        RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action name."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `score_threshold` after provisioning.\nA challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings {
    score_threshold: terrars::PrimField<f64>,
}

impl RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings { }

impl terrars::ToListMappable for RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings {
    #[doc =
        "A challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub score_threshold: terrars::PrimField<f64>,
}

impl BuildRecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings {
        RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings {
            score_threshold: self.score_threshold,
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettingsRef {
        RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score_threshold` after provisioning.\nA challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub fn score_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsDynamic {
    action_settings: std
    ::option
    ::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings>>,
    default_settings: std
    ::option
    ::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings>>,
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_settings: std
    ::option
    ::Option<std::vec::Vec<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_settings: std
    ::option
    ::Option<std::vec::Vec<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings>>,
    dynamic: RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsDynamic,
}

impl RecaptchaEnterpriseKey_WebSettings_ChallengeSettings {
    #[doc = "Set the field `action_settings`.\n"]
    pub fn set_action_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_ActionSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RecaptchaEnterpriseKey_WebSettings_ChallengeSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_WebSettings_ChallengeSettings {}

impl BuildRecaptchaEnterpriseKey_WebSettings_ChallengeSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_WebSettings_ChallengeSettings {
        RecaptchaEnterpriseKey_WebSettings_ChallengeSettings {
            action_settings: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsRef {
        RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(
        &self,
    ) -> terrars::ListRef<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings_DefaultSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKey_WebSettingsDynamic {
    challenge_settings: std
    ::option
    ::Option<terrars::DynamicBlock<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings>>,
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKey_WebSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_domains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_amp_traffic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_security_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings>>,
    dynamic: RecaptchaEnterpriseKey_WebSettingsDynamic,
}

impl RecaptchaEnterpriseKey_WebSettings {
    #[doc = "Set the field `allow_all_domains`.\nIf set to true, it means allowed_domains will not be enforced."]
    pub fn set_allow_all_domains(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_all_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_amp_traffic`.\nIf set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type."]
    pub fn set_allow_amp_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_amp_traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_domains`.\nDomains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'"]
    pub fn set_allowed_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `challenge_security_preference`.\nSettings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY"]
    pub fn set_challenge_security_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.challenge_security_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `challenge_settings`.\n"]
    pub fn set_challenge_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings_ChallengeSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RecaptchaEnterpriseKey_WebSettings {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKey_WebSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKey_WebSettings {
    #[doc =
        "Required. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE, POLICY_BASED_CHALLENGE"]
    pub integration_type: terrars::PrimField<std::string::String>,
}

impl BuildRecaptchaEnterpriseKey_WebSettings {
    pub fn build(self) -> RecaptchaEnterpriseKey_WebSettings {
        RecaptchaEnterpriseKey_WebSettings {
            allow_all_domains: core::default::Default::default(),
            allow_amp_traffic: core::default::Default::default(),
            allowed_domains: core::default::Default::default(),
            challenge_security_preference: core::default::Default::default(),
            integration_type: self.integration_type,
            challenge_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RecaptchaEnterpriseKey_WebSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKey_WebSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKey_WebSettingsRef {
        RecaptchaEnterpriseKey_WebSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKey_WebSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all_domains` after provisioning.\nIf set to true, it means allowed_domains will not be enforced."]
    pub fn allow_all_domains(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all_domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_amp_traffic` after provisioning.\nIf set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type."]
    pub fn allow_amp_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_amp_traffic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_domains` after provisioning.\nDomains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'"]
    pub fn allowed_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `challenge_security_preference` after provisioning.\nSettings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY"]
    pub fn challenge_security_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.challenge_security_preference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integration_type` after provisioning.\nRequired. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE, POLICY_BASED_CHALLENGE"]
    pub fn integration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge_settings` after provisioning.\n"]
    pub fn challenge_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKey_WebSettings_ChallengeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge_settings", self.base))
    }
}
