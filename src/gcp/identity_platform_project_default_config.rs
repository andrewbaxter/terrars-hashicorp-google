use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformProjectDefaultConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in: Option<Vec<IdentityPlatformProjectDefaultConfigSignInEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IdentityPlatformProjectDefaultConfigTimeoutsEl>,
    dynamic: IdentityPlatformProjectDefaultConfigDynamic,
}

struct IdentityPlatformProjectDefaultConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentityPlatformProjectDefaultConfigData>,
}

#[derive(Clone)]
pub struct IdentityPlatformProjectDefaultConfig(Rc<IdentityPlatformProjectDefaultConfig_>);

impl IdentityPlatformProjectDefaultConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `sign_in`.\n"]
    pub fn set_sign_in(self, v: impl Into<BlockAssignable<IdentityPlatformProjectDefaultConfigSignInEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sign_in = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sign_in = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IdentityPlatformProjectDefaultConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource. Example: \"projects/my-awesome-project/config\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> ListRef<IdentityPlatformProjectDefaultConfigSignInElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformProjectDefaultConfigTimeoutsElRef {
        IdentityPlatformProjectDefaultConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for IdentityPlatformProjectDefaultConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IdentityPlatformProjectDefaultConfig { }

impl ToListMappable for IdentityPlatformProjectDefaultConfig {
    type O = ListRef<IdentityPlatformProjectDefaultConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IdentityPlatformProjectDefaultConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_identity_platform_project_default_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfig {
    pub tf_id: String,
}

impl BuildIdentityPlatformProjectDefaultConfig {
    pub fn build(self, stack: &mut Stack) -> IdentityPlatformProjectDefaultConfig {
        let out = IdentityPlatformProjectDefaultConfig(Rc::new(IdentityPlatformProjectDefaultConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdentityPlatformProjectDefaultConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                sign_in: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentityPlatformProjectDefaultConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentityPlatformProjectDefaultConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource. Example: \"projects/my-awesome-project/config\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> ListRef<IdentityPlatformProjectDefaultConfigSignInElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformProjectDefaultConfigTimeoutsElRef {
        IdentityPlatformProjectDefaultConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_cost: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rounds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salt_separator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signer_key: Option<PrimField<String>>,
}

impl IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    #[doc= "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_cost`.\n"]
    pub fn set_memory_cost(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_cost = Some(v.into());
        self
    }

    #[doc= "Set the field `rounds`.\n"]
    pub fn set_rounds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rounds = Some(v.into());
        self
    }

    #[doc= "Set the field `salt_separator`.\n"]
    pub fn set_salt_separator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.salt_separator = Some(v.into());
        self
    }

    #[doc= "Set the field `signer_key`.\n"]
    pub fn set_signer_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.signer_key = Some(v.into());
        self
    }
}

impl ToListMappable for IdentityPlatformProjectDefaultConfigSignInElHashConfigEl {
    type O = BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElHashConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
        IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `memory_cost` after provisioning.\n"]
    pub fn memory_cost(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_cost", self.base))
    }

    #[doc= "Get a reference to the value of field `rounds` after provisioning.\n"]
    pub fn rounds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rounds", self.base))
    }

    #[doc= "Get a reference to the value of field `salt_separator` after provisioning.\n"]
    pub fn salt_separator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.salt_separator", self.base))
    }

    #[doc= "Get a reference to the value of field `signer_key` after provisioning.\n"]
    pub fn signer_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signer_key", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    enabled: PrimField<bool>,
}

impl IdentityPlatformProjectDefaultConfigSignInElAnonymousEl { }

impl ToListMappable for IdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    type O = BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    #[doc= "Whether anonymous user auth is enabled for the project or not."]
    pub enabled: PrimField<bool>,
}

impl BuildIdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
    pub fn build(self) -> IdentityPlatformProjectDefaultConfigSignInElAnonymousEl {
        IdentityPlatformProjectDefaultConfigSignInElAnonymousEl { enabled: self.enabled }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
        IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether anonymous user auth is enabled for the project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElEmailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_required: Option<PrimField<bool>>,
}

impl IdentityPlatformProjectDefaultConfigSignInElEmailEl {
    #[doc= "Set the field `enabled`.\nWhether email auth is enabled for the project or not."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `password_required`.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn set_password_required(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.password_required = Some(v.into());
        self
    }
}

impl ToListMappable for IdentityPlatformProjectDefaultConfigSignInElEmailEl {
    type O = BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElEmailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
        IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElEmailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether email auth is enabled for the project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `password_required` after provisioning.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn password_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.password_required", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_phone_numbers: Option<RecField<PrimField<String>>>,
}

impl IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    #[doc= "Set the field `enabled`.\nWhether phone number auth is enabled for the project or not."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `test_phone_numbers`.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn set_test_phone_numbers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.test_phone_numbers = Some(v.into());
        self
    }
}

impl ToListMappable for IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl {
    type O = BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
        IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether phone number auth is enabled for the project or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `test_phone_numbers` after provisioning.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn test_phone_numbers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.test_phone_numbers", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformProjectDefaultConfigSignInElDynamic {
    anonymous: Option<DynamicBlock<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>>,
    email: Option<DynamicBlock<IdentityPlatformProjectDefaultConfigSignInElEmailEl>>,
    phone_number: Option<DynamicBlock<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigSignInEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_duplicate_emails: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous: Option<Vec<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Vec<IdentityPlatformProjectDefaultConfigSignInElEmailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<Vec<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>>,
    dynamic: IdentityPlatformProjectDefaultConfigSignInElDynamic,
}

impl IdentityPlatformProjectDefaultConfigSignInEl {
    #[doc= "Set the field `allow_duplicate_emails`.\nWhether to allow more than one account to have the same email."]
    pub fn set_allow_duplicate_emails(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_duplicate_emails = Some(v.into());
        self
    }

    #[doc= "Set the field `anonymous`.\n"]
    pub fn set_anonymous(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElAnonymousEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.anonymous = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.anonymous = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElEmailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.email = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.email = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `phone_number`.\n"]
    pub fn set_phone_number(
        mut self,
        v: impl Into<BlockAssignable<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.phone_number = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_number = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for IdentityPlatformProjectDefaultConfigSignInEl {
    type O = BlockAssignable<IdentityPlatformProjectDefaultConfigSignInEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct IdentityPlatformProjectDefaultConfigSignInElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigSignInElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformProjectDefaultConfigSignInElRef {
        IdentityPlatformProjectDefaultConfigSignInElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigSignInElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_duplicate_emails` after provisioning.\nWhether to allow more than one account to have the same email."]
    pub fn allow_duplicate_emails(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_duplicate_emails", self.base))
    }

    #[doc= "Get a reference to the value of field `hash_config` after provisioning.\nOutput only. Hash config information."]
    pub fn hash_config(&self) -> ListRef<IdentityPlatformProjectDefaultConfigSignInElHashConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hash_config", self.base))
    }

    #[doc= "Get a reference to the value of field `anonymous` after provisioning.\n"]
    pub fn anonymous(&self) -> ListRef<IdentityPlatformProjectDefaultConfigSignInElAnonymousElRef> {
        ListRef::new(self.shared().clone(), format!("{}.anonymous", self.base))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> ListRef<IdentityPlatformProjectDefaultConfigSignInElEmailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> ListRef<IdentityPlatformProjectDefaultConfigSignInElPhoneNumberElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformProjectDefaultConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl IdentityPlatformProjectDefaultConfigTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for IdentityPlatformProjectDefaultConfigTimeoutsEl {
    type O = BlockAssignable<IdentityPlatformProjectDefaultConfigTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformProjectDefaultConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformProjectDefaultConfigTimeoutsElRef {
        IdentityPlatformProjectDefaultConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformProjectDefaultConfigTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformProjectDefaultConfigDynamic {
    sign_in: Option<DynamicBlock<IdentityPlatformProjectDefaultConfigSignInEl>>,
}
