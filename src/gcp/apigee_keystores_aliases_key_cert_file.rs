use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeKeystoresAliasesKeyCertFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    alias: PrimField<String>,
    cert: PrimField<String>,
    environment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    keystore: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certs_info: Option<Vec<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeKeystoresAliasesKeyCertFileTimeoutsEl>,
    dynamic: ApigeeKeystoresAliasesKeyCertFileDynamic,
}

struct ApigeeKeystoresAliasesKeyCertFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeKeystoresAliasesKeyCertFileData>,
}

#[derive(Clone)]
pub struct ApigeeKeystoresAliasesKeyCertFile(Rc<ApigeeKeystoresAliasesKeyCertFile_>);

impl ApigeeKeystoresAliasesKeyCertFile {
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

    #[doc= "Set the field `key`.\nPrivate Key content, omit if uploading to truststore"]
    pub fn set_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key = Some(v.into());
        self
    }

    #[doc= "Set the field `password`.\nPassword for the Private Key if it's encrypted"]
    pub fn set_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().password = Some(v.into());
        self
    }

    #[doc= "Set the field `certs_info`.\n"]
    pub fn set_certs_info(self, v: impl Into<BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certs_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certs_info = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeKeystoresAliasesKeyCertFileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nAlias Name"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cert` after provisioning.\nCert content"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nEnvironment associated with the alias"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nPrivate Key content, omit if uploading to truststore"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keystore` after provisioning.\nKeystore Name"]
    pub fn keystore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nOrganization ID associated with the alias"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nPassword for the Private Key if it's encrypted"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certs_info` after provisioning.\n"]
    pub fn certs_info(&self) -> ListRef<ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certs_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ApigeeKeystoresAliasesKeyCertFile {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApigeeKeystoresAliasesKeyCertFile { }

impl ToListMappable for ApigeeKeystoresAliasesKeyCertFile {
    type O = ListRef<ApigeeKeystoresAliasesKeyCertFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApigeeKeystoresAliasesKeyCertFile_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_keystores_aliases_key_cert_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeKeystoresAliasesKeyCertFile {
    pub tf_id: String,
    #[doc= "Alias Name"]
    pub alias: PrimField<String>,
    #[doc= "Cert content"]
    pub cert: PrimField<String>,
    #[doc= "Environment associated with the alias"]
    pub environment: PrimField<String>,
    #[doc= "Keystore Name"]
    pub keystore: PrimField<String>,
    #[doc= "Organization ID associated with the alias"]
    pub org_id: PrimField<String>,
}

impl BuildApigeeKeystoresAliasesKeyCertFile {
    pub fn build(self, stack: &mut Stack) -> ApigeeKeystoresAliasesKeyCertFile {
        let out = ApigeeKeystoresAliasesKeyCertFile(Rc::new(ApigeeKeystoresAliasesKeyCertFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeKeystoresAliasesKeyCertFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeKeystoresAliasesKeyCertFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeKeystoresAliasesKeyCertFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nAlias Name"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cert` after provisioning.\nCert content"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nEnvironment associated with the alias"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nPrivate Key content, omit if uploading to truststore"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keystore` after provisioning.\nKeystore Name"]
    pub fn keystore(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keystore", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nOrganization ID associated with the alias"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nPassword for the Private Key if it's encrypted"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nOptional.Type of Alias"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certs_info` after provisioning.\n"]
    pub fn certs_info(&self) -> ListRef<ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certs_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_constraints: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_valid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sig_alg_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_alternative_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_from: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<f64>>,
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
    #[doc= "Set the field `basic_constraints`.\nX.509 basic constraints extension."]
    pub fn set_basic_constraints(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.basic_constraints = Some(v.into());
        self
    }

    #[doc= "Set the field `expiry_date`.\nX.509 notAfter validity period in milliseconds since epoch."]
    pub fn set_expiry_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expiry_date = Some(v.into());
        self
    }

    #[doc= "Set the field `is_valid`.\nFlag that specifies whether the certificate is valid. \nFlag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid."]
    pub fn set_is_valid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.is_valid = Some(v.into());
        self
    }

    #[doc= "Set the field `issuer`.\nX.509 issuer."]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc= "Set the field `public_key`.\nPublic key component of the X.509 subject public key info."]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }

    #[doc= "Set the field `serial_number`.\nX.509 serial number."]
    pub fn set_serial_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.serial_number = Some(v.into());
        self
    }

    #[doc= "Set the field `sig_alg_name`.\nX.509 signatureAlgorithm."]
    pub fn set_sig_alg_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sig_alg_name = Some(v.into());
        self
    }

    #[doc= "Set the field `subject`.\nX.509 subject."]
    pub fn set_subject(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject = Some(v.into());
        self
    }

    #[doc= "Set the field `subject_alternative_names`.\nX.509 subject alternative names (SANs) extension."]
    pub fn set_subject_alternative_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.subject_alternative_names = Some(v.into());
        self
    }

    #[doc= "Set the field `valid_from`.\nX.509 notBefore validity period in milliseconds since epoch."]
    pub fn set_valid_from(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.valid_from = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nX.509 version."]
    pub fn set_version(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
        ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `basic_constraints` after provisioning.\nX.509 basic constraints extension."]
    pub fn basic_constraints(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.basic_constraints", self.base))
    }

    #[doc= "Get a reference to the value of field `expiry_date` after provisioning.\nX.509 notAfter validity period in milliseconds since epoch."]
    pub fn expiry_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiry_date", self.base))
    }

    #[doc= "Get a reference to the value of field `is_valid` after provisioning.\nFlag that specifies whether the certificate is valid. \nFlag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid."]
    pub fn is_valid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_valid", self.base))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\nX.509 issuer."]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nPublic key component of the X.509 subject public key info."]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc= "Get a reference to the value of field `serial_number` after provisioning.\nX.509 serial number."]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc= "Get a reference to the value of field `sig_alg_name` after provisioning.\nX.509 signatureAlgorithm."]
    pub fn sig_alg_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sig_alg_name", self.base))
    }

    #[doc= "Get a reference to the value of field `subject` after provisioning.\nX.509 subject."]
    pub fn subject(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc= "Get a reference to the value of field `subject_alternative_names` after provisioning.\nX.509 subject alternative names (SANs) extension."]
    pub fn subject_alternative_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc= "Get a reference to the value of field `valid_from` after provisioning.\nX.509 notBefore validity period in milliseconds since epoch."]
    pub fn valid_from(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nX.509 version."]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElDynamic {
    cert_info: Option<DynamicBlock<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>>,
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_info: Option<Vec<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>>,
    dynamic: ApigeeKeystoresAliasesKeyCertFileCertsInfoElDynamic,
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    #[doc= "Set the field `cert_info`.\n"]
    pub fn set_cert_info(
        mut self,
        v: impl Into<BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cert_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cert_info = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ApigeeKeystoresAliasesKeyCertFileCertsInfoEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
        ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileCertsInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cert_info` after provisioning.\n"]
    pub fn cert_info(&self) -> ListRef<ApigeeKeystoresAliasesKeyCertFileCertsInfoElCertInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cert_info", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
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

    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ApigeeKeystoresAliasesKeyCertFileTimeoutsEl {
    type O = BlockAssignable<ApigeeKeystoresAliasesKeyCertFileTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
        ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeKeystoresAliasesKeyCertFileTimeoutsElRef {
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

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeKeystoresAliasesKeyCertFileDynamic {
    certs_info: Option<DynamicBlock<ApigeeKeystoresAliasesKeyCertFileCertsInfoEl>>,
}
