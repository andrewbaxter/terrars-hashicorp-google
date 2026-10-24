use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApikeysKeyData {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions: std::option::Option<std::vec::Vec<ApikeysKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApikeysKeyTimeoutsEl>,
    dynamic: ApikeysKeyDynamic,
}

struct ApikeysKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApikeysKeyData>,
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

    #[doc = "Set the field `restrictions`.\n"]
    pub fn set_restrictions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKeyRestrictionsEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ApikeysKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique id in UUID4 format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApikeysKeyTimeoutsElRef {
        ApikeysKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(ApikeysKeyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique id in UUID4 format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions` after provisioning.\n"]
    pub fn restrictions(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApikeysKeyTimeoutsElRef {
        ApikeysKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    package_name: terrars::PrimField<std::string::String>,
    sha1_fingerprint: terrars::PrimField<std::string::String>,
}

impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl { }

impl terrars::ToListMappable for ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    #[doc = "The package name of the application."]
    pub package_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter."]
    pub sha1_fingerprint: terrars::PrimField<std::string::String>,
}

impl BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl {
            package_name: self.package_name,
            sha1_fingerprint: self.sha1_fingerprint,
        }
    }
}

pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef {
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
struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElDynamic {
    allowed_applications: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>>,
}

#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_applications: std
    ::option
    ::Option<std::vec::Vec<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>>,
    dynamic: ApikeysKeyRestrictionsElAndroidKeyRestrictionsElDynamic,
}

impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    #[doc = "Set the field `allowed_applications`.\n"]
    pub fn set_allowed_applications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsEl>,
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

impl terrars::ToListMappable for ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {}

impl BuildApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl {
            allowed_applications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_applications` after provisioning.\n"]
    pub fn allowed_applications(
        &self,
    ) -> terrars::ListRef<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElAllowedApplicationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_applications", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsElApiTargetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    service: terrars::PrimField<std::string::String>,
}

impl ApikeysKeyRestrictionsElApiTargetsEl {
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

impl terrars::ToListMappable for ApikeysKeyRestrictionsElApiTargetsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsElApiTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsElApiTargetsEl {
    #[doc =
        "The service for this restriction. It should be the canonical service name, for example: `translate.googleapis.com`. You can use `gcloud services list` to get a list of services that are enabled in the project."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildApikeysKeyRestrictionsElApiTargetsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElApiTargetsEl {
        ApikeysKeyRestrictionsElApiTargetsEl {
            methods: core::default::Default::default(),
            service: self.service,
        }
    }
}

pub struct ApikeysKeyRestrictionsElApiTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElApiTargetsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKeyRestrictionsElApiTargetsElRef {
        ApikeysKeyRestrictionsElApiTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElApiTargetsElRef {
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
pub struct ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    allowed_referrers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl { }

impl terrars::ToListMappable for ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    #[doc = "A list of regular expressions for the referrer URLs that are allowed to make API calls with this key."]
    pub allowed_referrers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl {
        ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl { allowed_referrers: self.allowed_referrers }
    }
}

pub struct ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef {
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
pub struct ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    allowed_bundle_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ApikeysKeyRestrictionsElIosKeyRestrictionsEl { }

impl terrars::ToListMappable for ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    #[doc = "A list of bundle IDs that are allowed when making API calls with this key."]
    pub allowed_bundle_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApikeysKeyRestrictionsElIosKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElIosKeyRestrictionsEl {
        ApikeysKeyRestrictionsElIosKeyRestrictionsEl { allowed_bundle_ids: self.allowed_bundle_ids }
    }
}

pub struct ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElIosKeyRestrictionsElRef {
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
pub struct ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    allowed_ips: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ApikeysKeyRestrictionsElServerKeyRestrictionsEl { }

impl terrars::ToListMappable for ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    #[doc = "A list of the caller IP addresses that are allowed to make API calls with this key."]
    pub allowed_ips: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApikeysKeyRestrictionsElServerKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsElServerKeyRestrictionsEl {
        ApikeysKeyRestrictionsElServerKeyRestrictionsEl { allowed_ips: self.allowed_ips }
    }
}

pub struct ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElServerKeyRestrictionsElRef {
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
struct ApikeysKeyRestrictionsElDynamic {
    android_key_restrictions: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>>,
    api_targets: std::option::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsElApiTargetsEl>>,
    browser_key_restrictions: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>>,
    ios_key_restrictions: std::option::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>>,
    server_key_restrictions: std
    ::option
    ::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>>,
}

#[derive(Serialize)]
pub struct ApikeysKeyRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    android_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_targets: std::option::Option<std::vec::Vec<ApikeysKeyRestrictionsElApiTargetsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    browser_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ios_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_key_restrictions: std::option::Option<std::vec::Vec<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>>,
    dynamic: ApikeysKeyRestrictionsElDynamic,
}

impl ApikeysKeyRestrictionsEl {
    #[doc = "Set the field `android_key_restrictions`.\n"]
    pub fn set_android_key_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKeyRestrictionsElAndroidKeyRestrictionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKeyRestrictionsElApiTargetsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKeyRestrictionsElBrowserKeyRestrictionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKeyRestrictionsElIosKeyRestrictionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApikeysKeyRestrictionsElServerKeyRestrictionsEl>>,
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

impl terrars::ToListMappable for ApikeysKeyRestrictionsEl {
    type O = terrars::BlockAssignable<ApikeysKeyRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyRestrictionsEl {}

impl BuildApikeysKeyRestrictionsEl {
    pub fn build(self) -> ApikeysKeyRestrictionsEl {
        ApikeysKeyRestrictionsEl {
            android_key_restrictions: core::default::Default::default(),
            api_targets: core::default::Default::default(),
            browser_key_restrictions: core::default::Default::default(),
            ios_key_restrictions: core::default::Default::default(),
            server_key_restrictions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApikeysKeyRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyRestrictionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKeyRestrictionsElRef {
        ApikeysKeyRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyRestrictionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `android_key_restrictions` after provisioning.\n"]
    pub fn android_key_restrictions(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElAndroidKeyRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.android_key_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `api_targets` after provisioning.\n"]
    pub fn api_targets(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElApiTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_targets", self.base))
    }

    #[doc = "Get a reference to the value of field `browser_key_restrictions` after provisioning.\n"]
    pub fn browser_key_restrictions(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElBrowserKeyRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.browser_key_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `ios_key_restrictions` after provisioning.\n"]
    pub fn ios_key_restrictions(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElIosKeyRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ios_key_restrictions", self.base))
    }

    #[doc = "Get a reference to the value of field `server_key_restrictions` after provisioning.\n"]
    pub fn server_key_restrictions(&self) -> terrars::ListRef<ApikeysKeyRestrictionsElServerKeyRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_key_restrictions", self.base))
    }
}

#[derive(Serialize)]
pub struct ApikeysKeyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApikeysKeyTimeoutsEl {
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

impl terrars::ToListMappable for ApikeysKeyTimeoutsEl {
    type O = terrars::BlockAssignable<ApikeysKeyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApikeysKeyTimeoutsEl {}

impl BuildApikeysKeyTimeoutsEl {
    pub fn build(self) -> ApikeysKeyTimeoutsEl {
        ApikeysKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ApikeysKeyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApikeysKeyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApikeysKeyTimeoutsElRef {
        ApikeysKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApikeysKeyTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ApikeysKeyDynamic {
    restrictions: std::option::Option<terrars::DynamicBlock<ApikeysKeyRestrictionsEl>>,
}
