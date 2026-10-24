use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BinaryAuthorizationAttestorData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attestation_authority_note: std::option::Option<std::vec::Vec<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BinaryAuthorizationAttestorTimeoutsEl>,
    dynamic: BinaryAuthorizationAttestorDynamic,
}

struct BinaryAuthorizationAttestor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BinaryAuthorizationAttestorData>,
}

#[derive(Clone)]
pub struct BinaryAuthorizationAttestor(std::rc::Rc<BinaryAuthorizationAttestor_>);

impl BinaryAuthorizationAttestor {
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
        "Set the field `description`.\nA descriptive comment. This field may be updated. The field may be\ndisplayed in chooser dialogs."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `attestation_authority_note`.\n"]
    pub fn set_attestation_authority_note(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_authority_note = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_authority_note = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BinaryAuthorizationAttestorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA descriptive comment. This field may be updated. The field may be\ndisplayed in chooser dialogs."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attestation_authority_note` after provisioning.\n"]
    pub fn attestation_authority_note(
        &self,
    ) -> terrars::ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation_authority_note", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationAttestorTimeoutsElRef {
        BinaryAuthorizationAttestorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BinaryAuthorizationAttestor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BinaryAuthorizationAttestor { }

impl terrars::ToListMappable for BinaryAuthorizationAttestor {
    type O = terrars::ListRef<BinaryAuthorizationAttestorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BinaryAuthorizationAttestor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_binary_authorization_attestor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBinaryAuthorizationAttestor {
    pub tf_id: std::string::String,
    #[doc = "The resource name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBinaryAuthorizationAttestor {
    pub fn build(self, stack: &mut terrars::Stack) -> BinaryAuthorizationAttestor {
        let out = BinaryAuthorizationAttestor(std::rc::Rc::new(BinaryAuthorizationAttestor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BinaryAuthorizationAttestorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                attestation_authority_note: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BinaryAuthorizationAttestorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationAttestorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BinaryAuthorizationAttestorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA descriptive comment. This field may be updated. The field may be\ndisplayed in chooser dialogs."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attestation_authority_note` after provisioning.\n"]
    pub fn attestation_authority_note(
        &self,
    ) -> terrars::ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation_authority_note", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationAttestorTimeoutsElRef {
        BinaryAuthorizationAttestorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key_pem: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signature_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    #[doc =
        "Set the field `public_key_pem`.\nA PEM-encoded public key, as described in\n'https://tools.ietf.org/html/rfc7468#section-13'"]
    pub fn set_public_key_pem(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key_pem = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `signature_algorithm`.\nThe signature algorithm used to verify a message against\na signature using this key. These signature algorithm must\nmatch the structure and any object identifiers encoded in\npublicKeyPem (i.e. this algorithm must match that of the\npublic key)."]
    pub fn set_signature_algorithm(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.signature_algorithm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    type O =
        terrars::BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {}

impl BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
    pub fn build(self) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl {
            public_key_pem: core::default::Default::default(),
            signature_algorithm: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_key_pem` after provisioning.\nA PEM-encoded public key, as described in\n'https://tools.ietf.org/html/rfc7468#section-13'"]
    pub fn public_key_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_pem", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signature_algorithm` after provisioning.\nThe signature algorithm used to verify a message against\na signature using this key. These signature algorithm must\nmatch the structure and any object identifiers encoded in\npublicKeyPem (i.e. this algorithm must match that of the\npublic key)."]
    pub fn signature_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature_algorithm", self.base))
    }
}

#[derive(Serialize, Default)]
struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElDynamic {
    pkix_public_key: std
    ::option
    ::Option<
        terrars::DynamicBlock<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl>,
    >,
}

#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ascii_armored_pgp_public_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pkix_public_key: std
    ::option
    ::Option<std::vec::Vec<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl>>,
    dynamic: BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElDynamic,
}

impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    #[doc =
        "Set the field `ascii_armored_pgp_public_key`.\nASCII-armored representation of a PGP public key, as the\nentire output by the command\n'gpg --export --armor foo@example.com' (either LF or CRLF\nline endings). When using this field, id should be left\nblank. The BinAuthz API handlers will calculate the ID\nand fill it in automatically. BinAuthz computes this ID\nas the OpenPGP RFC4880 V4 fingerprint, represented as\nupper-case hex. If id is provided by the caller, it will\nbe overwritten by the API-calculated ID."]
    pub fn set_ascii_armored_pgp_public_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ascii_armored_pgp_public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment`.\nA descriptive comment. This field may be updated."]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `id`.\nThe ID of this public key. Signatures verified by BinAuthz\nmust include the ID of the public key that can be used to\nverify them, and that ID must match the contents of this\nfield exactly. Additional restrictions on this field can\nbe imposed based on which public key type is encapsulated.\nSee the documentation on publicKey cases below for details."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pkix_public_key`.\n"]
    pub fn set_pkix_public_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pkix_public_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pkix_public_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {}

impl BuildBinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
    pub fn build(self) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl {
            ascii_armored_pgp_public_key: core::default::Default::default(),
            comment: core::default::Default::default(),
            id: core::default::Default::default(),
            pkix_public_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ascii_armored_pgp_public_key` after provisioning.\nASCII-armored representation of a PGP public key, as the\nentire output by the command\n'gpg --export --armor foo@example.com' (either LF or CRLF\nline endings). When using this field, id should be left\nblank. The BinAuthz API handlers will calculate the ID\nand fill it in automatically. BinAuthz computes this ID\nas the OpenPGP RFC4880 V4 fingerprint, represented as\nupper-case hex. If id is provided by the caller, it will\nbe overwritten by the API-calculated ID."]
    pub fn ascii_armored_pgp_public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ascii_armored_pgp_public_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `comment` after provisioning.\nA descriptive comment. This field may be updated."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of this public key. Signatures verified by BinAuthz\nmust include the ID of the public key that can be used to\nverify them, and that ID must match the contents of this\nfield exactly. Additional restrictions on this field can\nbe imposed based on which public key type is encapsulated.\nSee the documentation on publicKey cases below for details."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `pkix_public_key` after provisioning.\n"]
    pub fn pkix_public_key(
        &self,
    ) -> terrars::ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElPkixPublicKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pkix_public_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct BinaryAuthorizationAttestorAttestationAuthorityNoteElDynamic {
    public_keys: std
    ::option
    ::Option<terrars::DynamicBlock<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>>,
}

#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    note_reference: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_keys: std::option::Option<std::vec::Vec<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>>,
    dynamic: BinaryAuthorizationAttestorAttestationAuthorityNoteElDynamic,
}

impl BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    #[doc = "Set the field `public_keys`.\n"]
    pub fn set_public_keys(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_keys = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    #[doc =
        "The resource name of a ATTESTATION_AUTHORITY Note, created by the\nuser. If the Note is in a different project from the Attestor, it\nshould be specified in the format 'projects/*/notes/*' (or the legacy\n'providers/*/notes/*'). This field may not be updated.\nAn attestation by this attestor is stored as a Container Analysis\nATTESTATION_AUTHORITY Occurrence that names a container image\nand that links to this Note."]
    pub note_reference: terrars::PrimField<std::string::String>,
}

impl BuildBinaryAuthorizationAttestorAttestationAuthorityNoteEl {
    pub fn build(self) -> BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
        BinaryAuthorizationAttestorAttestationAuthorityNoteEl {
            note_reference: self.note_reference,
            public_keys: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
        BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationAttestorAttestationAuthorityNoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delegation_service_account_email` after provisioning.\nThis field will contain the service account email address that\nthis Attestor will use as the principal when querying Container\nAnalysis. Attestor administrators must grant this service account\nthe IAM role needed to read attestations from the noteReference in\nContainer Analysis (containeranalysis.notes.occurrences.viewer).\nThis email address is fixed for the lifetime of the Attestor, but\ncallers should not make any other assumptions about the service\naccount email; future versions may use an email based on a\ndifferent naming pattern."]
    pub fn delegation_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegation_service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `note_reference` after provisioning.\nThe resource name of a ATTESTATION_AUTHORITY Note, created by the\nuser. If the Note is in a different project from the Attestor, it\nshould be specified in the format 'projects/*/notes/*' (or the legacy\n'providers/*/notes/*'). This field may not be updated.\nAn attestation by this attestor is stored as a Container Analysis\nATTESTATION_AUTHORITY Occurrence that names a container image\nand that links to this Note."]
    pub fn note_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.note_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `public_keys` after provisioning.\n"]
    pub fn public_keys(&self) -> terrars::ListRef<BinaryAuthorizationAttestorAttestationAuthorityNoteElPublicKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct BinaryAuthorizationAttestorTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BinaryAuthorizationAttestorTimeoutsEl {
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

impl terrars::ToListMappable for BinaryAuthorizationAttestorTimeoutsEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationAttestorTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationAttestorTimeoutsEl {}

impl BuildBinaryAuthorizationAttestorTimeoutsEl {
    pub fn build(self) -> BinaryAuthorizationAttestorTimeoutsEl {
        BinaryAuthorizationAttestorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationAttestorTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationAttestorTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BinaryAuthorizationAttestorTimeoutsElRef {
        BinaryAuthorizationAttestorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationAttestorTimeoutsElRef {
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
struct BinaryAuthorizationAttestorDynamic {
    attestation_authority_note: std
    ::option
    ::Option<terrars::DynamicBlock<BinaryAuthorizationAttestorAttestationAuthorityNoteEl>>,
}
