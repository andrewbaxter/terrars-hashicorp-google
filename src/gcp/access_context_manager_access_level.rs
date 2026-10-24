use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerAccessLevelData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic: std::option::Option<std::vec::Vec<AccessContextManagerAccessLevelBasicEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom: std::option::Option<std::vec::Vec<AccessContextManagerAccessLevelCustomEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerAccessLevelTimeoutsEl>,
    dynamic: AccessContextManagerAccessLevelDynamic,
}

struct AccessContextManagerAccessLevel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerAccessLevelData>,
}

#[derive(Clone)]
pub struct AccessContextManagerAccessLevel(std::rc::Rc<AccessContextManagerAccessLevel_>);

impl AccessContextManagerAccessLevel {
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

    #[doc = "Set the field `description`.\nDescription of the AccessLevel and its use. Does not affect behavior."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerAccessLevelBasicEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom`.\n"]
    pub fn set_custom(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerAccessLevelCustomEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AccessContextManagerAccessLevelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the AccessLevel and its use. Does not affect behavior."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the Access Level. The short_name component must begin\nwith a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this AccessLevel lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(&self) -> terrars::ListRef<AccessContextManagerAccessLevelCustomElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAccessLevelTimeoutsElRef {
        AccessContextManagerAccessLevelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerAccessLevel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerAccessLevel { }

impl terrars::ToListMappable for AccessContextManagerAccessLevel {
    type O = terrars::ListRef<AccessContextManagerAccessLevelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerAccessLevel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_access_level".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerAccessLevel {
    pub tf_id: std::string::String,
    #[doc =
        "Resource name for the Access Level. The short_name component must begin\nwith a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The AccessPolicy this AccessLevel lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerAccessLevel {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerAccessLevel {
        let out = AccessContextManagerAccessLevel(std::rc::Rc::new(AccessContextManagerAccessLevel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessContextManagerAccessLevelData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                title: self.title,
                basic: core::default::Default::default(),
                custom: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerAccessLevelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerAccessLevelRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the AccessLevel and its use. Does not affect behavior."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the Access Level. The short_name component must begin\nwith a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this AccessLevel lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(&self) -> terrars::ListRef<AccessContextManagerAccessLevelCustomElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAccessLevelTimeoutsElRef {
        AccessContextManagerAccessLevelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_version: std::option::Option<terrars::PrimField<std::string::String>>,
    os_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_verified_chrome_os: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    #[doc =
        "Set the field `minimum_version`.\nThe minimum allowed OS version. If not set, any version\nof this OS satisfies the constraint.\nFormat: \"major.minor.patch\" such as \"10.5.301\", \"9.2.1\"."]
    pub fn set_minimum_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.minimum_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_verified_chrome_os`.\nIf you specify DESKTOP_CHROME_OS for osType, you can optionally include requireVerifiedChromeOs to require Chrome Verified Access."]
    pub fn set_require_verified_chrome_os(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_verified_chrome_os = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    #[doc =
        "The operating system type of the device. Possible values: [\"OS_UNSPECIFIED\", \"DESKTOP_MAC\", \"DESKTOP_WINDOWS\", \"DESKTOP_LINUX\", \"DESKTOP_CHROME_OS\", \"ANDROID\", \"IOS\"]"]
    pub os_type: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
            minimum_version: core::default::Default::default(),
            os_type: self.os_type,
            require_verified_chrome_os: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `minimum_version` after provisioning.\nThe minimum allowed OS version. If not set, any version\nof this OS satisfies the constraint.\nFormat: \"major.minor.patch\" such as \"10.5.301\", \"9.2.1\"."]
    pub fn minimum_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_type` after provisioning.\nThe operating system type of the device. Possible values: [\"OS_UNSPECIFIED\", \"DESKTOP_MAC\", \"DESKTOP_WINDOWS\", \"DESKTOP_LINUX\", \"DESKTOP_CHROME_OS\", \"ANDROID\", \"IOS\"]"]
    pub fn os_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_verified_chrome_os` after provisioning.\nIf you specify DESKTOP_CHROME_OS for osType, you can optionally include requireVerifiedChromeOs to require Chrome Verified Access."]
    pub fn require_verified_chrome_os(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_verified_chrome_os", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElDynamic {
    os_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_device_management_levels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_encryption_statuses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_admin_approval: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_corp_owned: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_screen_lock: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_constraints: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl>>,
    dynamic: AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElDynamic,
}

impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    #[doc =
        "Set the field `allowed_device_management_levels`.\nA list of allowed device management levels.\nAn empty list allows all management levels. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"NONE\", \"BASIC\", \"COMPLETE\"]"]
    pub fn set_allowed_device_management_levels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_device_management_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allowed_encryption_statuses`.\nA list of allowed encryptions statuses.\nAn empty list allows all statuses. Possible values: [\"ENCRYPTION_UNSPECIFIED\", \"ENCRYPTION_UNSUPPORTED\", \"UNENCRYPTED\", \"ENCRYPTED\"]"]
    pub fn set_allowed_encryption_statuses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_encryption_statuses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_admin_approval`.\nWhether the device needs to be approved by the customer admin."]
    pub fn set_require_admin_approval(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_admin_approval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_corp_owned`.\nWhether the device needs to be corp owned."]
    pub fn set_require_corp_owned(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_corp_owned = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_screen_lock`.\nWhether or not screenlock is required for the DevicePolicy\nto be true. Defaults to false."]
    pub fn set_require_screen_lock(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_screen_lock = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os_constraints`.\n"]
    pub fn set_os_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.os_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.os_constraints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {}

impl BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
            allowed_device_management_levels: core::default::Default::default(),
            allowed_encryption_statuses: core::default::Default::default(),
            require_admin_approval: core::default::Default::default(),
            require_corp_owned: core::default::Default::default(),
            require_screen_lock: core::default::Default::default(),
            os_constraints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_device_management_levels` after provisioning.\nA list of allowed device management levels.\nAn empty list allows all management levels. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"NONE\", \"BASIC\", \"COMPLETE\"]"]
    pub fn allowed_device_management_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_device_management_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_encryption_statuses` after provisioning.\nA list of allowed encryptions statuses.\nAn empty list allows all statuses. Possible values: [\"ENCRYPTION_UNSPECIFIED\", \"ENCRYPTION_UNSUPPORTED\", \"UNENCRYPTED\", \"ENCRYPTED\"]"]
    pub fn allowed_encryption_statuses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_encryption_statuses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_admin_approval` after provisioning.\nWhether the device needs to be approved by the customer admin."]
    pub fn require_admin_approval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_admin_approval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_corp_owned` after provisioning.\nWhether the device needs to be corp owned."]
    pub fn require_corp_owned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_corp_owned", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_screen_lock` after provisioning.\nWhether or not screenlock is required for the DevicePolicy\nto be true. Defaults to false."]
    pub fn require_screen_lock(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_screen_lock", self.base))
    }

    #[doc = "Get a reference to the value of field `os_constraints` after provisioning.\n"]
    pub fn os_constraints(
        &self,
    ) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_constraints", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_ip_subnetworks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    #[doc = "Set the field `vpc_ip_subnetworks`.\nCIDR block IP subnetwork specification. Must be IPv4."]
    pub fn set_vpc_ip_subnetworks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vpc_ip_subnetworks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    #[doc =
        "Required. Network name to be allowed by this Access Level. Networks of foreign organizations requires 'compute.network.get' permission to be granted to caller."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
            network: self.network,
            vpc_ip_subnetworks: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nRequired. Network name to be allowed by this Access Level. Networks of foreign organizations requires 'compute.network.get' permission to be granted to caller."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_ip_subnetworks` after provisioning.\nCIDR block IP subnetwork specification. Must be IPv4."]
    pub fn vpc_ip_subnetworks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_ip_subnetworks", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElDynamic {
    vpc_subnetwork: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_subnetwork: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl>>,
    dynamic: AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElDynamic,
}

impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    #[doc = "Set the field `vpc_subnetwork`.\n"]
    pub fn set_vpc_subnetwork(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_subnetwork = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_subnetwork = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {}

impl BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
            vpc_subnetwork: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_subnetwork` after provisioning.\n"]
    pub fn vpc_subnetwork(
        &self,
    ) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_subnetwork", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElConditionsElDynamic {
    device_policy: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>>,
    vpc_network_sources: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_subnetworks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    members: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required_access_levels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_policy: std::option::Option<std::vec::Vec<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_network_sources: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>>,
    dynamic: AccessContextManagerAccessLevelBasicElConditionsElDynamic,
}

impl AccessContextManagerAccessLevelBasicElConditionsEl {
    #[doc =
        "Set the field `ip_subnetworks`.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn set_ip_subnetworks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_subnetworks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `members`.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn set_members(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.members = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `negate`.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn set_negate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.negate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regions`.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required_access_levels`.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn set_required_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required_access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_policy`.\n"]
    pub fn set_device_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.device_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.device_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_network_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network_sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelBasicElConditionsEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelBasicElConditionsEl {}

impl BuildAccessContextManagerAccessLevelBasicElConditionsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsEl {
        AccessContextManagerAccessLevelBasicElConditionsEl {
            ip_subnetworks: core::default::Default::default(),
            members: core::default::Default::default(),
            negate: core::default::Default::default(),
            regions: core::default::Default::default(),
            required_access_levels: core::default::Default::default(),
            device_policy: core::default::Default::default(),
            vpc_network_sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelBasicElConditionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelBasicElConditionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElRef {
        AccessContextManagerAccessLevelBasicElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelBasicElConditionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_subnetworks` after provisioning.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn ip_subnetworks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_subnetworks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `members` after provisioning.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn members(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.members", self.base))
    }

    #[doc =
        "Get a reference to the value of field `negate` after provisioning.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn negate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regions` after provisioning.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required_access_levels` after provisioning.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn required_access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required_access_levels", self.base))
    }

    #[doc = "Get a reference to the value of field `device_policy` after provisioning.\n"]
    pub fn device_policy(&self) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(
        &self,
    ) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_network_sources", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElDynamic {
    conditions: std::option::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    combining_function: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<AccessContextManagerAccessLevelBasicElConditionsEl>>,
    dynamic: AccessContextManagerAccessLevelBasicElDynamic,
}

impl AccessContextManagerAccessLevelBasicEl {
    #[doc =
        "Set the field `combining_function`.\nHow the conditions list should be combined to determine if a request\nis granted this AccessLevel. If AND is used, each Condition in\nconditions must be satisfied for the AccessLevel to be applied. If\nOR is used, at least one Condition in conditions must be satisfied\nfor the AccessLevel to be applied. Default value: \"AND\" Possible values: [\"AND\", \"OR\"]"]
    pub fn set_combining_function(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.combining_function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelBasicEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelBasicEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelBasicEl {}

impl BuildAccessContextManagerAccessLevelBasicEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicEl {
        AccessContextManagerAccessLevelBasicEl {
            combining_function: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelBasicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelBasicElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessContextManagerAccessLevelBasicElRef {
        AccessContextManagerAccessLevelBasicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelBasicElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `combining_function` after provisioning.\nHow the conditions list should be combined to determine if a request\nis granted this AccessLevel. If AND is used, each Condition in\nconditions must be satisfied for the AccessLevel to be applied. If\nOR is used, at least one Condition in conditions must be satisfied\nfor the AccessLevel to be applied. Default value: \"AND\" Possible values: [\"AND\", \"OR\"]"]
    pub fn combining_function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.combining_function", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<AccessContextManagerAccessLevelBasicElConditionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelCustomElExprEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerAccessLevelCustomElExprEl {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelCustomElExprEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelCustomElExprEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelCustomElExprEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerAccessLevelCustomElExprEl {
    pub fn build(self) -> AccessContextManagerAccessLevelCustomElExprEl {
        AccessContextManagerAccessLevelCustomElExprEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelCustomElExprElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelCustomElExprElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAccessLevelCustomElExprElRef {
        AccessContextManagerAccessLevelCustomElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelCustomElExprElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelCustomElDynamic {
    expr: std::option::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelCustomElExprEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelCustomEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<std::vec::Vec<AccessContextManagerAccessLevelCustomElExprEl>>,
    dynamic: AccessContextManagerAccessLevelCustomElDynamic,
}

impl AccessContextManagerAccessLevelCustomEl {
    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerAccessLevelCustomElExprEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expr = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerAccessLevelCustomEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelCustomEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelCustomEl {}

impl BuildAccessContextManagerAccessLevelCustomEl {
    pub fn build(self) -> AccessContextManagerAccessLevelCustomEl {
        AccessContextManagerAccessLevelCustomEl {
            expr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelCustomElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelCustomElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessContextManagerAccessLevelCustomElRef {
        AccessContextManagerAccessLevelCustomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelCustomElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<AccessContextManagerAccessLevelCustomElExprElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerAccessLevelTimeoutsEl {
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

impl terrars::ToListMappable for AccessContextManagerAccessLevelTimeoutsEl {
    type O = terrars::BlockAssignable<AccessContextManagerAccessLevelTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAccessLevelTimeoutsEl {}

impl BuildAccessContextManagerAccessLevelTimeoutsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelTimeoutsEl {
        AccessContextManagerAccessLevelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerAccessLevelTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAccessLevelTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessContextManagerAccessLevelTimeoutsElRef {
        AccessContextManagerAccessLevelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAccessLevelTimeoutsElRef {
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
struct AccessContextManagerAccessLevelDynamic {
    basic: std::option::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelBasicEl>>,
    custom: std::option::Option<terrars::DynamicBlock<AccessContextManagerAccessLevelCustomEl>>,
}
