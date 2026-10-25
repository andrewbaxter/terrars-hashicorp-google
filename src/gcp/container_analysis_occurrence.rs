#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAnalysisOccurrenceData_ {
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
    note_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remediation: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attestation: std::option::Option<std::vec::Vec<ContainerAnalysisOccurrence_Attestation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAnalysisOccurrence_Timeouts>,
    dynamic: ContainerAnalysisOccurrenceDynamic,
}

struct ContainerAnalysisOccurrence_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAnalysisOccurrenceData_>,
}

#[derive(Clone)]
pub struct ContainerAnalysisOccurrence(std::rc::Rc<ContainerAnalysisOccurrence_>);

impl ContainerAnalysisOccurrence {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remediation`.\nA description of actions that can be taken to remedy the note."]
    pub fn set_remediation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().remediation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attestation`.\n"]
    pub fn set_attestation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAnalysisOccurrence_Attestation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAnalysisOccurrence_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe note kind which explicitly denotes which of the occurrence\ndetails are specified. This field can be used as a filter in list\nrequests."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the occurrence."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `note_name` after provisioning.\nThe analysis note associated with this occurrence, in the form of\nprojects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a\nfilter in list requests."]
    pub fn note_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.note_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remediation` after provisioning.\nA description of actions that can be taken to remedy the note."]
    pub fn remediation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_uri` after provisioning.\nRequired. Immutable. A URI that represents the resource for which\nthe occurrence applies. For example,\nhttps://gcr.io/project/image@sha256:123abc for a Docker image."]
    pub fn resource_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attestation` after provisioning.\n"]
    pub fn attestation(&self) -> terrars::ListRef<ContainerAnalysisOccurrence_AttestationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisOccurrence_TimeoutsRef {
        ContainerAnalysisOccurrence_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ContainerAnalysisOccurrence {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAnalysisOccurrence { }

impl terrars::ToListMappable for ContainerAnalysisOccurrence {
    type O = terrars::ListRef<ContainerAnalysisOccurrenceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAnalysisOccurrence_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_analysis_occurrence".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAnalysisOccurrence {
    pub tf_id: std::string::String,
    #[doc =
        "The analysis note associated with this occurrence, in the form of\nprojects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a\nfilter in list requests."]
    pub note_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Immutable. A URI that represents the resource for which\nthe occurrence applies. For example,\nhttps://gcr.io/project/image@sha256:123abc for a Docker image."]
    pub resource_uri: terrars::PrimField<std::string::String>,
}

impl BuildContainerAnalysisOccurrence {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAnalysisOccurrence {
        let out = ContainerAnalysisOccurrence(std::rc::Rc::new(ContainerAnalysisOccurrence_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAnalysisOccurrenceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                note_name: self.note_name,
                project: core::default::Default::default(),
                remediation: core::default::Default::default(),
                resource_uri: self.resource_uri,
                attestation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerAnalysisOccurrenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisOccurrenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAnalysisOccurrenceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the repository was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe note kind which explicitly denotes which of the occurrence\ndetails are specified. This field can be used as a filter in list\nrequests."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the occurrence."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `note_name` after provisioning.\nThe analysis note associated with this occurrence, in the form of\nprojects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a\nfilter in list requests."]
    pub fn note_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.note_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remediation` after provisioning.\nA description of actions that can be taken to remedy the note."]
    pub fn remediation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_uri` after provisioning.\nRequired. Immutable. A URI that represents the resource for which\nthe occurrence applies. For example,\nhttps://gcr.io/project/image@sha256:123abc for a Docker image."]
    pub fn resource_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the repository was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attestation` after provisioning.\n"]
    pub fn attestation(&self) -> terrars::ListRef<ContainerAnalysisOccurrence_AttestationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisOccurrence_TimeoutsRef {
        ContainerAnalysisOccurrence_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ContainerAnalysisOccurrenceDynamic {
    attestation: std::option::Option<terrars::DynamicBlock<ContainerAnalysisOccurrence_Attestation>>,
}

#[derive(Serialize)]
pub struct ContainerAnalysisOccurrence_Attestation_Signatures {
    public_key_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signature: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAnalysisOccurrence_Attestation_Signatures {
    #[doc =
        "Set the field `signature`.\nThe content of the signature, an opaque bytestring.\nThe payload that this signature verifies MUST be\nunambiguously provided with the Signature during\nverification. A wrapper message might provide the\npayload explicitly. Alternatively, a message might\nhave a canonical serialization that can always be\nunambiguously computed to derive the payload."]
    pub fn set_signature(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signature = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAnalysisOccurrence_Attestation_Signatures {
    type O = terrars::BlockAssignable<ContainerAnalysisOccurrence_Attestation_Signatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisOccurrence_Attestation_Signatures {
    #[doc =
        "The identifier for the public key that verifies this\nsignature. MUST be an RFC3986 conformant\nURI. * When possible, the key id should be an\nimmutable reference, such as a cryptographic digest.\nExamples of valid values:\n\n* OpenPGP V4 public key fingerprint. See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr\n  for more details on this scheme.\n    * 'openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA'\n* RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization):\n    * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\""]
    pub public_key_id: terrars::PrimField<std::string::String>,
}

impl BuildContainerAnalysisOccurrence_Attestation_Signatures {
    pub fn build(self) -> ContainerAnalysisOccurrence_Attestation_Signatures {
        ContainerAnalysisOccurrence_Attestation_Signatures {
            public_key_id: self.public_key_id,
            signature: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAnalysisOccurrence_Attestation_SignaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisOccurrence_Attestation_SignaturesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAnalysisOccurrence_Attestation_SignaturesRef {
        ContainerAnalysisOccurrence_Attestation_SignaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisOccurrence_Attestation_SignaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_key_id` after provisioning.\nThe identifier for the public key that verifies this\nsignature. MUST be an RFC3986 conformant\nURI. * When possible, the key id should be an\nimmutable reference, such as a cryptographic digest.\nExamples of valid values:\n\n* OpenPGP V4 public key fingerprint. See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr\n  for more details on this scheme.\n    * 'openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA'\n* RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization):\n    * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\""]
    pub fn public_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signature` after provisioning.\nThe content of the signature, an opaque bytestring.\nThe payload that this signature verifies MUST be\nunambiguously provided with the Signature during\nverification. A wrapper message might provide the\npayload explicitly. Alternatively, a message might\nhave a canonical serialization that can always be\nunambiguously computed to derive the payload."]
    pub fn signature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signature", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAnalysisOccurrence_AttestationDynamic {
    signatures: std::option::Option<terrars::DynamicBlock<ContainerAnalysisOccurrence_Attestation_Signatures>>,
}

#[derive(Serialize)]
pub struct ContainerAnalysisOccurrence_Attestation {
    serialized_payload: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signatures: std::option::Option<std::vec::Vec<ContainerAnalysisOccurrence_Attestation_Signatures>>,
    dynamic: ContainerAnalysisOccurrence_AttestationDynamic,
}

impl ContainerAnalysisOccurrence_Attestation {
    #[doc = "Set the field `signatures`.\n"]
    pub fn set_signatures(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAnalysisOccurrence_Attestation_Signatures>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.signatures = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.signatures = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAnalysisOccurrence_Attestation {
    type O = terrars::BlockAssignable<ContainerAnalysisOccurrence_Attestation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisOccurrence_Attestation {
    #[doc = "The serialized payload that is verified by one or\nmore signatures. A base64-encoded string."]
    pub serialized_payload: terrars::PrimField<std::string::String>,
}

impl BuildContainerAnalysisOccurrence_Attestation {
    pub fn build(self) -> ContainerAnalysisOccurrence_Attestation {
        ContainerAnalysisOccurrence_Attestation {
            serialized_payload: self.serialized_payload,
            signatures: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAnalysisOccurrence_AttestationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisOccurrence_AttestationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAnalysisOccurrence_AttestationRef {
        ContainerAnalysisOccurrence_AttestationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisOccurrence_AttestationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `serialized_payload` after provisioning.\nThe serialized payload that is verified by one or\nmore signatures. A base64-encoded string."]
    pub fn serialized_payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialized_payload", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAnalysisOccurrence_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAnalysisOccurrence_Timeouts {
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

impl terrars::ToListMappable for ContainerAnalysisOccurrence_Timeouts {
    type O = terrars::BlockAssignable<ContainerAnalysisOccurrence_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisOccurrence_Timeouts {}

impl BuildContainerAnalysisOccurrence_Timeouts {
    pub fn build(self) -> ContainerAnalysisOccurrence_Timeouts {
        ContainerAnalysisOccurrence_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ContainerAnalysisOccurrence_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisOccurrence_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAnalysisOccurrence_TimeoutsRef {
        ContainerAnalysisOccurrence_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisOccurrence_TimeoutsRef {
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
