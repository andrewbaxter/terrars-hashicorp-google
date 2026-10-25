#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct KmsKeyRingImportJobData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    import_job_id: terrars::PrimField<std::string::String>,
    import_method: terrars::PrimField<std::string::String>,
    key_ring: terrars::PrimField<std::string::String>,
    protection_level: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KmsKeyRingImportJob_Timeouts>,
}

struct KmsKeyRingImportJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KmsKeyRingImportJobData_>,
}

#[derive(Clone)]
pub struct KmsKeyRingImportJob(std::rc::Rc<KmsKeyRingImportJob_>);

impl KmsKeyRingImportJob {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KmsKeyRingImportJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the key creator (for example, an HSM) at key creation time.\nUse this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly present if the chosen ImportMethod is one with a protection level of HSM."]
    pub fn attestation(&self) -> terrars::ListRef<KmsKeyRingImportJob_AttestationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe time at which this resource is scheduled for expiration and can no longer be used.\nThis is in RFC3339 text format."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `import_job_id` after provisioning.\nIt must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}"]
    pub fn import_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `import_method` after provisioning.\nThe wrapping method to be used for incoming key material. Possible values: [\"RSA_OAEP_3072_SHA1_AES_256\", \"RSA_OAEP_4096_SHA1_AES_256\", \"RSA_OAEP_3072_SHA256_AES_256\", \"RSA_OAEP_4096_SHA256_AES_256\", \"RSA_OAEP_3072_SHA256\", \"RSA_OAEP_4096_SHA256\"]"]
    pub fn import_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this import job belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_ring", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this ImportJob in the format projects/*/locations/*/keyRings/*/importJobs/*."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protection_level` after provisioning.\nThe protection level of the ImportJob. This must match the protectionLevel of the\nversionTemplate on the CryptoKey you attempt to import into. Possible values: [\"SOFTWARE\", \"HSM\", \"EXTERNAL\"]"]
    pub fn protection_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protection_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nThe public key with which to wrap key material prior to import. Only returned if state is 'ACTIVE'."]
    pub fn public_key(&self) -> terrars::ListRef<KmsKeyRingImportJob_PublicKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the ImportJob, indicating if it can be used."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsKeyRingImportJob_TimeoutsRef {
        KmsKeyRingImportJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for KmsKeyRingImportJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KmsKeyRingImportJob { }

impl terrars::ToListMappable for KmsKeyRingImportJob {
    type O = terrars::ListRef<KmsKeyRingImportJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KmsKeyRingImportJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_kms_key_ring_import_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsKeyRingImportJob {
    pub tf_id: std::string::String,
    #[doc = "It must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}"]
    pub import_job_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The wrapping method to be used for incoming key material. Possible values: [\"RSA_OAEP_3072_SHA1_AES_256\", \"RSA_OAEP_4096_SHA1_AES_256\", \"RSA_OAEP_3072_SHA256_AES_256\", \"RSA_OAEP_4096_SHA256_AES_256\", \"RSA_OAEP_3072_SHA256\", \"RSA_OAEP_4096_SHA256\"]"]
    pub import_method: terrars::PrimField<std::string::String>,
    #[doc =
        "The KeyRing that this import job belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub key_ring: terrars::PrimField<std::string::String>,
    #[doc =
        "The protection level of the ImportJob. This must match the protectionLevel of the\nversionTemplate on the CryptoKey you attempt to import into. Possible values: [\"SOFTWARE\", \"HSM\", \"EXTERNAL\"]"]
    pub protection_level: terrars::PrimField<std::string::String>,
}

impl BuildKmsKeyRingImportJob {
    pub fn build(self, stack: &mut terrars::Stack) -> KmsKeyRingImportJob {
        let out = KmsKeyRingImportJob(std::rc::Rc::new(KmsKeyRingImportJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KmsKeyRingImportJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                import_job_id: self.import_job_id,
                import_method: self.import_method,
                key_ring: self.key_ring,
                protection_level: self.protection_level,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsKeyRingImportJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsKeyRingImportJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KmsKeyRingImportJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attestation` after provisioning.\nStatement that was generated and signed by the key creator (for example, an HSM) at key creation time.\nUse this statement to verify attributes of the key as stored on the HSM, independently of Google.\nOnly present if the chosen ImportMethod is one with a protection level of HSM."]
    pub fn attestation(&self) -> terrars::ListRef<KmsKeyRingImportJob_AttestationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe time at which this resource is scheduled for expiration and can no longer be used.\nThis is in RFC3339 text format."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `import_job_id` after provisioning.\nIt must be unique within a KeyRing and match the regular expression [a-zA-Z0-9_-]{1,63}"]
    pub fn import_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `import_method` after provisioning.\nThe wrapping method to be used for incoming key material. Possible values: [\"RSA_OAEP_3072_SHA1_AES_256\", \"RSA_OAEP_4096_SHA1_AES_256\", \"RSA_OAEP_3072_SHA256_AES_256\", \"RSA_OAEP_4096_SHA256_AES_256\", \"RSA_OAEP_3072_SHA256\", \"RSA_OAEP_4096_SHA256\"]"]
    pub fn import_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this import job belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_ring", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this ImportJob in the format projects/*/locations/*/keyRings/*/importJobs/*."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protection_level` after provisioning.\nThe protection level of the ImportJob. This must match the protectionLevel of the\nversionTemplate on the CryptoKey you attempt to import into. Possible values: [\"SOFTWARE\", \"HSM\", \"EXTERNAL\"]"]
    pub fn protection_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protection_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_key` after provisioning.\nThe public key with which to wrap key material prior to import. Only returned if state is 'ACTIVE'."]
    pub fn public_key(&self) -> terrars::ListRef<KmsKeyRingImportJob_PublicKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the ImportJob, indicating if it can be used."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsKeyRingImportJob_TimeoutsRef {
        KmsKeyRingImportJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KmsKeyRingImportJob_Attestation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsKeyRingImportJob_Attestation {
    #[doc = "Set the field `content`.\n"]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KmsKeyRingImportJob_Attestation {
    type O = terrars::BlockAssignable<KmsKeyRingImportJob_Attestation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsKeyRingImportJob_Attestation {}

impl BuildKmsKeyRingImportJob_Attestation {
    pub fn build(self) -> KmsKeyRingImportJob_Attestation {
        KmsKeyRingImportJob_Attestation {
            content: core::default::Default::default(),
            format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KmsKeyRingImportJob_AttestationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsKeyRingImportJob_AttestationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsKeyRingImportJob_AttestationRef {
        KmsKeyRingImportJob_AttestationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsKeyRingImportJob_AttestationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
}

#[derive(Serialize)]
pub struct KmsKeyRingImportJob_PublicKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsKeyRingImportJob_PublicKey {
    #[doc = "Set the field `pem`.\n"]
    pub fn set_pem(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pem = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KmsKeyRingImportJob_PublicKey {
    type O = terrars::BlockAssignable<KmsKeyRingImportJob_PublicKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsKeyRingImportJob_PublicKey {}

impl BuildKmsKeyRingImportJob_PublicKey {
    pub fn build(self) -> KmsKeyRingImportJob_PublicKey {
        KmsKeyRingImportJob_PublicKey { pem: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct KmsKeyRingImportJob_PublicKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsKeyRingImportJob_PublicKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsKeyRingImportJob_PublicKeyRef {
        KmsKeyRingImportJob_PublicKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsKeyRingImportJob_PublicKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pem` after provisioning.\n"]
    pub fn pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem", self.base))
    }
}

#[derive(Serialize)]
pub struct KmsKeyRingImportJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsKeyRingImportJob_Timeouts {
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

impl terrars::ToListMappable for KmsKeyRingImportJob_Timeouts {
    type O = terrars::BlockAssignable<KmsKeyRingImportJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsKeyRingImportJob_Timeouts {}

impl BuildKmsKeyRingImportJob_Timeouts {
    pub fn build(self) -> KmsKeyRingImportJob_Timeouts {
        KmsKeyRingImportJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KmsKeyRingImportJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsKeyRingImportJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsKeyRingImportJob_TimeoutsRef {
        KmsKeyRingImportJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsKeyRingImportJob_TimeoutsRef {
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
}
