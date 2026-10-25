#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformInboundSamlConfigData_ {
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
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_config: std::option::Option<std::vec::Vec<IdentityPlatformInboundSamlConfig_IdpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sp_config: std::option::Option<std::vec::Vec<IdentityPlatformInboundSamlConfig_SpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IdentityPlatformInboundSamlConfig_Timeouts>,
    dynamic: IdentityPlatformInboundSamlConfigDynamic,
}

struct IdentityPlatformInboundSamlConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IdentityPlatformInboundSamlConfigData_>,
}

#[derive(Clone)]
pub struct IdentityPlatformInboundSamlConfig(std::rc::Rc<IdentityPlatformInboundSamlConfig_>);

impl IdentityPlatformInboundSamlConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_IdpConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_SpConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<IdentityPlatformInboundSamlConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> terrars::ListRef<IdentityPlatformInboundSamlConfig_IdpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sp_config` after provisioning.\n"]
    pub fn sp_config(&self) -> terrars::ListRef<IdentityPlatformInboundSamlConfig_SpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformInboundSamlConfig_TimeoutsRef {
        IdentityPlatformInboundSamlConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IdentityPlatformInboundSamlConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IdentityPlatformInboundSamlConfig { }

impl terrars::ToListMappable for IdentityPlatformInboundSamlConfig {
    type O = terrars::ListRef<IdentityPlatformInboundSamlConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IdentityPlatformInboundSamlConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_identity_platform_inbound_saml_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentityPlatformInboundSamlConfig {
    pub tf_id: std::string::String,
    #[doc = "Human friendly display name."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the InboundSamlConfig resource. Must start with 'saml.' and can only have alphanumeric characters,\nhyphens, underscores or periods. The part after 'saml.' must also start with a lowercase letter, end with an\nalphanumeric character, and have at least 2 characters."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformInboundSamlConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> IdentityPlatformInboundSamlConfig {
        let out = IdentityPlatformInboundSamlConfig(std::rc::Rc::new(IdentityPlatformInboundSamlConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IdentityPlatformInboundSamlConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
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

pub struct IdentityPlatformInboundSamlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformInboundSamlConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentityPlatformInboundSamlConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> terrars::ListRef<IdentityPlatformInboundSamlConfig_IdpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sp_config` after provisioning.\n"]
    pub fn sp_config(&self) -> terrars::ListRef<IdentityPlatformInboundSamlConfig_SpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformInboundSamlConfig_TimeoutsRef {
        IdentityPlatformInboundSamlConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformInboundSamlConfigDynamic {
    idp_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformInboundSamlConfig_IdpConfig>>,
    sp_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformInboundSamlConfig_SpConfig>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {
    #[doc = "Set the field `x509_certificate`.\nThe IdP's x509 certificate."]
    pub fn set_x509_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.x509_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {
    type O = terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {}

impl BuildIdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {
    pub fn build(self) -> IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {
        IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates {
            x509_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificatesRef {
        IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `x509_certificate` after provisioning.\nThe IdP's x509 certificate."]
    pub fn x509_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x509_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformInboundSamlConfig_IdpConfigDynamic {
    idp_certificates: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformInboundSamlConfig_IdpConfig {
    idp_entity_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_request: std::option::Option<terrars::PrimField<bool>>,
    sso_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp_certificates: std::option::Option<std::vec::Vec<IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates>>,
    dynamic: IdentityPlatformInboundSamlConfig_IdpConfigDynamic,
}

impl IdentityPlatformInboundSamlConfig_IdpConfig {
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
                    ::Into<terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificates>>,
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

impl terrars::ToListMappable for IdentityPlatformInboundSamlConfig_IdpConfig {
    type O = terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_IdpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformInboundSamlConfig_IdpConfig {
    #[doc = "Unique identifier for all SAML entities"]
    pub idp_entity_id: terrars::PrimField<std::string::String>,
    #[doc = "URL to send Authentication request to."]
    pub sso_url: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformInboundSamlConfig_IdpConfig {
    pub fn build(self) -> IdentityPlatformInboundSamlConfig_IdpConfig {
        IdentityPlatformInboundSamlConfig_IdpConfig {
            idp_entity_id: self.idp_entity_id,
            sign_request: core::default::Default::default(),
            sso_url: self.sso_url,
            idp_certificates: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformInboundSamlConfig_IdpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformInboundSamlConfig_IdpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformInboundSamlConfig_IdpConfigRef {
        IdentityPlatformInboundSamlConfig_IdpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformInboundSamlConfig_IdpConfigRef {
    #[allow(dead_code)]
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
    pub fn idp_certificates(&self) -> terrars::ListRef<IdentityPlatformInboundSamlConfig_IdpConfig_IdpCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {
    #[doc = "Set the field `x509_certificate`.\n"]
    pub fn set_x509_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.x509_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {
    type O = terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_SpConfig_SpCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {}

impl BuildIdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {
    pub fn build(self) -> IdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {
        IdentityPlatformInboundSamlConfig_SpConfig_SpCertificates {
            x509_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformInboundSamlConfig_SpConfig_SpCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformInboundSamlConfig_SpConfig_SpCertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformInboundSamlConfig_SpConfig_SpCertificatesRef {
        IdentityPlatformInboundSamlConfig_SpConfig_SpCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformInboundSamlConfig_SpConfig_SpCertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `x509_certificate` after provisioning.\n"]
    pub fn x509_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x509_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformInboundSamlConfig_SpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    callback_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sp_entity_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformInboundSamlConfig_SpConfig {
    #[doc =
        "Set the field `callback_uri`.\nCallback URI where responses from IDP are handled. Must start with 'https://'."]
    pub fn set_callback_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.callback_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sp_entity_id`.\nUnique identifier for all SAML entities."]
    pub fn set_sp_entity_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sp_entity_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformInboundSamlConfig_SpConfig {
    type O = terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_SpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformInboundSamlConfig_SpConfig {}

impl BuildIdentityPlatformInboundSamlConfig_SpConfig {
    pub fn build(self) -> IdentityPlatformInboundSamlConfig_SpConfig {
        IdentityPlatformInboundSamlConfig_SpConfig {
            callback_uri: core::default::Default::default(),
            sp_entity_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformInboundSamlConfig_SpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformInboundSamlConfig_SpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformInboundSamlConfig_SpConfigRef {
        IdentityPlatformInboundSamlConfig_SpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformInboundSamlConfig_SpConfigRef {
    #[allow(dead_code)]
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
    pub fn sp_certificates(&self) -> terrars::ListRef<IdentityPlatformInboundSamlConfig_SpConfig_SpCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sp_certificates", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sp_entity_id` after provisioning.\nUnique identifier for all SAML entities."]
    pub fn sp_entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sp_entity_id", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformInboundSamlConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformInboundSamlConfig_Timeouts {
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

impl terrars::ToListMappable for IdentityPlatformInboundSamlConfig_Timeouts {
    type O = terrars::BlockAssignable<IdentityPlatformInboundSamlConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformInboundSamlConfig_Timeouts {}

impl BuildIdentityPlatformInboundSamlConfig_Timeouts {
    pub fn build(self) -> IdentityPlatformInboundSamlConfig_Timeouts {
        IdentityPlatformInboundSamlConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformInboundSamlConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformInboundSamlConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformInboundSamlConfig_TimeoutsRef {
        IdentityPlatformInboundSamlConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformInboundSamlConfig_TimeoutsRef {
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
