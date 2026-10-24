use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeKeystoresAliasesSelfSignedCertData {
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
    alias: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_validity_in_days: std::option::Option<terrars::PrimField<f64>>,
    environment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_size: std::option::Option<terrars::PrimField<std::string::String>>,
    keystore: terrars::PrimField<std::string::String>,
    org_id: terrars::PrimField<std::string::String>,
    sig_alg: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<std::vec::Vec<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_dns_names: std
    ::option
    ::Option<std::vec::Vec<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl>,
    dynamic: ApigeeKeystoresAliasesSelfSignedCertDynamic,
}

struct ApigeeKeystoresAliasesSelfSignedCert_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeKeystoresAliasesSelfSignedCertData>,
}

#[derive(Clone)]
pub struct ApigeeKeystoresAliasesSelfSignedCert(std::rc::Rc<ApigeeKeystoresAliasesSelfSignedCert_>);

impl ApigeeKeystoresAliasesSelfSignedCert {
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
        "Set the field `cert_validity_in_days`.\nValidity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365."]
    pub fn set_cert_validity_in_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cert_validity_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_size`.\nKey size. Default and maximum value is 2048 bits."]
    pub fn set_key_size(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subject = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subject = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subject_alternative_dns_names`.\n"]
    pub fn set_subject_alternative_dns_names(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subject_alternative_dns_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subject_alternative_dns_names = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nAlias for the key/certificate pair. Values must match the regular expression [\\w\\s-.]{1,255}.\nThis must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either\nthis parameter or the JSON body."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cert_validity_in_days` after provisioning.\nValidity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365."]
    pub fn cert_validity_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_validity_in_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certs_info` after provisioning.\nChain of certificates under this alias."]
    pub fn certs_info(&self) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certs_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe Apigee environment name"]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_size` after provisioning.\nKey size. Default and maximum value is 2048 bits."]
    pub fn key_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keystore` after provisioning.\nThe Apigee keystore name associated in an Apigee environment"]
    pub fn keystore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee environment"]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sig_alg` after provisioning.\nSignature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA"]
    pub fn sig_alg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sig_alg", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subject_alternative_dns_names` after provisioning.\n"]
    pub fn subject_alternative_dns_names(
        &self,
    ) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_dns_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ApigeeKeystoresAliasesSelfSignedCert {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeKeystoresAliasesSelfSignedCert { }

impl terrars::ToListMappable for ApigeeKeystoresAliasesSelfSignedCert {
    type O = terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeKeystoresAliasesSelfSignedCert_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_keystores_aliases_self_signed_cert".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeKeystoresAliasesSelfSignedCert {
    pub tf_id: std::string::String,
    #[doc =
        "Alias for the key/certificate pair. Values must match the regular expression [\\w\\s-.]{1,255}.\nThis must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either\nthis parameter or the JSON body."]
    pub alias: terrars::PrimField<std::string::String>,
    #[doc = "The Apigee environment name"]
    pub environment: terrars::PrimField<std::string::String>,
    #[doc = "The Apigee keystore name associated in an Apigee environment"]
    pub keystore: terrars::PrimField<std::string::String>,
    #[doc = "The Apigee Organization name associated with the Apigee environment"]
    pub org_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Signature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA"]
    pub sig_alg: terrars::PrimField<std::string::String>,
}

impl BuildApigeeKeystoresAliasesSelfSignedCert {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeKeystoresAliasesSelfSignedCert {
        let out = ApigeeKeystoresAliasesSelfSignedCert(std::rc::Rc::new(ApigeeKeystoresAliasesSelfSignedCert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeKeystoresAliasesSelfSignedCertData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: self.alias,
                cert_validity_in_days: core::default::Default::default(),
                environment: self.environment,
                id: core::default::Default::default(),
                key_size: core::default::Default::default(),
                keystore: self.keystore,
                org_id: self.org_id,
                sig_alg: self.sig_alg,
                subject: core::default::Default::default(),
                subject_alternative_dns_names: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeKeystoresAliasesSelfSignedCertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesSelfSignedCertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeKeystoresAliasesSelfSignedCertRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nAlias for the key/certificate pair. Values must match the regular expression [\\w\\s-.]{1,255}.\nThis must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either\nthis parameter or the JSON body."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cert_validity_in_days` after provisioning.\nValidity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365."]
    pub fn cert_validity_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_validity_in_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certs_info` after provisioning.\nChain of certificates under this alias."]
    pub fn certs_info(&self) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certs_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\nThe Apigee environment name"]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_size` after provisioning.\nKey size. Default and maximum value is 2048 bits."]
    pub fn key_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keystore` after provisioning.\nThe Apigee keystore name associated in an Apigee environment"]
    pub fn keystore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee environment"]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sig_alg` after provisioning.\nSignature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA"]
    pub fn sig_alg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sig_alg", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subject_alternative_dns_names` after provisioning.\n"]
    pub fn subject_alternative_dns_names(
        &self,
    ) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_dns_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_constraints: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiry_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_valid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serial_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sig_alg_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<f64>>,
}

impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    #[doc = "Set the field `basic_constraints`.\n"]
    pub fn set_basic_constraints(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.basic_constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiry_date`.\n"]
    pub fn set_expiry_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiry_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_valid`.\n"]
    pub fn set_is_valid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.is_valid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serial_number`.\n"]
    pub fn set_serial_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serial_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sig_alg_name`.\n"]
    pub fn set_sig_alg_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sig_alg_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subject_alternative_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_from`.\n"]
    pub fn set_valid_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.valid_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {}

impl BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl {
            basic_constraints: core::default::Default::default(),
            expiry_date: core::default::Default::default(),
            is_valid: core::default::Default::default(),
            issuer: core::default::Default::default(),
            public_key: core::default::Default::default(),
            serial_number: core::default::Default::default(),
            sig_alg_name: core::default::Default::default(),
            subject: core::default::Default::default(),
            subject_alternative_names: core::default::Default::default(),
            valid_from: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_constraints` after provisioning.\n"]
    pub fn basic_constraints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `expiry_date` after provisioning.\n"]
    pub fn expiry_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiry_date", self.base))
    }

    #[doc = "Get a reference to the value of field `is_valid` after provisioning.\n"]
    pub fn is_valid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_valid", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc = "Get a reference to the value of field `sig_alg_name` after provisioning.\n"]
    pub fn sig_alg_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sig_alg_name", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_info: std::option::Option<terrars::ListField<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl>>,
}

impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    #[doc = "Set the field `cert_info`.\n"]
    pub fn set_cert_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoEl>>,
    ) -> Self {
        self.cert_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {}

impl BuildApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoEl { cert_info: core::default::Default::default() }
    }
}

pub struct ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
        ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesSelfSignedCertCertsInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert_info` after provisioning.\n"]
    pub fn cert_info(&self) -> terrars::ListRef<ApigeeKeystoresAliasesSelfSignedCertCertsInfoElCertInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert_info", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    #[doc = "Set the field `common_name`.\nCommon name of the organization. Maximum length is 64 characters."]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `country_code`.\nTwo-letter country code. Example, IN for India, US for United States of America."]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\nEmail address. Max 255 characters."]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\nCity or town name. Maximum length is 128 characters."]
    pub fn set_locality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `org`.\nOrganization name. Maximum length is 64 characters."]
    pub fn set_org(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.org = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `org_unit`.\nOrganization team name. Maximum length is 64 characters."]
    pub fn set_org_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.org_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\nState or district name. Maximum length is 128 characters."]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesSelfSignedCertSubjectEl {}

impl BuildApigeeKeystoresAliasesSelfSignedCertSubjectEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
        ApigeeKeystoresAliasesSelfSignedCertSubjectEl {
            common_name: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            locality: core::default::Default::default(),
            org: core::default::Default::default(),
            org_unit: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
        ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesSelfSignedCertSubjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `common_name` after provisioning.\nCommon name of the organization. Maximum length is 64 characters."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nTwo-letter country code. Example, IN for India, US for United States of America."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nEmail address. Max 255 characters."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `locality` after provisioning.\nCity or town name. Maximum length is 128 characters."]
    pub fn locality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc =
        "Get a reference to the value of field `org` after provisioning.\nOrganization name. Maximum length is 64 characters."]
    pub fn org(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org", self.base))
    }

    #[doc =
        "Get a reference to the value of field `org_unit` after provisioning.\nOrganization team name. Maximum length is 64 characters."]
    pub fn org_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_unit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState or district name. Maximum length is 128 characters."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    #[doc = "Set the field `subject_alternative_name`.\nSubject Alternative Name"]
    pub fn set_subject_alternative_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subject_alternative_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {}

impl BuildApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
        ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl {
            subject_alternative_name: core::default::Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
        ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `subject_alternative_name` after provisioning.\nSubject Alternative Name"]
    pub fn subject_alternative_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_alternative_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
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
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {}

impl BuildApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
    pub fn build(self) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
        ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesSelfSignedCertTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct ApigeeKeystoresAliasesSelfSignedCertDynamic {
    subject: std::option::Option<terrars::DynamicBlock<ApigeeKeystoresAliasesSelfSignedCertSubjectEl>>,
    subject_alternative_dns_names: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeKeystoresAliasesSelfSignedCertSubjectAlternativeDnsNamesEl>>,
}
