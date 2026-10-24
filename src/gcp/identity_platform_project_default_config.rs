use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformProjectDefaultConfigData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_in: std::option::Option<std::vec::Vec<IdentityPlatformProjectDefaultConfigSignInEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IdentityPlatformProjectDefaultConfigTimeoutsEl>,
    dynamic: IdentityPlatformProjectDefaultConfigDynamic,
}

struct IdentityPlatformProjectDefaultConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IdentityPlatformProjectDefaultConfigData>,
}

#[derive(Clone)]
pub struct IdentityPlatformProjectDefaultConfig(std::rc::Rc<IdentityPlatformProjectDefaultConfig_>);

impl IdentityPlatformProjectDefaultConfig {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sign_in`.\n"]
    pub fn set_sign_in(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sign_in = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sign_in = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IdentityPlatformProjectDefaultConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource. Example: \"projects/my-awesome-project/config\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> terrars::ListRef<IdentityPlatformProjectDefaultConfigSignInElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformProjectDefaultConfigTimeoutsElRef {
        IdentityPlatformProjectDefaultConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IdentityPlatformProjectDefaultConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IdentityPlatformProjectDefaultConfig { }

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfig {
    type O = terrars::ListRef<IdentityPlatformProjectDefaultConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IdentityPlatformProjectDefaultConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_identity_platform_project_default_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfig {
    pub tf_id: std::string::String,
}

impl BuildIdentityPlatformProjectDefaultConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> IdentityPlatformProjectDefaultConfig {
        let out = IdentityPlatformProjectDefaultConfig(std::rc::Rc::new(IdentityPlatformProjectDefaultConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IdentityPlatformProjectDefaultConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                sign_in: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentityPlatformProjectDefaultConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentityPlatformProjectDefaultConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource. Example: \"projects/my-awesome-project/config\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> terrars::ListRef<IdentityPlatformProjectDefaultConfigSignInElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformProjectDefaultConfigTimeoutsElRef {
        IdentityPlatformProjectDefaultConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_cost: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rounds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salt_separator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signer_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_cost`.\n"]
    pub fn set_memory_cost(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_cost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rounds`.\n"]
    pub fn set_rounds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rounds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `salt_separator`.\n"]
    pub fn set_salt_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.salt_separator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signer_key`.\n"]
    pub fn set_signer_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signer_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    type O = terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElHashConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigSignInElHashConfigEl {}

impl BuildIdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
        IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
            algorithm: core::default::Default::default(),
            memory_cost: core::default::Default::default(),
            rounds: core::default::Default::default(),
            salt_separator: core::default::Default::default(),
            signer_key: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
        IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_cost` after provisioning.\n"]
    pub fn memory_cost(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_cost", self.base))
    }

    #[doc = "Get a reference to the value of field `rounds` after provisioning.\n"]
    pub fn rounds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rounds", self.base))
    }

    #[doc = "Get a reference to the value of field `salt_separator` after provisioning.\n"]
    pub fn salt_separator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.salt_separator", self.base))
    }

    #[doc = "Get a reference to the value of field `signer_key` after provisioning.\n"]
    pub fn signer_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signer_key", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    enabled: terrars::PrimField<bool>,
}

impl IdentityPlatformProjectDefaultConfigSignInElAnonymousEl { }

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    type O = terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    #[doc = "Whether anonymous user auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
        IdentityPlatformProjectDefaultConfigSignInElAnonymousEl { enabled: self.enabled }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
        IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether anonymous user auth is enabled for the project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElEmailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_required: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformProjectDefaultConfigSignInElEmailEl {
    #[doc = "Set the field `enabled`.\nWhether email auth is enabled for the project or not."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_required`.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn set_password_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.password_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfigSignInElEmailEl {
    type O = terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElEmailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigSignInElEmailEl {}

impl BuildIdentityPlatformProjectDefaultConfigSignInElEmailEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigSignInElEmailEl {
        IdentityPlatformProjectDefaultConfigSignInElEmailEl {
            enabled: core::default::Default::default(),
            password_required: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
        IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether email auth is enabled for the project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_required` after provisioning.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn password_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_required", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_phone_numbers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    #[doc = "Set the field `enabled`.\nWhether phone number auth is enabled for the project or not."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `test_phone_numbers`.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn set_test_phone_numbers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.test_phone_numbers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    type O = terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {}

impl BuildIdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
        IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
            enabled: core::default::Default::default(),
            test_phone_numbers: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
        IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether phone number auth is enabled for the project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `test_phone_numbers` after provisioning.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn test_phone_numbers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.test_phone_numbers", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformProjectDefaultConfigSignInElDynamic {
    anonymous: std::option::Option<terrars::DynamicBlock<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>>,
    email: std::option::Option<terrars::DynamicBlock<IdentityPlatformProjectDefaultConfigSignInElEmailEl>>,
    phone_number: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_duplicate_emails: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonymous: std::option::Option<std::vec::Vec<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<std::vec::Vec<IdentityPlatformProjectDefaultConfigSignInElEmailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<std::vec::Vec<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>>,
    dynamic: IdentityPlatformProjectDefaultConfigSignInElDynamic,
}

impl IdentityPlatformProjectDefaultConfigSignInEl {
    #[doc = "Set the field `allow_duplicate_emails`.\nWhether to allow more than one account to have the same email."]
    pub fn set_allow_duplicate_emails(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_duplicate_emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anonymous`.\n"]
    pub fn set_anonymous(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.anonymous = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.anonymous = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElEmailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.email = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.email = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phone_number = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_number = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfigSignInEl {
    type O = terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigSignInEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigSignInEl {}

impl BuildIdentityPlatformProjectDefaultConfigSignInEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigSignInEl {
        IdentityPlatformProjectDefaultConfigSignInEl {
            allow_duplicate_emails: core::default::Default::default(),
            anonymous: core::default::Default::default(),
            email: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigSignInElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformProjectDefaultConfigSignInElRef {
        IdentityPlatformProjectDefaultConfigSignInElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_duplicate_emails` after provisioning.\nWhether to allow more than one account to have the same email."]
    pub fn allow_duplicate_emails(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_duplicate_emails", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hash_config` after provisioning.\nOutput only. Hash config information."]
    pub fn hash_config(&self) -> terrars::ListRef<IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hash_config", self.base))
    }

    #[doc = "Get a reference to the value of field `anonymous` after provisioning.\n"]
    pub fn anonymous(&self) -> terrars::ListRef<IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anonymous", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<IdentityPlatformProjectDefaultConfigSignInElEmailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::ListRef<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformProjectDefaultConfigTimeoutsEl {
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

impl terrars::ToListMappable for IdentityPlatformProjectDefaultConfigTimeoutsEl {
    type O = terrars::BlockAssignable<IdentityPlatformProjectDefaultConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigTimeoutsEl {}

impl BuildIdentityPlatformProjectDefaultConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigTimeoutsEl {
        IdentityPlatformProjectDefaultConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformProjectDefaultConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformProjectDefaultConfigTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformProjectDefaultConfigTimeoutsElRef {
        IdentityPlatformProjectDefaultConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigTimeoutsElRef {
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
struct IdentityPlatformProjectDefaultConfigDynamic {
    sign_in: std::option::Option<terrars::DynamicBlock<IdentityPlatformProjectDefaultConfigSignInEl>>,
}
