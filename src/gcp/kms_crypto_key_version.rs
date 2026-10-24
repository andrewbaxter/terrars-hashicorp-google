use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct KmsCryptoKeyVersionData {
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
    crypto_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KmsCryptoKeyVersionTimeoutsEl>,
}

struct KmsCryptoKeyVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KmsCryptoKeyVersionData>,
}

#[derive(Clone)]
pub struct KmsCryptoKeyVersion(std::rc::Rc<KmsCryptoKeyVersion_>);

impl KmsCryptoKeyVersion {
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

    #[doc =
        "Set the field `state`.\nThe current state of the CryptoKeyVersion. Possible values: [\"PENDING_GENERATION\", \"ENABLED\", \"DISABLED\", \"DESTROYED\", \"DESTROY_SCHEDULED\", \"PENDING_IMPORT\", \"IMPORT_FAILED\"]"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KmsCryptoKeyVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nThe CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly provided for key versions with protectionLevel HSM."]
    pub fn attestation(&self) -> terrars::ListRef<KmsCryptoKeyVersionAttestationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_key` after provisioning.\nThe name of the cryptoKey associated with the CryptoKeyVersions.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}''"]
    pub fn crypto_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generate_time` after provisioning.\nThe time this CryptoKeyVersion key material was generated"]
    pub fn generate_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CryptoKeyVersion."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protection_level` after provisioning.\nThe ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
    pub fn protection_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protection_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the CryptoKeyVersion. Possible values: [\"PENDING_GENERATION\", \"ENABLED\", \"DISABLED\", \"DESTROYED\", \"DESTROY_SCHEDULED\", \"PENDING_IMPORT\", \"IMPORT_FAILED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCryptoKeyVersionTimeoutsElRef {
        KmsCryptoKeyVersionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for KmsCryptoKeyVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KmsCryptoKeyVersion { }

impl terrars::ToListMappable for KmsCryptoKeyVersion {
    type O = terrars::ListRef<KmsCryptoKeyVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KmsCryptoKeyVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_kms_crypto_key_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsCryptoKeyVersion {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the cryptoKey associated with the CryptoKeyVersions.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}''"]
    pub crypto_key: terrars::PrimField<std::string::String>,
}

impl BuildKmsCryptoKeyVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> KmsCryptoKeyVersion {
        let out = KmsCryptoKeyVersion(std::rc::Rc::new(KmsCryptoKeyVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KmsCryptoKeyVersionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                crypto_key: self.crypto_key,
                id: core::default::Default::default(),
                state: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsCryptoKeyVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCryptoKeyVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KmsCryptoKeyVersionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nThe CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly provided for key versions with protectionLevel HSM."]
    pub fn attestation(&self) -> terrars::ListRef<KmsCryptoKeyVersionAttestationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_key` after provisioning.\nThe name of the cryptoKey associated with the CryptoKeyVersions.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyring}}/cryptoKeys/{{cryptoKey}}''"]
    pub fn crypto_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generate_time` after provisioning.\nThe time this CryptoKeyVersion key material was generated"]
    pub fn generate_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generate_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CryptoKeyVersion."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protection_level` after provisioning.\nThe ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
    pub fn protection_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protection_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the CryptoKeyVersion. Possible values: [\"PENDING_GENERATION\", \"ENABLED\", \"DISABLED\", \"DESTROYED\", \"DESTROY_SCHEDULED\", \"PENDING_IMPORT\", \"IMPORT_FAILED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCryptoKeyVersionTimeoutsElRef {
        KmsCryptoKeyVersionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KmsCryptoKeyVersionAttestationElCertChainsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cavium_certs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_card_certs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_partition_certs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsCryptoKeyVersionAttestationElCertChainsEl {
    #[doc = "Set the field `cavium_certs`.\n"]
    pub fn set_cavium_certs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cavium_certs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_card_certs`.\n"]
    pub fn set_google_card_certs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.google_card_certs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_partition_certs`.\n"]
    pub fn set_google_partition_certs(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.google_partition_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KmsCryptoKeyVersionAttestationElCertChainsEl {
    type O = terrars::BlockAssignable<KmsCryptoKeyVersionAttestationElCertChainsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCryptoKeyVersionAttestationElCertChainsEl {}

impl BuildKmsCryptoKeyVersionAttestationElCertChainsEl {
    pub fn build(self) -> KmsCryptoKeyVersionAttestationElCertChainsEl {
        KmsCryptoKeyVersionAttestationElCertChainsEl {
            cavium_certs: core::default::Default::default(),
            google_card_certs: core::default::Default::default(),
            google_partition_certs: core::default::Default::default(),
        }
    }
}

pub struct KmsCryptoKeyVersionAttestationElCertChainsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCryptoKeyVersionAttestationElCertChainsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KmsCryptoKeyVersionAttestationElCertChainsElRef {
        KmsCryptoKeyVersionAttestationElCertChainsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCryptoKeyVersionAttestationElCertChainsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cavium_certs` after provisioning.\n"]
    pub fn cavium_certs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cavium_certs", self.base))
    }

    #[doc = "Get a reference to the value of field `google_card_certs` after provisioning.\n"]
    pub fn google_card_certs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_card_certs", self.base))
    }

    #[doc = "Get a reference to the value of field `google_partition_certs` after provisioning.\n"]
    pub fn google_partition_certs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_partition_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ekm_connection_key_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_key_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    #[doc = "Set the field `ekm_connection_key_path`.\n"]
    pub fn set_ekm_connection_key_path(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ekm_connection_key_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_key_uri`.\n"]
    pub fn set_external_key_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_key_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    type O = terrars::BlockAssignable<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {}

impl BuildKmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
    pub fn build(self) -> KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
        KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl {
            ekm_connection_key_path: core::default::Default::default(),
            external_key_uri: core::default::Default::default(),
        }
    }
}

pub struct KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
        KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ekm_connection_key_path` after provisioning.\n"]
    pub fn ekm_connection_key_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ekm_connection_key_path", self.base))
    }

    #[doc = "Get a reference to the value of field `external_key_uri` after provisioning.\n"]
    pub fn external_key_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_key_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct KmsCryptoKeyVersionAttestationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_chains: std::option::Option<terrars::ListField<KmsCryptoKeyVersionAttestationElCertChainsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_protection_level_options: std
    ::option
    ::Option<terrars::ListField<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsCryptoKeyVersionAttestationEl {
    #[doc = "Set the field `cert_chains`.\n"]
    pub fn set_cert_chains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<KmsCryptoKeyVersionAttestationElCertChainsEl>>,
    ) -> Self {
        self.cert_chains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_protection_level_options`.\n"]
    pub fn set_external_protection_level_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsEl>>,
    ) -> Self {
        self.external_protection_level_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KmsCryptoKeyVersionAttestationEl {
    type O = terrars::BlockAssignable<KmsCryptoKeyVersionAttestationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCryptoKeyVersionAttestationEl {}

impl BuildKmsCryptoKeyVersionAttestationEl {
    pub fn build(self) -> KmsCryptoKeyVersionAttestationEl {
        KmsCryptoKeyVersionAttestationEl {
            cert_chains: core::default::Default::default(),
            content: core::default::Default::default(),
            external_protection_level_options: core::default::Default::default(),
            format: core::default::Default::default(),
        }
    }
}

pub struct KmsCryptoKeyVersionAttestationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCryptoKeyVersionAttestationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsCryptoKeyVersionAttestationElRef {
        KmsCryptoKeyVersionAttestationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCryptoKeyVersionAttestationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert_chains` after provisioning.\n"]
    pub fn cert_chains(&self) -> terrars::ListRef<KmsCryptoKeyVersionAttestationElCertChainsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert_chains", self.base))
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `external_protection_level_options` after provisioning.\n"]
    pub fn external_protection_level_options(
        &self,
    ) -> terrars::ListRef<KmsCryptoKeyVersionAttestationElExternalProtectionLevelOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_protection_level_options", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
}

#[derive(Serialize)]
pub struct KmsCryptoKeyVersionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsCryptoKeyVersionTimeoutsEl {
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

impl terrars::ToListMappable for KmsCryptoKeyVersionTimeoutsEl {
    type O = terrars::BlockAssignable<KmsCryptoKeyVersionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCryptoKeyVersionTimeoutsEl {}

impl BuildKmsCryptoKeyVersionTimeoutsEl {
    pub fn build(self) -> KmsCryptoKeyVersionTimeoutsEl {
        KmsCryptoKeyVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KmsCryptoKeyVersionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCryptoKeyVersionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsCryptoKeyVersionTimeoutsElRef {
        KmsCryptoKeyVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCryptoKeyVersionTimeoutsElRef {
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
