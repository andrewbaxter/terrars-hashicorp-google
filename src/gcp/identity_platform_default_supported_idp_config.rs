use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformDefaultSupportedIdpConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    idp_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl>,
}

struct IdentityPlatformDefaultSupportedIdpConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IdentityPlatformDefaultSupportedIdpConfigData>,
}

#[derive(Clone)]
pub struct IdentityPlatformDefaultSupportedIdpConfig(Rc<IdentityPlatformDefaultSupportedIdpConfig_>);

impl IdentityPlatformDefaultSupportedIdpConfig {
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

    #[doc= "Set the field `enabled`.\nIf this IDP allows the user to sign in"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\nOAuth client ID"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\nOAuth client secret"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf this IDP allows the user to sign in"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `idp_id` after provisioning.\nID of the IDP. Possible values include:\n\n* 'apple.com'\n\n* 'facebook.com'\n\n* 'gc.apple.com'\n\n* 'github.com'\n\n* 'google.com'\n\n* 'linkedin.com'\n\n* 'microsoft.com'\n\n* 'playgames.google.com'\n\n* 'twitter.com'\n\n* 'yahoo.com'"]
    pub fn idp_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idp_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the DefaultSupportedIdpConfig resource"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
        IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for IdentityPlatformDefaultSupportedIdpConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IdentityPlatformDefaultSupportedIdpConfig { }

impl ToListMappable for IdentityPlatformDefaultSupportedIdpConfig {
    type O = ListRef<IdentityPlatformDefaultSupportedIdpConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IdentityPlatformDefaultSupportedIdpConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_identity_platform_default_supported_idp_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentityPlatformDefaultSupportedIdpConfig {
    pub tf_id: String,
    #[doc= "OAuth client ID"]
    pub client_id: PrimField<String>,
    #[doc= "OAuth client secret"]
    pub client_secret: PrimField<String>,
    #[doc= "ID of the IDP. Possible values include:\n\n* 'apple.com'\n\n* 'facebook.com'\n\n* 'gc.apple.com'\n\n* 'github.com'\n\n* 'google.com'\n\n* 'linkedin.com'\n\n* 'microsoft.com'\n\n* 'playgames.google.com'\n\n* 'twitter.com'\n\n* 'yahoo.com'"]
    pub idp_id: PrimField<String>,
}

impl BuildIdentityPlatformDefaultSupportedIdpConfig {
    pub fn build(self, stack: &mut Stack) -> IdentityPlatformDefaultSupportedIdpConfig {
        let out = IdentityPlatformDefaultSupportedIdpConfig(Rc::new(IdentityPlatformDefaultSupportedIdpConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IdentityPlatformDefaultSupportedIdpConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_id: self.client_id,
                client_secret: self.client_secret,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                idp_id: self.idp_id,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentityPlatformDefaultSupportedIdpConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformDefaultSupportedIdpConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentityPlatformDefaultSupportedIdpConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\nOAuth client ID"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `client_secret` after provisioning.\nOAuth client secret"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nIf this IDP allows the user to sign in"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `idp_id` after provisioning.\nID of the IDP. Possible values include:\n\n* 'apple.com'\n\n* 'facebook.com'\n\n* 'gc.apple.com'\n\n* 'github.com'\n\n* 'google.com'\n\n* 'linkedin.com'\n\n* 'microsoft.com'\n\n* 'playgames.google.com'\n\n* 'twitter.com'\n\n* 'yahoo.com'"]
    pub fn idp_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idp_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the DefaultSupportedIdpConfig resource"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
        IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {
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

impl ToListMappable for IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {
    type O = BlockAssignable<IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {}

impl BuildIdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {
        IdentityPlatformDefaultSupportedIdpConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
        IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformDefaultSupportedIdpConfigTimeoutsElRef {
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
