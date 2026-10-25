#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataKmsSecretAsymmetricData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    ciphertext: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crc32: std::option::Option<terrars::PrimField<std::string::String>>,
    crypto_key_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataKmsSecretAsymmetric_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKmsSecretAsymmetricData_>,
}

#[derive(Clone)]
pub struct DataKmsSecretAsymmetric(std::rc::Rc<DataKmsSecretAsymmetric_>);

impl DataKmsSecretAsymmetric {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        "Set the field `crc32`.\nThe crc32 checksum of the ciphertext, hexadecimal encoding. If not specified, it will be computed"]
    pub fn set_crc32(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().crc32 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ciphertext` after provisioning.\nThe public key encrypted ciphertext in base64 encoding"]
    pub fn ciphertext(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ciphertext", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crc32` after provisioning.\nThe crc32 checksum of the ciphertext, hexadecimal encoding. If not specified, it will be computed"]
    pub fn crc32(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crc32", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_key_version` after provisioning.\nThe fully qualified KMS crypto key version name"]
    pub fn crypto_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plaintext", self.extract_ref()))
    }
}

impl terrars::Referable for DataKmsSecretAsymmetric {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKmsSecretAsymmetric { }

impl terrars::ToListMappable for DataKmsSecretAsymmetric {
    type O = terrars::ListRef<DataKmsSecretAsymmetricRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKmsSecretAsymmetric_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_kms_secret_asymmetric".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKmsSecretAsymmetric {
    pub tf_id: std::string::String,
    #[doc = "The public key encrypted ciphertext in base64 encoding"]
    pub ciphertext: terrars::PrimField<std::string::String>,
    #[doc = "The fully qualified KMS crypto key version name"]
    pub crypto_key_version: terrars::PrimField<std::string::String>,
}

impl BuildDataKmsSecretAsymmetric {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKmsSecretAsymmetric {
        let out = DataKmsSecretAsymmetric(std::rc::Rc::new(DataKmsSecretAsymmetric_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKmsSecretAsymmetricData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                ciphertext: self.ciphertext,
                crc32: core::default::Default::default(),
                crypto_key_version: self.crypto_key_version,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKmsSecretAsymmetricRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKmsSecretAsymmetricRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKmsSecretAsymmetricRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `ciphertext` after provisioning.\nThe public key encrypted ciphertext in base64 encoding"]
    pub fn ciphertext(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ciphertext", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crc32` after provisioning.\nThe crc32 checksum of the ciphertext, hexadecimal encoding. If not specified, it will be computed"]
    pub fn crc32(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crc32", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_key_version` after provisioning.\nThe fully qualified KMS crypto key version name"]
    pub fn crypto_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\n"]
    pub fn plaintext(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plaintext", self.extract_ref()))
    }
}
