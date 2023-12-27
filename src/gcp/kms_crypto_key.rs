use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct KmsCryptoKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destroy_scheduled_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_only: Option<PrimField<bool>>,
    key_ring: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_initial_version_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KmsCryptoKeyTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_template: Option<Vec<KmsCryptoKeyVersionTemplateEl>>,
    dynamic: KmsCryptoKeyDynamic,
}

struct KmsCryptoKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KmsCryptoKeyData>,
}

#[derive(Clone)]
pub struct KmsCryptoKey(Rc<KmsCryptoKey_>);

impl KmsCryptoKey {
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

    #[doc= "Set the field `destroy_scheduled_duration`.\nThe period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.\nIf not specified at creation time, the default duration is 24 hours."]
    pub fn set_destroy_scheduled_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().destroy_scheduled_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `import_only`.\nWhether this key may contain imported versions only."]
    pub fn set_import_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().import_only = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose`.\nThe immutable purpose of this CryptoKey. See the\n[purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)\nfor possible inputs.\nDefault value is \"ENCRYPT_DECRYPT\"."]
    pub fn set_purpose(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().purpose = Some(v.into());
        self
    }

    #[doc= "Set the field `rotation_period`.\nEvery time this period passes, generate a new CryptoKeyVersion and set it as the primary.\nThe first rotation will take place after the specified period. The rotation period has\nthe format of a decimal number with up to 9 fractional digits, followed by the\nletter 's' (seconds). It must be greater than a day (ie, 86400)."]
    pub fn set_rotation_period(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rotation_period = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_initial_version_creation`.\nIf set to true, the request will create a CryptoKey without any CryptoKeyVersions.\nYou must use the 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion."]
    pub fn set_skip_initial_version_creation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_initial_version_creation = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KmsCryptoKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Set the field `version_template`.\n"]
    pub fn set_version_template(self, v: impl Into<BlockAssignable<KmsCryptoKeyVersionTemplateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().version_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.version_template = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `destroy_scheduled_duration` after provisioning.\nThe period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.\nIf not specified at creation time, the default duration is 24 hours."]
    pub fn destroy_scheduled_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destroy_scheduled_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_only` after provisioning.\nWhether this key may contain imported versions only."]
    pub fn import_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this key belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_ring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name for the CryptoKey."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\nThe immutable purpose of this CryptoKey. See the\n[purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)\nfor possible inputs.\nDefault value is \"ENCRYPT_DECRYPT\"."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_period` after provisioning.\nEvery time this period passes, generate a new CryptoKeyVersion and set it as the primary.\nThe first rotation will take place after the specified period. The rotation period has\nthe format of a decimal number with up to 9 fractional digits, followed by the\nletter 's' (seconds). It must be greater than a day (ie, 86400)."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_initial_version_creation` after provisioning.\nIf set to true, the request will create a CryptoKey without any CryptoKeyVersions.\nYou must use the 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion."]
    pub fn skip_initial_version_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_initial_version_creation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCryptoKeyTimeoutsElRef {
        KmsCryptoKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_template` after provisioning.\n"]
    pub fn version_template(&self) -> ListRef<KmsCryptoKeyVersionTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.version_template", self.extract_ref()))
    }
}

impl Referable for KmsCryptoKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for KmsCryptoKey { }

impl ToListMappable for KmsCryptoKey {
    type O = ListRef<KmsCryptoKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KmsCryptoKey_ {
    fn extract_resource_type(&self) -> String {
        "google_kms_crypto_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsCryptoKey {
    pub tf_id: String,
    #[doc= "The KeyRing that this key belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub key_ring: PrimField<String>,
    #[doc= "The resource name for the CryptoKey."]
    pub name: PrimField<String>,
}

impl BuildKmsCryptoKey {
    pub fn build(self, stack: &mut Stack) -> KmsCryptoKey {
        let out = KmsCryptoKey(Rc::new(KmsCryptoKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KmsCryptoKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                destroy_scheduled_duration: core::default::Default::default(),
                id: core::default::Default::default(),
                import_only: core::default::Default::default(),
                key_ring: self.key_ring,
                labels: core::default::Default::default(),
                name: self.name,
                purpose: core::default::Default::default(),
                rotation_period: core::default::Default::default(),
                skip_initial_version_creation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                version_template: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsCryptoKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsCryptoKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KmsCryptoKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `destroy_scheduled_duration` after provisioning.\nThe period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED.\nIf not specified at creation time, the default duration is 24 hours."]
    pub fn destroy_scheduled_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destroy_scheduled_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `import_only` after provisioning.\nWhether this key may contain imported versions only."]
    pub fn import_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.import_only", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_ring` after provisioning.\nThe KeyRing that this key belongs to.\nFormat: ''projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}''."]
    pub fn key_ring(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_ring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name for the CryptoKey."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\nThe immutable purpose of this CryptoKey. See the\n[purpose reference](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys#CryptoKeyPurpose)\nfor possible inputs.\nDefault value is \"ENCRYPT_DECRYPT\"."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rotation_period` after provisioning.\nEvery time this period passes, generate a new CryptoKeyVersion and set it as the primary.\nThe first rotation will take place after the specified period. The rotation period has\nthe format of a decimal number with up to 9 fractional digits, followed by the\nletter 's' (seconds). It must be greater than a day (ie, 86400)."]
    pub fn rotation_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rotation_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `skip_initial_version_creation` after provisioning.\nIf set to true, the request will create a CryptoKey without any CryptoKeyVersions.\nYou must use the 'google_kms_key_ring_import_job' resource to import the CryptoKeyVersion."]
    pub fn skip_initial_version_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_initial_version_creation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCryptoKeyTimeoutsElRef {
        KmsCryptoKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_template` after provisioning.\n"]
    pub fn version_template(&self) -> ListRef<KmsCryptoKeyVersionTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.version_template", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KmsCryptoKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl KmsCryptoKeyTimeoutsEl {
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

impl ToListMappable for KmsCryptoKeyTimeoutsEl {
    type O = BlockAssignable<KmsCryptoKeyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCryptoKeyTimeoutsEl {}

impl BuildKmsCryptoKeyTimeoutsEl {
    pub fn build(self) -> KmsCryptoKeyTimeoutsEl {
        KmsCryptoKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KmsCryptoKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsCryptoKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KmsCryptoKeyTimeoutsElRef {
        KmsCryptoKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCryptoKeyTimeoutsElRef {
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

#[derive(Serialize)]
pub struct KmsCryptoKeyVersionTemplateEl {
    algorithm: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection_level: Option<PrimField<String>>,
}

impl KmsCryptoKeyVersionTemplateEl {
    #[doc= "Set the field `protection_level`.\nThe protection level to use when creating a version based on this template. Possible values include \"SOFTWARE\", \"HSM\", \"EXTERNAL\", \"EXTERNAL_VPC\". Defaults to \"SOFTWARE\"."]
    pub fn set_protection_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protection_level = Some(v.into());
        self
    }
}

impl ToListMappable for KmsCryptoKeyVersionTemplateEl {
    type O = BlockAssignable<KmsCryptoKeyVersionTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCryptoKeyVersionTemplateEl {
    #[doc= "The algorithm to use when creating a version based on this template.\nSee the [algorithm reference](https://cloud.google.com/kms/docs/reference/rest/v1/CryptoKeyVersionAlgorithm) for possible inputs."]
    pub algorithm: PrimField<String>,
}

impl BuildKmsCryptoKeyVersionTemplateEl {
    pub fn build(self) -> KmsCryptoKeyVersionTemplateEl {
        KmsCryptoKeyVersionTemplateEl {
            algorithm: self.algorithm,
            protection_level: core::default::Default::default(),
        }
    }
}

pub struct KmsCryptoKeyVersionTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KmsCryptoKeyVersionTemplateElRef {
    fn new(shared: StackShared, base: String) -> KmsCryptoKeyVersionTemplateElRef {
        KmsCryptoKeyVersionTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCryptoKeyVersionTemplateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nThe algorithm to use when creating a version based on this template.\nSee the [algorithm reference](https://cloud.google.com/kms/docs/reference/rest/v1/CryptoKeyVersionAlgorithm) for possible inputs."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `protection_level` after provisioning.\nThe protection level to use when creating a version based on this template. Possible values include \"SOFTWARE\", \"HSM\", \"EXTERNAL\", \"EXTERNAL_VPC\". Defaults to \"SOFTWARE\"."]
    pub fn protection_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protection_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct KmsCryptoKeyDynamic {
    version_template: Option<DynamicBlock<KmsCryptoKeyVersionTemplateEl>>,
}
