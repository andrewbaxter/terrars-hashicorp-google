use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct RecaptchaEnterpriseKeyData {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    android_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKeyAndroidSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ios_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKeyIosSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_options: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKeyTestingOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RecaptchaEnterpriseKeyTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waf_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKeyWafSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_settings: std::option::Option<std::vec::Vec<RecaptchaEnterpriseKeyWebSettingsEl>>,
    dynamic: RecaptchaEnterpriseKeyDynamic,
}

struct RecaptchaEnterpriseKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RecaptchaEnterpriseKeyData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKeyAndroidSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKeyIosSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKeyTestingOptionsEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<RecaptchaEnterpriseKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waf_settings`.\n"]
    pub fn set_waf_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKeyWafSettingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RecaptchaEnterpriseKeyWebSettingsEl>>,
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Key in the format \"projects/{project}/keys/{key}\"."]
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
    pub fn android_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyAndroidSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.android_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ios_settings` after provisioning.\n"]
    pub fn ios_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyIosSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ios_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `testing_options` after provisioning.\n"]
    pub fn testing_options(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyTestingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.testing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecaptchaEnterpriseKeyTimeoutsElRef {
        RecaptchaEnterpriseKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waf_settings` after provisioning.\n"]
    pub fn waf_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyWafSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waf_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_settings` after provisioning.\n"]
    pub fn web_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyWebSettingsElRef> {
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
            data: std::cell::RefCell::new(RecaptchaEnterpriseKeyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Key in the format \"projects/{project}/keys/{key}\"."]
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
    pub fn android_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyAndroidSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.android_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ios_settings` after provisioning.\n"]
    pub fn ios_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyIosSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ios_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `testing_options` after provisioning.\n"]
    pub fn testing_options(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyTestingOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.testing_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecaptchaEnterpriseKeyTimeoutsElRef {
        RecaptchaEnterpriseKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `waf_settings` after provisioning.\n"]
    pub fn waf_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyWafSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.waf_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_settings` after provisioning.\n"]
    pub fn web_settings(&self) -> terrars::ListRef<RecaptchaEnterpriseKeyWebSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyAndroidSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_package_names: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_package_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl RecaptchaEnterpriseKeyAndroidSettingsEl {
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

impl terrars::ToListMappable for RecaptchaEnterpriseKeyAndroidSettingsEl {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKeyAndroidSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKeyAndroidSettingsEl {}

impl BuildRecaptchaEnterpriseKeyAndroidSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyAndroidSettingsEl {
        RecaptchaEnterpriseKeyAndroidSettingsEl {
            allow_all_package_names: core::default::Default::default(),
            allowed_package_names: core::default::Default::default(),
        }
    }
}

pub struct RecaptchaEnterpriseKeyAndroidSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyAndroidSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKeyAndroidSettingsElRef {
        RecaptchaEnterpriseKeyAndroidSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKeyAndroidSettingsElRef {
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
pub struct RecaptchaEnterpriseKeyIosSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_bundle_ids: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_bundle_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl RecaptchaEnterpriseKeyIosSettingsEl {
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

impl terrars::ToListMappable for RecaptchaEnterpriseKeyIosSettingsEl {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKeyIosSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKeyIosSettingsEl {}

impl BuildRecaptchaEnterpriseKeyIosSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyIosSettingsEl {
        RecaptchaEnterpriseKeyIosSettingsEl {
            allow_all_bundle_ids: core::default::Default::default(),
            allowed_bundle_ids: core::default::Default::default(),
        }
    }
}

pub struct RecaptchaEnterpriseKeyIosSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyIosSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKeyIosSettingsElRef {
        RecaptchaEnterpriseKeyIosSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKeyIosSettingsElRef {
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
pub struct RecaptchaEnterpriseKeyTestingOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_challenge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_score: std::option::Option<terrars::PrimField<f64>>,
}

impl RecaptchaEnterpriseKeyTestingOptionsEl {
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

impl terrars::ToListMappable for RecaptchaEnterpriseKeyTestingOptionsEl {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKeyTestingOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKeyTestingOptionsEl {}

impl BuildRecaptchaEnterpriseKeyTestingOptionsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyTestingOptionsEl {
        RecaptchaEnterpriseKeyTestingOptionsEl {
            testing_challenge: core::default::Default::default(),
            testing_score: core::default::Default::default(),
        }
    }
}

pub struct RecaptchaEnterpriseKeyTestingOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyTestingOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKeyTestingOptionsElRef {
        RecaptchaEnterpriseKeyTestingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKeyTestingOptionsElRef {
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
pub struct RecaptchaEnterpriseKeyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RecaptchaEnterpriseKeyTimeoutsEl {
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

impl terrars::ToListMappable for RecaptchaEnterpriseKeyTimeoutsEl {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKeyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKeyTimeoutsEl {}

impl BuildRecaptchaEnterpriseKeyTimeoutsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyTimeoutsEl {
        RecaptchaEnterpriseKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct RecaptchaEnterpriseKeyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKeyTimeoutsElRef {
        RecaptchaEnterpriseKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKeyTimeoutsElRef {
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
pub struct RecaptchaEnterpriseKeyWafSettingsEl {
    waf_feature: terrars::PrimField<std::string::String>,
    waf_service: terrars::PrimField<std::string::String>,
}

impl RecaptchaEnterpriseKeyWafSettingsEl { }

impl terrars::ToListMappable for RecaptchaEnterpriseKeyWafSettingsEl {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKeyWafSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKeyWafSettingsEl {
    #[doc =
        "Supported WAF features. For more information, see https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features. Possible values: CHALLENGE_PAGE, SESSION_TOKEN, ACTION_TOKEN, EXPRESS"]
    pub waf_feature: terrars::PrimField<std::string::String>,
    #[doc = "The WAF service that uses this key. Possible values: CA, FASTLY"]
    pub waf_service: terrars::PrimField<std::string::String>,
}

impl BuildRecaptchaEnterpriseKeyWafSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWafSettingsEl {
        RecaptchaEnterpriseKeyWafSettingsEl {
            waf_feature: self.waf_feature,
            waf_service: self.waf_service,
        }
    }
}

pub struct RecaptchaEnterpriseKeyWafSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyWafSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKeyWafSettingsElRef {
        RecaptchaEnterpriseKeyWafSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKeyWafSettingsElRef {
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
pub struct RecaptchaEnterpriseKeyWebSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all_domains: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_amp_traffic: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_security_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    integration_type: terrars::PrimField<std::string::String>,
}

impl RecaptchaEnterpriseKeyWebSettingsEl {
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
}

impl terrars::ToListMappable for RecaptchaEnterpriseKeyWebSettingsEl {
    type O = terrars::BlockAssignable<RecaptchaEnterpriseKeyWebSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRecaptchaEnterpriseKeyWebSettingsEl {
    #[doc =
        "Required. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE"]
    pub integration_type: terrars::PrimField<std::string::String>,
}

impl BuildRecaptchaEnterpriseKeyWebSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWebSettingsEl {
        RecaptchaEnterpriseKeyWebSettingsEl {
            allow_all_domains: core::default::Default::default(),
            allow_amp_traffic: core::default::Default::default(),
            allowed_domains: core::default::Default::default(),
            challenge_security_preference: core::default::Default::default(),
            integration_type: self.integration_type,
        }
    }
}

pub struct RecaptchaEnterpriseKeyWebSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RecaptchaEnterpriseKeyWebSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RecaptchaEnterpriseKeyWebSettingsElRef {
        RecaptchaEnterpriseKeyWebSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RecaptchaEnterpriseKeyWebSettingsElRef {
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
        "Get a reference to the value of field `integration_type` after provisioning.\nRequired. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE"]
    pub fn integration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKeyDynamic {
    android_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKeyAndroidSettingsEl>>,
    ios_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKeyIosSettingsEl>>,
    testing_options: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKeyTestingOptionsEl>>,
    waf_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKeyWafSettingsEl>>,
    web_settings: std::option::Option<terrars::DynamicBlock<RecaptchaEnterpriseKeyWebSettingsEl>>,
}
