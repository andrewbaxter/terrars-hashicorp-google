use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeKeystoresAliasesKeyCertFileData {
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
    cert: terrars::PrimField<std::string::String>,
    environment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    keystore: terrars::PrimField<std::string::String>,
    org_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certs_info: std::option::Option<std::vec::Vec<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeKeystoresAliasesKeyCertFileTimeoutsEl>,
    dynamic: ApigeeKeystoresAliasesKeyCertFileDynamic,
}

struct ApigeeKeystoresAliasesKeyCertFile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeKeystoresAliasesKeyCertFileData>,
}

#[derive(Clone)]
pub struct ApigeeKeystoresAliasesKeyCertFile(std::rc::Rc<ApigeeKeystoresAliasesKeyCertFile_>);

impl ApigeeKeystoresAliasesKeyCertFile {
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

    #[doc = "Set the field `key`.\nPrivate Key content, omit if uploading to truststore"]
    pub fn set_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nPassword for the Private Key if it's encrypted"]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certs_info`.\n"]
    pub fn set_certs_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certs_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certs_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeKeystoresAliasesKeyCertFileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\nAlias Name"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\nCert content"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nEnvironment associated with the alias"]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nPrivate Key content, omit if uploading to truststore"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keystore` after provisioning.\nKeystore Name"]
    pub fn keystore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nOrganization ID associated with the alias"]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword for the Private Key if it's encrypted"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certs_info` after provisioning.\n"]
    pub fn certs_info(&self) -> terrars::ListRef<ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certs_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ApigeeKeystoresAliasesKeyCertFile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeKeystoresAliasesKeyCertFile { }

impl terrars::ToListMappable for ApigeeKeystoresAliasesKeyCertFile {
    type O = terrars::ListRef<ApigeeKeystoresAliasesKeyCertFileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeKeystoresAliasesKeyCertFile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_keystores_aliases_key_cert_file".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeKeystoresAliasesKeyCertFile {
    pub tf_id: std::string::String,
    #[doc = "Alias Name"]
    pub alias: terrars::PrimField<std::string::String>,
    #[doc = "Cert content"]
    pub cert: terrars::PrimField<std::string::String>,
    #[doc = "Environment associated with the alias"]
    pub environment: terrars::PrimField<std::string::String>,
    #[doc = "Keystore Name"]
    pub keystore: terrars::PrimField<std::string::String>,
    #[doc = "Organization ID associated with the alias"]
    pub org_id: terrars::PrimField<std::string::String>,
}

impl BuildApigeeKeystoresAliasesKeyCertFile {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeKeystoresAliasesKeyCertFile {
        let out = ApigeeKeystoresAliasesKeyCertFile(std::rc::Rc::new(ApigeeKeystoresAliasesKeyCertFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeKeystoresAliasesKeyCertFileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: self.alias,
                cert: self.cert,
                environment: self.environment,
                id: core::default::Default::default(),
                key: core::default::Default::default(),
                keystore: self.keystore,
                org_id: self.org_id,
                password: core::default::Default::default(),
                certs_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeKeystoresAliasesKeyCertFileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesKeyCertFileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\nAlias Name"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\nCert content"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nEnvironment associated with the alias"]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nPrivate Key content, omit if uploading to truststore"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keystore` after provisioning.\nKeystore Name"]
    pub fn keystore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nOrganization ID associated with the alias"]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword for the Private Key if it's encrypted"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certs_info` after provisioning.\n"]
    pub fn certs_info(&self) -> terrars::ListRef<ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certs_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
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

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
    #[doc = "Set the field `basic_constraints`.\nX.509 basic constraints extension."]
    pub fn set_basic_constraints(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.basic_constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiry_date`.\nX.509 notAfter validity period in milliseconds since epoch."]
    pub fn set_expiry_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiry_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_valid`.\nFlag that specifies whether the certificate is valid. \nFlag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid."]
    pub fn set_is_valid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.is_valid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\nX.509 issuer."]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\nPublic key component of the X.509 subject public key info."]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serial_number`.\nX.509 serial number."]
    pub fn set_serial_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serial_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sig_alg_name`.\nX.509 signatureAlgorithm."]
    pub fn set_sig_alg_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sig_alg_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\nX.509 subject."]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alternative_names`.\nX.509 subject alternative names (SANs) extension."]
    pub fn set_subject_alternative_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subject_alternative_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_from`.\nX.509 notBefore validity period in milliseconds since epoch."]
    pub fn set_valid_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.valid_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nX.509 version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {}

impl BuildApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
        ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
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

pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
        ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `basic_constraints` after provisioning.\nX.509 basic constraints extension."]
    pub fn basic_constraints(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_constraints", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expiry_date` after provisioning.\nX.509 notAfter validity period in milliseconds since epoch."]
    pub fn expiry_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiry_date", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_valid` after provisioning.\nFlag that specifies whether the certificate is valid. \nFlag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid."]
    pub fn is_valid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_valid", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nX.509 issuer."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nPublic key component of the X.509 subject public key info."]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\nX.509 serial number."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc = "Get a reference to the value of field `sig_alg_name` after provisioning.\nX.509 signatureAlgorithm."]
    pub fn sig_alg_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sig_alg_name", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\nX.509 subject."]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject_alternative_names` after provisioning.\nX.509 subject alternative names (SANs) extension."]
    pub fn subject_alternative_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `valid_from` after provisioning.\nX.509 notBefore validity period in milliseconds since epoch."]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nX.509 version."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElDynamic {
    cert_info: std::option::Option<terrars::DynamicBlock<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>>,
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_info: std::option::Option<std::vec::Vec<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>>,
    dynamic: ApigeeKeystoresAliasesKeyCertFileCertsInfoElDynamic,
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    #[doc = "Set the field `cert_info`.\n"]
    pub fn set_cert_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cert_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cert_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesKeyCertFileCertsInfoEl {}

impl BuildApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    pub fn build(self) -> ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
        ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
            cert_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
        ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert_info` after provisioning.\n"]
    pub fn cert_info(&self) -> terrars::ListRef<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert_info", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
    type O = terrars::BlockAssignable<ApigeeKeystoresAliasesKeyCertFileTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeKeystoresAliasesKeyCertFileTimeoutsEl {}

impl BuildApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
    pub fn build(self) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeKeystoresAliasesKeyCertFileDynamic {
    certs_info: std::option::Option<terrars::DynamicBlock<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>>,
}
