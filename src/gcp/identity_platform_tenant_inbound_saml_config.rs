use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformTenantInboundSamlConfigData {
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
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    tenant: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_config: std::option::Option<std::vec::Vec<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sp_config: std::option::Option<std::vec::Vec<IdentityPlatformTenantInboundSamlConfigSpConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IdentityPlatformTenantInboundSamlConfigTimeoutsEl>,
    dynamic: IdentityPlatformTenantInboundSamlConfigDynamic,
}

struct IdentityPlatformTenantInboundSamlConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IdentityPlatformTenantInboundSamlConfigData>,
}

#[derive(Clone)]
pub struct IdentityPlatformTenantInboundSamlConfig(std::rc::Rc<IdentityPlatformTenantInboundSamlConfig_>);

impl IdentityPlatformTenantInboundSamlConfig {
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

    #[doc = "Set the field `enabled`.\nIf this config allows users to sign in with the provider."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `idp_config`.\n"]
    pub fn set_idp_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().idp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.idp_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sp_config`.\n"]
    pub fn set_sp_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigSpConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sp_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IdentityPlatformTenantInboundSamlConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf this config allows users to sign in with the provider."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nThe name of the tenant where this inbound SAML config resource exists"]
    pub fn tenant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> terrars::ListRef<IdentityPlatformTenantInboundSamlConfigIdpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sp_config` after provisioning.\n"]
    pub fn sp_config(&self) -> terrars::ListRef<IdentityPlatformTenantInboundSamlConfigSpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
        IdentityPlatformTenantInboundSamlConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IdentityPlatformTenantInboundSamlConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IdentityPlatformTenantInboundSamlConfig { }

impl terrars::ToListMappable for IdentityPlatformTenantInboundSamlConfig {
    type O = terrars::ListRef<IdentityPlatformTenantInboundSamlConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IdentityPlatformTenantInboundSamlConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_identity_platform_tenant_inbound_saml_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentityPlatformTenantInboundSamlConfig {
    pub tf_id: std::string::String,
    #[doc = "Human friendly display name."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the tenant where this inbound SAML config resource exists"]
    pub tenant: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformTenantInboundSamlConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> IdentityPlatformTenantInboundSamlConfig {
        let out = IdentityPlatformTenantInboundSamlConfig(std::rc::Rc::new(IdentityPlatformTenantInboundSamlConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IdentityPlatformTenantInboundSamlConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                tenant: self.tenant,
                idp_config: core::default::Default::default(),
                sp_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentityPlatformTenantInboundSamlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformTenantInboundSamlConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentityPlatformTenantInboundSamlConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman friendly display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIf this config allows users to sign in with the provider."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant` after provisioning.\nThe name of the tenant where this inbound SAML config resource exists"]
    pub fn tenant(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> terrars::ListRef<IdentityPlatformTenantInboundSamlConfigIdpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sp_config` after provisioning.\n"]
    pub fn sp_config(&self) -> terrars::ListRef<IdentityPlatformTenantInboundSamlConfigSpConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
        IdentityPlatformTenantInboundSamlConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    #[doc = "Set the field `x509_certificate`.\nThe x509 certificate"]
    pub fn set_x509_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.x509_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    type O = terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {}

impl BuildIdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
        IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl {
            x509_certificate: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
        IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `x509_certificate` after provisioning.\nThe x509 certificate"]
    pub fn x509_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x509_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformTenantInboundSamlConfigIdpConfigElDynamic {
    idp_certificates: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    idp_entity_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_request: std::option::Option<terrars::PrimField<bool>>,
    sso_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_certificates: std
    ::option
    ::Option<std::vec::Vec<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>>,
    dynamic: IdentityPlatformTenantInboundSamlConfigIdpConfigElDynamic,
}

impl IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    #[doc = "Set the field `sign_request`.\nIndicates if outbounding SAMLRequest should be signed."]
    pub fn set_sign_request(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sign_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idp_certificates`.\n"]
    pub fn set_idp_certificates(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idp_certificates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idp_certificates = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    type O = terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    #[doc = "Unique identifier for all SAML entities"]
    pub idp_entity_id: terrars::PrimField<std::string::String>,
    #[doc = "URL to send Authentication request to."]
    pub sso_url: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformTenantInboundSamlConfigIdpConfigEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
        IdentityPlatformTenantInboundSamlConfigIdpConfigEl {
            idp_entity_id: self.idp_entity_id,
            sign_request: core::default::Default::default(),
            sso_url: self.sso_url,
            idp_certificates: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
        IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformTenantInboundSamlConfigIdpConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `idp_entity_id` after provisioning.\nUnique identifier for all SAML entities"]
    pub fn idp_entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idp_entity_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sign_request` after provisioning.\nIndicates if outbounding SAMLRequest should be signed."]
    pub fn sign_request(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sign_request", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sso_url` after provisioning.\nURL to send Authentication request to."]
    pub fn sso_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sso_url", self.base))
    }

    #[doc = "Get a reference to the value of field `idp_certificates` after provisioning.\n"]
    pub fn idp_certificates(
        &self,
    ) -> terrars::ListRef<IdentityPlatformTenantInboundSamlConfigIdpConfigElIdpCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    #[doc = "Set the field `x509_certificate`.\n"]
    pub fn set_x509_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.x509_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    type O = terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {}

impl BuildIdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
        IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesEl {
            x509_certificate: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
        IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `x509_certificate` after provisioning.\n"]
    pub fn x509_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x509_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigSpConfigEl {
    callback_uri: terrars::PrimField<std::string::String>,
    sp_entity_id: terrars::PrimField<std::string::String>,
}

impl IdentityPlatformTenantInboundSamlConfigSpConfigEl { }

impl terrars::ToListMappable for IdentityPlatformTenantInboundSamlConfigSpConfigEl {
    type O = terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigSpConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformTenantInboundSamlConfigSpConfigEl {
    #[doc = "Callback URI where responses from IDP are handled. Must start with 'https://'."]
    pub callback_uri: terrars::PrimField<std::string::String>,
    #[doc = "Unique identifier for all SAML entities."]
    pub sp_entity_id: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformTenantInboundSamlConfigSpConfigEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigSpConfigEl {
        IdentityPlatformTenantInboundSamlConfigSpConfigEl {
            callback_uri: self.callback_uri,
            sp_entity_id: self.sp_entity_id,
        }
    }
}

pub struct IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
        IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformTenantInboundSamlConfigSpConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `callback_uri` after provisioning.\nCallback URI where responses from IDP are handled. Must start with 'https://'."]
    pub fn callback_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.callback_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sp_certificates` after provisioning.\nThe IDP's certificate data to verify the signature in the SAMLResponse issued by the IDP."]
    pub fn sp_certificates(
        &self,
    ) -> terrars::ListRef<IdentityPlatformTenantInboundSamlConfigSpConfigElSpCertificatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sp_certificates", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sp_entity_id` after provisioning.\nUnique identifier for all SAML entities."]
    pub fn sp_entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sp_entity_id", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
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

impl terrars::ToListMappable for IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
    type O = terrars::BlockAssignable<IdentityPlatformTenantInboundSamlConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformTenantInboundSamlConfigTimeoutsEl {}

impl BuildIdentityPlatformTenantInboundSamlConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
        IdentityPlatformTenantInboundSamlConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
        IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformTenantInboundSamlConfigTimeoutsElRef {
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
struct IdentityPlatformTenantInboundSamlConfigDynamic {
    idp_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformTenantInboundSamlConfigIdpConfigEl>>,
    sp_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformTenantInboundSamlConfigSpConfigEl>>,
}
