use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ServiceAccountKeyData {
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
    keepers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key_type: Option<PrimField<String>>,
    service_account_id: PrimField<String>,
}

struct ServiceAccountKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServiceAccountKeyData>,
}

#[derive(Clone)]
pub struct ServiceAccountKey(Rc<ServiceAccountKey_>);

impl ServiceAccountKey {
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

    #[doc= "Set the field `keepers`.\nArbitrary map of values that, when changed, will trigger recreation of resource."]
    pub fn set_keepers(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().keepers = Some(v.into());
        self
    }

    #[doc= "Set the field `key_algorithm`.\nThe algorithm used to generate the key, used only on create. KEY_ALG_RSA_2048 is the default algorithm. Valid values are: \"KEY_ALG_RSA_1024\", \"KEY_ALG_RSA_2048\"."]
    pub fn set_key_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().key_algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `private_key_type`.\n"]
    pub fn set_private_key_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_key_type = Some(v.into());
        self
    }

    #[doc= "Set the field `public_key_data`.\nA field that allows clients to upload their own public key. If set, use this public key data to create a service account key for given service account. Please note, the expected format for this field is a base64 encoded X509_PEM."]
    pub fn set_public_key_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_key_data = Some(v.into());
        self
    }

    #[doc= "Set the field `public_key_type`.\n"]
    pub fn set_public_key_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().public_key_type = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_algorithm` after provisioning.\nThe algorithm used to generate the key, used only on create. KEY_ALG_RSA_2048 is the default algorithm. Valid values are: \"KEY_ALG_RSA_1024\", \"KEY_ALG_RSA_2048\"."]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name used for this key pair"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nThe private key in JSON format, base64 encoded. This is what you normally get as a file when creating service account keys through the CLI or web console. This is only populated when creating a new key."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key_type` after provisioning.\n"]
    pub fn private_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nThe public key, base64 encoded"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key_data` after provisioning.\nA field that allows clients to upload their own public key. If set, use this public key data to create a service account key for given service account. Please note, the expected format for this field is a base64 encoded X509_PEM."]
    pub fn public_key_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key_type` after provisioning.\n"]
    pub fn public_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account_id` after provisioning.\nThe ID of the parent service account of the key. This can be a string in the format {ACCOUNT} or projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}, where {ACCOUNT} is the email address or unique id of the service account. If the {ACCOUNT} syntax is used, the project will be inferred from the provider's configuration."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_after` after provisioning.\nThe key can be used after this timestamp. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_before` after provisioning.\nThe key can be used before this timestamp. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_before", self.extract_ref()))
    }
}

impl Referable for ServiceAccountKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ServiceAccountKey { }

impl ToListMappable for ServiceAccountKey {
    type O = ListRef<ServiceAccountKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServiceAccountKey_ {
    fn extract_resource_type(&self) -> String {
        "google_service_account_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServiceAccountKey {
    pub tf_id: String,
    #[doc= "The ID of the parent service account of the key. This can be a string in the format {ACCOUNT} or projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}, where {ACCOUNT} is the email address or unique id of the service account. If the {ACCOUNT} syntax is used, the project will be inferred from the provider's configuration."]
    pub service_account_id: PrimField<String>,
}

impl BuildServiceAccountKey {
    pub fn build(self, stack: &mut Stack) -> ServiceAccountKey {
        let out = ServiceAccountKey(Rc::new(ServiceAccountKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServiceAccountKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                keepers: core::default::Default::default(),
                key_algorithm: core::default::Default::default(),
                private_key_type: core::default::Default::default(),
                public_key_data: core::default::Default::default(),
                public_key_type: core::default::Default::default(),
                service_account_id: self.service_account_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceAccountKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceAccountKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServiceAccountKeyRef {
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

    #[doc= "Get a reference to the value of field `keepers` after provisioning.\nArbitrary map of values that, when changed, will trigger recreation of resource."]
    pub fn keepers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.keepers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_algorithm` after provisioning.\nThe algorithm used to generate the key, used only on create. KEY_ALG_RSA_2048 is the default algorithm. Valid values are: \"KEY_ALG_RSA_1024\", \"KEY_ALG_RSA_2048\"."]
    pub fn key_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name used for this key pair"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\nThe private key in JSON format, base64 encoded. This is what you normally get as a file when creating service account keys through the CLI or web console. This is only populated when creating a new key."]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_key_type` after provisioning.\n"]
    pub fn private_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\nThe public key, base64 encoded"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key_data` after provisioning.\nA field that allows clients to upload their own public key. If set, use this public key data to create a service account key for given service account. Please note, the expected format for this field is a base64 encoded X509_PEM."]
    pub fn public_key_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `public_key_type` after provisioning.\n"]
    pub fn public_key_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account_id` after provisioning.\nThe ID of the parent service account of the key. This can be a string in the format {ACCOUNT} or projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}, where {ACCOUNT} is the email address or unique id of the service account. If the {ACCOUNT} syntax is used, the project will be inferred from the provider's configuration."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_after` after provisioning.\nThe key can be used after this timestamp. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_before` after provisioning.\nThe key can be used before this timestamp. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn valid_before(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid_before", self.extract_ref()))
    }
}
