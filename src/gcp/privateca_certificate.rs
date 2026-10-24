use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PrivatecaCertificateData {
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
    certificate_authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_csr: std::option::Option<terrars::PrimField<std::string::String>>,
    pool: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrivatecaCertificateTimeoutsEl>,
    dynamic: PrivatecaCertificateDynamic,
}

struct PrivatecaCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrivatecaCertificateData>,
}

#[derive(Clone)]
pub struct PrivatecaCertificate(std::rc::Rc<PrivatecaCertificate_>);

impl PrivatecaCertificate {
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
        "Set the field `certificate_authority`.\nThe Certificate Authority ID that should issue the certificate. For example, to issue a Certificate from\na Certificate Authority with resource name 'projects/my-project/locations/us-central1/caPools/my-pool/certificateAuthorities/my-ca',\nargument 'pool' should be set to 'projects/my-project/locations/us-central1/caPools/my-pool', argument 'certificate_authority'\nshould be set to 'my-ca'."]
    pub fn set_certificate_authority(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_authority = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `certificate_template`.\nThe resource name for a CertificateTemplate used to issue this certificate,\nin the format 'projects/*/locations/*/certificateTemplates/*'. If this is specified,\nthe caller must have the necessary permission to use this template. If this is\nomitted, no template will be used. This template must be in the same location\nas the Certificate."]
    pub fn set_certificate_template(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lifetime`.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_lifetime(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pem_csr`.\nImmutable. A pem-encoded X.509 certificate signing request (CSR)."]
    pub fn set_pem_csr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pem_csr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrivatecaCertificateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nThe Certificate Authority ID that should issue the certificate. For example, to issue a Certificate from\na Certificate Authority with resource name 'projects/my-project/locations/us-central1/caPools/my-pool/certificateAuthorities/my-ca',\nargument 'pool' should be set to 'projects/my-project/locations/us-central1/caPools/my-pool', argument 'certificate_authority'\nshould be set to 'my-ca'."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_description` after provisioning.\nOutput only. Details regarding the revocation of this Certificate. This Certificate is considered revoked if and only if this field is present."]
    pub fn certificate_description(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_template` after provisioning.\nThe resource name for a CertificateTemplate used to issue this certificate,\nin the format 'projects/*/locations/*/certificateTemplates/*'. If this is specified,\nthe caller must have the necessary permission to use this template. If this is\nomitted, no template will be used. This template must be in the same location\nas the Certificate."]
    pub fn certificate_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time that this resource was created on the server.\nThis is in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issuer_certificate_authority` after provisioning.\nThe resource name of the issuing CertificateAuthority in the format 'projects/*/locations/*/caPools/*/certificateAuthorities/*'."]
    pub fn issuer_certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the Certificate. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this Certificate."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nOutput only. The pem-encoded, signed X.509 certificate."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate_chain` after provisioning.\nThe chain that may be used to verify the X.509 certificate. Expected to be in issuer-to-root order according to RFC 5246."]
    pub fn pem_certificate_chain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pem_certificate_chain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_csr` after provisioning.\nImmutable. A pem-encoded X.509 certificate signing request (CSR)."]
    pub fn pem_csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_csr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool` after provisioning.\nThe name of the CaPool this Certificate belongs to."]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revocation_details` after provisioning.\nOutput only. Details regarding the revocation of this Certificate. This Certificate is\nconsidered revoked if and only if this field is present."]
    pub fn revocation_details(&self) -> terrars::ListRef<PrivatecaCertificateRevocationDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateAuthority was updated.\nThis is in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<PrivatecaCertificateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTimeoutsElRef {
        PrivatecaCertificateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PrivatecaCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrivatecaCertificate { }

impl terrars::ToListMappable for PrivatecaCertificate {
    type O = terrars::ListRef<PrivatecaCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrivatecaCertificate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_privateca_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrivatecaCertificate {
    pub tf_id: std::string::String,
    #[doc =
        "Location of the Certificate. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name for this Certificate."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the CaPool this Certificate belongs to."]
    pub pool: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> PrivatecaCertificate {
        let out = PrivatecaCertificate(std::rc::Rc::new(PrivatecaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrivatecaCertificateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                certificate_authority: core::default::Default::default(),
                certificate_template: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                lifetime: core::default::Default::default(),
                location: self.location,
                name: self.name,
                pem_csr: core::default::Default::default(),
                pool: self.pool,
                project: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrivatecaCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrivatecaCertificateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority` after provisioning.\nThe Certificate Authority ID that should issue the certificate. For example, to issue a Certificate from\na Certificate Authority with resource name 'projects/my-project/locations/us-central1/caPools/my-pool/certificateAuthorities/my-ca',\nargument 'pool' should be set to 'projects/my-project/locations/us-central1/caPools/my-pool', argument 'certificate_authority'\nshould be set to 'my-ca'."]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_description` after provisioning.\nOutput only. Details regarding the revocation of this Certificate. This Certificate is considered revoked if and only if this field is present."]
    pub fn certificate_description(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_template` after provisioning.\nThe resource name for a CertificateTemplate used to issue this certificate,\nin the format 'projects/*/locations/*/certificateTemplates/*'. If this is specified,\nthe caller must have the necessary permission to use this template. If this is\nomitted, no template will be used. This template must be in the same location\nas the Certificate."]
    pub fn certificate_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time that this resource was created on the server.\nThis is in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issuer_certificate_authority` after provisioning.\nThe resource name of the issuing CertificateAuthority in the format 'projects/*/locations/*/caPools/*/certificateAuthorities/*'."]
    pub fn issuer_certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer_certificate_authority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata to apply to this resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the Certificate. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name for this Certificate."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate` after provisioning.\nOutput only. The pem-encoded, signed X.509 certificate."]
    pub fn pem_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_certificate_chain` after provisioning.\nThe chain that may be used to verify the X.509 certificate. Expected to be in issuer-to-root order according to RFC 5246."]
    pub fn pem_certificate_chain(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pem_certificate_chain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_csr` after provisioning.\nImmutable. A pem-encoded X.509 certificate signing request (CSR)."]
    pub fn pem_csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_csr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool` after provisioning.\nThe name of the CaPool this Certificate belongs to."]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revocation_details` after provisioning.\nOutput only. Details regarding the revocation of this Certificate. This Certificate is\nconsidered revoked if and only if this field is present."]
    pub fn revocation_details(&self) -> terrars::ListRef<PrivatecaCertificateRevocationDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which this CertificateAuthority was updated.\nThis is in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<PrivatecaCertificateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivatecaCertificateTimeoutsElRef {
        PrivatecaCertificateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl {
        PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl { key_id: core::default::Default::default() }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdElRef {
        PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElCertFingerprintEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_hash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElCertFingerprintEl {
    #[doc = "Set the field `sha256_hash`.\n"]
    pub fn set_sha256_hash(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_hash = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElCertFingerprintEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElCertFingerprintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElCertFingerprintEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElCertFingerprintEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElCertFingerprintEl {
        PrivatecaCertificateCertificateDescriptionElCertFingerprintEl {
            sha256_hash: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElCertFingerprintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElCertFingerprintElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElCertFingerprintElRef {
        PrivatecaCertificateCertificateDescriptionElCertFingerprintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElCertFingerprintElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sha256_hash` after provisioning.\n"]
    pub fn sha256_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElPublicKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElPublicKeyEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElPublicKeyEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElPublicKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElPublicKeyEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElPublicKeyEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElPublicKeyEl {
        PrivatecaCertificateCertificateDescriptionElPublicKeyEl {
            format: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElPublicKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElPublicKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElPublicKeyElRef {
        PrivatecaCertificateCertificateDescriptionElPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElPublicKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    province: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    street_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {
    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\n"]
    pub fn set_locality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organization`.\n"]
    pub fn set_organization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\n"]
    pub fn set_organizational_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\n"]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\n"]
    pub fn set_province(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.province = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `street_address`.\n"]
    pub fn set_street_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.street_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl {
            common_name: core::default::Default::default(),
            country_code: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            organizational_unit: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            street_address: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectElRef {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc = "Get a reference to the value of field `locality` after provisioning.\n"]
    pub fn locality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\n"]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc = "Get a reference to the value of field `province` after provisioning.\n"]
    pub fn province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc = "Get a reference to the value of field `street_address` after provisioning.\n"]
    pub fn street_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.street_address", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {
    #[doc = "Set the field `object_id_path`.\n"]
    pub fn set_object_id_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.object_id_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {
    type O =
        terrars
        ::BlockAssignable<
            PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdElRef {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    critical: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    obect_id: std
    ::option
    ::Option<
        terrars
        ::ListField<
            PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {
    #[doc = "Set the field `critical`.\n"]
    pub fn set_critical(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.critical = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `obect_id`.\n"]
    pub fn set_obect_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdEl,
                        >,
                    >,
    ) -> Self {
        self.obect_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {
    type O =
        terrars
        ::BlockAssignable<
            PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl {
            critical: core::default::Default::default(),
            obect_id: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElRef {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `critical` after provisioning.\n"]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc = "Get a reference to the value of field `obect_id` after provisioning.\n"]
    pub fn obect_id(
        &self,
    ) -> terrars
    ::ListRef<
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElObectIdElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.obect_id", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_sans: std
    ::option
    ::Option<
        terrars
        ::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {
    #[doc = "Set the field `custom_sans`.\n"]
    pub fn set_custom_sans(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansEl,
                        >,
                    >,
    ) -> Self {
        self.custom_sans = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_names`.\n"]
    pub fn set_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_addresses`.\n"]
    pub fn set_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uris`.\n"]
    pub fn set_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {
    type O =
        terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl {
            custom_sans: core::default::Default::default(),
            dns_names: core::default::Default::default(),
            email_addresses: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
            uris: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElRef {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_sans` after provisioning.\n"]
    pub fn custom_sans(
        &self,
    ) -> terrars
    ::ListRef<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElCustomSansElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_sans", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_names` after provisioning.\n"]
    pub fn dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_names", self.base))
    }

    #[doc = "Get a reference to the value of field `email_addresses` after provisioning.\n"]
    pub fn email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\n"]
    pub fn uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hex_serial_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifetime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_after_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_before_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_name: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl>>,
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {
    #[doc = "Set the field `hex_serial_number`.\n"]
    pub fn set_hex_serial_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hex_serial_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifetime`.\n"]
    pub fn set_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_after_time`.\n"]
    pub fn set_not_after_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.not_after_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_before_time`.\n"]
    pub fn set_not_before_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.not_before_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectEl>,
                    >,
    ) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alt_name`.\n"]
    pub fn set_subject_alt_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameEl>,
                    >,
    ) -> Self {
        self.subject_alt_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl {
            hex_serial_number: core::default::Default::default(),
            lifetime: core::default::Default::default(),
            not_after_time: core::default::Default::default(),
            not_before_time: core::default::Default::default(),
            subject: core::default::Default::default(),
            subject_alt_name: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElRef {
        PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hex_serial_number` after provisioning.\n"]
    pub fn hex_serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hex_serial_number", self.base))
    }

    #[doc = "Get a reference to the value of field `lifetime` after provisioning.\n"]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.base))
    }

    #[doc = "Get a reference to the value of field `not_after_time` after provisioning.\n"]
    pub fn not_after_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after_time", self.base))
    }

    #[doc = "Get a reference to the value of field `not_before_time` after provisioning.\n"]
    pub fn not_before_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before_time", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alt_name` after provisioning.\n"]
    pub fn subject_alt_name(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElSubjectAltNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_name", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl {
        PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl { key_id: core::default::Default::default() }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElSubjectKeyIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElSubjectKeyIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElSubjectKeyIdElRef {
        PrivatecaCertificateCertificateDescriptionElSubjectKeyIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElSubjectKeyIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {
    #[doc = "Set the field `object_id_path`.\n"]
    pub fn set_object_id_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.object_id_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {
    type O =
        terrars
        ::BlockAssignable<
            PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    critical: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<
        terrars
        ::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {
    #[doc = "Set the field `critical`.\n"]
    pub fn set_critical(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.critical = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdEl,
                        >,
                    >,
    ) -> Self {
        self.object_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {
    type O =
        terrars
        ::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl {
            critical: core::default::Default::default(),
            object_id: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `critical` after provisioning.\n"]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc = "Get a reference to the value of field `object_id` after provisioning.\n"]
    pub fn object_id(
        &self,
    ) -> terrars
    ::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElObjectIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {
    #[doc = "Set the field `is_ca`.\n"]
    pub fn set_is_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_issuer_path_length`.\n"]
    pub fn set_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_ca` after provisioning.\n"]
    pub fn is_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `max_issuer_path_length` after provisioning.\n"]
    pub fn max_issuer_path_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_issuer_path_length", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_commitment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_encipherment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decipher_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_signature: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encipher_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_agreement: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_encipherment: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {
    #[doc = "Set the field `cert_sign`.\n"]
    pub fn set_cert_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cert_sign = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_commitment`.\n"]
    pub fn set_content_commitment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.content_commitment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crl_sign`.\n"]
    pub fn set_crl_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.crl_sign = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_encipherment`.\n"]
    pub fn set_data_encipherment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_encipherment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decipher_only`.\n"]
    pub fn set_decipher_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.decipher_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digital_signature`.\n"]
    pub fn set_digital_signature(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.digital_signature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encipher_only`.\n"]
    pub fn set_encipher_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encipher_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_agreement`.\n"]
    pub fn set_key_agreement(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_agreement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_encipherment`.\n"]
    pub fn set_key_encipherment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_encipherment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {
    type O =
        terrars
        ::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl {
            cert_sign: core::default::Default::default(),
            content_commitment: core::default::Default::default(),
            crl_sign: core::default::Default::default(),
            data_encipherment: core::default::Default::default(),
            decipher_only: core::default::Default::default(),
            digital_signature: core::default::Default::default(),
            encipher_only: core::default::Default::default(),
            key_agreement: core::default::Default::default(),
            key_encipherment: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert_sign` after provisioning.\n"]
    pub fn cert_sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_sign", self.base))
    }

    #[doc = "Get a reference to the value of field `content_commitment` after provisioning.\n"]
    pub fn content_commitment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_commitment", self.base))
    }

    #[doc = "Get a reference to the value of field `crl_sign` after provisioning.\n"]
    pub fn crl_sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crl_sign", self.base))
    }

    #[doc = "Get a reference to the value of field `data_encipherment` after provisioning.\n"]
    pub fn data_encipherment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_encipherment", self.base))
    }

    #[doc = "Get a reference to the value of field `decipher_only` after provisioning.\n"]
    pub fn decipher_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decipher_only", self.base))
    }

    #[doc = "Get a reference to the value of field `digital_signature` after provisioning.\n"]
    pub fn digital_signature(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digital_signature", self.base))
    }

    #[doc = "Get a reference to the value of field `encipher_only` after provisioning.\n"]
    pub fn encipher_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encipher_only", self.base))
    }

    #[doc = "Get a reference to the value of field `key_agreement` after provisioning.\n"]
    pub fn key_agreement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_agreement", self.base))
    }

    #[doc = "Get a reference to the value of field `key_encipherment` after provisioning.\n"]
    pub fn key_encipherment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_encipherment", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_signing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_signing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_stamping: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {
    #[doc = "Set the field `client_auth`.\n"]
    pub fn set_client_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_signing`.\n"]
    pub fn set_code_signing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.code_signing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_protection`.\n"]
    pub fn set_email_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.email_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocsp_signing`.\n"]
    pub fn set_ocsp_signing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ocsp_signing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_auth`.\n"]
    pub fn set_server_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_stamping`.\n"]
    pub fn set_time_stamping(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.time_stamping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {
    type O =
        terrars
        ::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_auth` after provisioning.\n"]
    pub fn client_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `code_signing` after provisioning.\n"]
    pub fn code_signing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing", self.base))
    }

    #[doc = "Get a reference to the value of field `email_protection` after provisioning.\n"]
    pub fn email_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_signing` after provisioning.\n"]
    pub fn ocsp_signing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsp_signing", self.base))
    }

    #[doc = "Get a reference to the value of field `server_auth` after provisioning.\n"]
    pub fn server_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `time_stamping` after provisioning.\n"]
    pub fn time_stamping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_stamping", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[doc = "Set the field `object_id_path`.\n"]
    pub fn set_object_id_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.object_id_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {
    type O =
        terrars
        ::BlockAssignable<
            PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(
        self,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std
    ::option
    ::Option<
        terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<
        terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<
        terrars
        ::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageEl,
                        >,
                    >,
    ) -> Self {
        self.base_key_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extended_key_usage`.\n"]
    pub fn set_extended_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageEl,
                        >,
                    >,
    ) -> Self {
        self.extended_key_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unknown_extended_key_usages`.\n"]
    pub fn set_unknown_extended_key_usages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesEl,
                        >,
                    >,
    ) -> Self {
        self.unknown_extended_key_usages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElBaseKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars
    ::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElExtendedKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars
    ::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElUnknownExtendedKeyUsagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    critical: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {
    #[doc = "Set the field `critical`.\n"]
    pub fn set_critical(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.critical = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_dns_names`.\n"]
    pub fn set_excluded_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_email_addresses`.\n"]
    pub fn set_excluded_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_ip_ranges`.\n"]
    pub fn set_excluded_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_uris`.\n"]
    pub fn set_excluded_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permitted_dns_names`.\n"]
    pub fn set_permitted_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permitted_email_addresses`.\n"]
    pub fn set_permitted_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permitted_ip_ranges`.\n"]
    pub fn set_permitted_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permitted_uris`.\n"]
    pub fn set_permitted_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {
    type O =
        terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl {
            critical: core::default::Default::default(),
            excluded_dns_names: core::default::Default::default(),
            excluded_email_addresses: core::default::Default::default(),
            excluded_ip_ranges: core::default::Default::default(),
            excluded_uris: core::default::Default::default(),
            permitted_dns_names: core::default::Default::default(),
            permitted_email_addresses: core::default::Default::default(),
            permitted_ip_ranges: core::default::Default::default(),
            permitted_uris: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `critical` after provisioning.\n"]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_dns_names` after provisioning.\n"]
    pub fn excluded_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_dns_names", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_email_addresses` after provisioning.\n"]
    pub fn excluded_email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_email_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_ip_ranges` after provisioning.\n"]
    pub fn excluded_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_uris` after provisioning.\n"]
    pub fn excluded_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `permitted_dns_names` after provisioning.\n"]
    pub fn permitted_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_dns_names", self.base))
    }

    #[doc = "Get a reference to the value of field `permitted_email_addresses` after provisioning.\n"]
    pub fn permitted_email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_email_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `permitted_ip_ranges` after provisioning.\n"]
    pub fn permitted_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `permitted_uris` after provisioning.\n"]
    pub fn permitted_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {
    #[doc = "Set the field `object_id_path`.\n"]
    pub fn set_object_id_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.object_id_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_constraints: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl>>,
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionEl {
    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsEl,
                        >,
                    >,
    ) -> Self {
        self.additional_extensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aia_ocsp_servers`.\n"]
    pub fn set_aia_ocsp_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aia_ocsp_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ca_options`.\n"]
    pub fn set_ca_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsEl>,
                    >,
    ) -> Self {
        self.ca_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_usage`.\n"]
    pub fn set_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageEl>,
                    >,
    ) -> Self {
        self.key_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_constraints`.\n"]
    pub fn set_name_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsEl>,
                    >,
    ) -> Self {
        self.name_constraints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_ids`.\n"]
    pub fn set_policy_ids(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsEl>,
                    >,
    ) -> Self {
        self.policy_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionElX509DescriptionEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionElX509DescriptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionEl {}

impl BuildPrivatecaCertificateCertificateDescriptionElX509DescriptionEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionEl {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionEl {
            additional_extensions: core::default::Default::default(),
            aia_ocsp_servers: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            name_constraints: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElX509DescriptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElX509DescriptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElX509DescriptionElRef {
        PrivatecaCertificateCertificateDescriptionElX509DescriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElX509DescriptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElAdditionalExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\n"]
    pub fn aia_ocsp_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aia_ocsp_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElCaOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElNameConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElPolicyIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateCertificateDescriptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_issuing_certificate_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authority_key_id: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_fingerprint: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElCertFingerprintEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_distribution_points: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElPublicKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_description: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_key_id: std::option::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_description: std
    ::option
    ::Option<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionEl>>,
}

impl PrivatecaCertificateCertificateDescriptionEl {
    #[doc = "Set the field `aia_issuing_certificate_urls`.\n"]
    pub fn set_aia_issuing_certificate_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aia_issuing_certificate_urls = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authority_key_id`.\n"]
    pub fn set_authority_key_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdEl>>,
    ) -> Self {
        self.authority_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cert_fingerprint`.\n"]
    pub fn set_cert_fingerprint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<PrivatecaCertificateCertificateDescriptionElCertFingerprintEl>>,
    ) -> Self {
        self.cert_fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crl_distribution_points`.\n"]
    pub fn set_crl_distribution_points(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.crl_distribution_points = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<PrivatecaCertificateCertificateDescriptionElPublicKeyEl>>,
    ) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_description`.\n"]
    pub fn set_subject_description(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionEl>>,
    ) -> Self {
        self.subject_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_key_id`.\n"]
    pub fn set_subject_key_id(
        mut self,
        v: impl std::convert::Into<terrars::ListField<PrivatecaCertificateCertificateDescriptionElSubjectKeyIdEl>>,
    ) -> Self {
        self.subject_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x509_description`.\n"]
    pub fn set_x509_description(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<PrivatecaCertificateCertificateDescriptionElX509DescriptionEl>>,
    ) -> Self {
        self.x509_description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateCertificateDescriptionEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateCertificateDescriptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateCertificateDescriptionEl {}

impl BuildPrivatecaCertificateCertificateDescriptionEl {
    pub fn build(self) -> PrivatecaCertificateCertificateDescriptionEl {
        PrivatecaCertificateCertificateDescriptionEl {
            aia_issuing_certificate_urls: core::default::Default::default(),
            authority_key_id: core::default::Default::default(),
            cert_fingerprint: core::default::Default::default(),
            crl_distribution_points: core::default::Default::default(),
            public_key: core::default::Default::default(),
            subject_description: core::default::Default::default(),
            subject_key_id: core::default::Default::default(),
            x509_description: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateCertificateDescriptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateCertificateDescriptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateCertificateDescriptionElRef {
        PrivatecaCertificateCertificateDescriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateCertificateDescriptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aia_issuing_certificate_urls` after provisioning.\n"]
    pub fn aia_issuing_certificate_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aia_issuing_certificate_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `authority_key_id` after provisioning.\n"]
    pub fn authority_key_id(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElAuthorityKeyIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authority_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cert_fingerprint` after provisioning.\n"]
    pub fn cert_fingerprint(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElCertFingerprintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cert_fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `crl_distribution_points` after provisioning.\n"]
    pub fn crl_distribution_points(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crl_distribution_points", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElPublicKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_description` after provisioning.\n"]
    pub fn subject_description(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElSubjectDescriptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_description", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_key_id` after provisioning.\n"]
    pub fn subject_key_id(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElSubjectKeyIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `x509_description` after provisioning.\n"]
    pub fn x509_description(&self) -> terrars::ListRef<PrivatecaCertificateCertificateDescriptionElX509DescriptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509_description", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateRevocationDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revocation_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revocation_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateRevocationDetailsEl {
    #[doc = "Set the field `revocation_state`.\n"]
    pub fn set_revocation_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revocation_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revocation_time`.\n"]
    pub fn set_revocation_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revocation_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateRevocationDetailsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateRevocationDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateRevocationDetailsEl {}

impl BuildPrivatecaCertificateRevocationDetailsEl {
    pub fn build(self) -> PrivatecaCertificateRevocationDetailsEl {
        PrivatecaCertificateRevocationDetailsEl {
            revocation_state: core::default::Default::default(),
            revocation_time: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateRevocationDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateRevocationDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateRevocationDetailsElRef {
        PrivatecaCertificateRevocationDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateRevocationDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revocation_state` after provisioning.\n"]
    pub fn revocation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revocation_state", self.base))
    }

    #[doc = "Get a reference to the value of field `revocation_time` after provisioning.\n"]
    pub fn revocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revocation_time", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElPublicKeyEl {
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateConfigElPublicKeyEl {
    #[doc =
        "Set the field `key`.\nRequired. A public key. When this is specified in a request, the padding and encoding can be any of the options described by the respective 'KeyType' value. When this is generated by the service, it will always be an RFC 5280 SubjectPublicKeyInfo structure containing an algorithm identifier and a key. A base64-encoded string."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElPublicKeyEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElPublicKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElPublicKeyEl {
    #[doc =
        "The format of the public key. Currently, only PEM format is supported. Possible values: [\"KEY_TYPE_UNSPECIFIED\", \"PEM\"]"]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateConfigElPublicKeyEl {
    pub fn build(self) -> PrivatecaCertificateConfigElPublicKeyEl {
        PrivatecaCertificateConfigElPublicKeyEl {
            format: self.format,
            key: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElPublicKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElPublicKeyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateConfigElPublicKeyElRef {
        PrivatecaCertificateConfigElPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElPublicKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nThe format of the public key. Currently, only PEM format is supported. Possible values: [\"KEY_TYPE_UNSPECIFIED\", \"PEM\"]"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nRequired. A public key. When this is specified in a request, the padding and encoding can be any of the options described by the respective 'KeyType' value. When this is generated by the service, it will always be an RFC 5280 SubjectPublicKeyInfo structure containing an algorithm identifier and a key. A base64-encoded string."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElSubjectConfigElSubjectEl {
    common_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality: std::option::Option<terrars::PrimField<std::string::String>>,
    organization: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    province: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    street_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateConfigElSubjectConfigElSubjectEl {
    #[doc = "Set the field `country_code`.\nThe country code of the subject."]
    pub fn set_country_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\nThe locality or city of the subject."]
    pub fn set_locality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\nThe organizational unit of the subject."]
    pub fn set_organizational_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_code`.\nThe postal code of the subject."]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `province`.\nThe province, territory, or regional state of the subject."]
    pub fn set_province(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.province = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `street_address`.\nThe street address of the subject."]
    pub fn set_street_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.street_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElSubjectConfigElSubjectEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElSubjectConfigElSubjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElSubjectConfigElSubjectEl {
    #[doc = "The common name of the distinguished name."]
    pub common_name: terrars::PrimField<std::string::String>,
    #[doc = "The organization of the subject."]
    pub organization: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateConfigElSubjectConfigElSubjectEl {
    pub fn build(self) -> PrivatecaCertificateConfigElSubjectConfigElSubjectEl {
        PrivatecaCertificateConfigElSubjectConfigElSubjectEl {
            common_name: self.common_name,
            country_code: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: self.organization,
            organizational_unit: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            province: core::default::Default::default(),
            street_address: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElSubjectConfigElSubjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElSubjectConfigElSubjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElSubjectConfigElSubjectElRef {
        PrivatecaCertificateConfigElSubjectConfigElSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElSubjectConfigElSubjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `common_name` after provisioning.\nThe common name of the distinguished name."]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `country_code` after provisioning.\nThe country code of the subject."]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `locality` after provisioning.\nThe locality or city of the subject."]
    pub fn locality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nThe organization of the subject."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organizational_unit` after provisioning.\nThe organizational unit of the subject."]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\nThe postal code of the subject."]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `province` after provisioning.\nThe province, territory, or regional state of the subject."]
    pub fn province(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.province", self.base))
    }

    #[doc =
        "Get a reference to the value of field `street_address` after provisioning.\nThe street address of the subject."]
    pub fn street_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.street_address", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {
    #[doc = "Set the field `dns_names`.\nContains only valid, fully-qualified host names."]
    pub fn set_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_addresses`.\nContains only valid RFC 2822 E-mail addresses."]
    pub fn set_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\nContains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses."]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uris`.\nContains only valid RFC 3986 URIs."]
    pub fn set_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {}

impl BuildPrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {
    pub fn build(self) -> PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {
        PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl {
            dns_names: core::default::Default::default(),
            email_addresses: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
            uris: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameElRef {
        PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dns_names` after provisioning.\nContains only valid, fully-qualified host names."]
    pub fn dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_addresses` after provisioning.\nContains only valid RFC 2822 E-mail addresses."]
    pub fn email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_addresses` after provisioning.\nContains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses."]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `uris` after provisioning.\nContains only valid RFC 3986 URIs."]
    pub fn uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uris", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateConfigElSubjectConfigElDynamic {
    subject: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElSubjectConfigElSubjectEl>>,
    subject_alt_name: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElSubjectConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElSubjectConfigElSubjectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_name: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl>>,
    dynamic: PrivatecaCertificateConfigElSubjectConfigElDynamic,
}

impl PrivatecaCertificateConfigElSubjectConfigEl {
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElSubjectConfigElSubjectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subject_alt_name`.\n"]
    pub fn set_subject_alt_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject_alt_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alt_name = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElSubjectConfigEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElSubjectConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElSubjectConfigEl {}

impl BuildPrivatecaCertificateConfigElSubjectConfigEl {
    pub fn build(self) -> PrivatecaCertificateConfigElSubjectConfigEl {
        PrivatecaCertificateConfigElSubjectConfigEl {
            subject: core::default::Default::default(),
            subject_alt_name: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElSubjectConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElSubjectConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElSubjectConfigElRef {
        PrivatecaCertificateConfigElSubjectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElSubjectConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::ListRef<PrivatecaCertificateConfigElSubjectConfigElSubjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alt_name` after provisioning.\n"]
    pub fn subject_alt_name(&self) -> terrars::ListRef<PrivatecaCertificateConfigElSubjectConfigElSubjectAltNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_name", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl { }

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
        PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
        PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElDynamic {
    object_id: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
    critical: terrars::PrimField<bool>,
    value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>>,
    dynamic: PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElDynamic,
}

impl PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
    #[doc = "Set the field `object_id`.\n"]
    pub fn set_object_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_id = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
    #[doc =
        "Indicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub critical: terrars::PrimField<bool>,
    #[doc = "The value of this X.509 extension. A base64-encoded string."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
        PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl {
            critical: self.critical,
            value: self.value,
            object_id: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElRef {
        PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not this extension is critical (i.e., if the client does not know how to\nhandle this extension, the client should consider this to be an error)."]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value of this X.509 extension. A base64-encoded string."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `object_id` after provisioning.\n"]
    pub fn object_id(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElCaOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_max_issuer_path_length: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCertificateConfigElX509ConfigElCaOptionsEl {
    #[doc = "Set the field `is_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn set_is_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_issuer_path_length`.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn set_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `non_ca`.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn set_non_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.non_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zero_max_issuer_path_length`.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElCaOptionsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElCaOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElCaOptionsEl {}

impl BuildPrivatecaCertificateConfigElX509ConfigElCaOptionsEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElCaOptionsEl {
        PrivatecaCertificateConfigElX509ConfigElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            non_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElCaOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElCaOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElCaOptionsElRef {
        PrivatecaCertificateConfigElX509ConfigElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElCaOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to true."]
    pub fn is_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_issuer_path_length` after provisioning.\nRefers to the \"path length constraint\" in Basic Constraints extension. For a CA certificate, this value describes the depth of\nsubordinate CA certificates that are allowed. If this value is less than 0, the request will fail."]
    pub fn max_issuer_path_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_issuer_path_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `non_ca` after provisioning.\nWhen true, the \"CA\" in Basic Constraints extension will be set to false.\nIf both 'is_ca' and 'non_ca' are unset, the extension will be omitted from the CA certificate."]
    pub fn non_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_ca", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\nWhen true, the \"path length constraint\" in Basic Constraints extension will be set to 0.\nif both 'max_issuer_path_length' and 'zero_max_issuer_path_length' are unset,\nthe max path length will be omitted from the CA certificate."]
    pub fn zero_max_issuer_path_length(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_max_issuer_path_length", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_commitment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_sign: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_encipherment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decipher_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_signature: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encipher_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_agreement: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_encipherment: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    #[doc = "Set the field `cert_sign`.\nThe key may be used to sign certificates."]
    pub fn set_cert_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cert_sign = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_commitment`.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn set_content_commitment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.content_commitment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crl_sign`.\nThe key may be used sign certificate revocation lists."]
    pub fn set_crl_sign(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.crl_sign = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_encipherment`.\nThe key may be used to encipher data."]
    pub fn set_data_encipherment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.data_encipherment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decipher_only`.\nThe key may be used to decipher only."]
    pub fn set_decipher_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.decipher_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digital_signature`.\nThe key may be used for digital signatures."]
    pub fn set_digital_signature(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.digital_signature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encipher_only`.\nThe key may be used to encipher only."]
    pub fn set_encipher_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encipher_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_agreement`.\nThe key may be used in a key agreement protocol."]
    pub fn set_key_agreement(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_agreement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_encipherment`.\nThe key may be used to encipher other keys."]
    pub fn set_key_encipherment(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_encipherment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {}

impl BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
            cert_sign: core::default::Default::default(),
            content_commitment: core::default::Default::default(),
            crl_sign: core::default::Default::default(),
            data_encipherment: core::default::Default::default(),
            decipher_only: core::default::Default::default(),
            digital_signature: core::default::Default::default(),
            encipher_only: core::default::Default::default(),
            key_agreement: core::default::Default::default(),
            key_encipherment: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cert_sign` after provisioning.\nThe key may be used to sign certificates."]
    pub fn cert_sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_sign", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_commitment` after provisioning.\nThe key may be used for cryptographic commitments. Note that this may also be referred to as \"non-repudiation\"."]
    pub fn content_commitment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_commitment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crl_sign` after provisioning.\nThe key may be used sign certificate revocation lists."]
    pub fn crl_sign(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crl_sign", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_encipherment` after provisioning.\nThe key may be used to encipher data."]
    pub fn data_encipherment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_encipherment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `decipher_only` after provisioning.\nThe key may be used to decipher only."]
    pub fn decipher_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decipher_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `digital_signature` after provisioning.\nThe key may be used for digital signatures."]
    pub fn digital_signature(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digital_signature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encipher_only` after provisioning.\nThe key may be used to encipher only."]
    pub fn encipher_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encipher_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_agreement` after provisioning.\nThe key may be used in a key agreement protocol."]
    pub fn key_agreement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_agreement", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_encipherment` after provisioning.\nThe key may be used to encipher other keys."]
    pub fn key_encipherment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_encipherment", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_signing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_signing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_auth: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_stamping: std::option::Option<terrars::PrimField<bool>>,
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    #[doc =
        "Set the field `client_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn set_client_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `code_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn set_code_signing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.code_signing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `email_protection`.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn set_email_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.email_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ocsp_signing`.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn set_ocsp_signing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ocsp_signing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_auth`.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn set_server_auth(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_stamping`.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn set_time_stamping(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.time_stamping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {}

impl BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as \"TLS WWW client authentication\", though regularly used for non-WWW TLS."]
    pub fn client_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_auth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `code_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as \"Signing of downloadable executable code client authentication\"."]
    pub fn code_signing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_signing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email_protection` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as \"Email protection\"."]
    pub fn email_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_protection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocsp_signing` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as \"Signing OCSP responses\"."]
    pub fn ocsp_signing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsp_signing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_auth` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as \"TLS WWW server authentication\", though regularly used for non-WWW TLS."]
    pub fn server_auth(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_auth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_stamping` after provisioning.\nCorresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as \"Binding the hash of an object to a time\"."]
    pub fn time_stamping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_stamping", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl { }

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: self.object_id_path,
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElDynamic {
    base_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>>,
    extended_key_usage: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>>,
    unknown_extended_key_usages: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl>>,
    dynamic: PrivatecaCertificateConfigElX509ConfigElKeyUsageElDynamic,
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.base_key_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.base_key_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `extended_key_usage`.\n"]
    pub fn set_extended_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extended_key_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extended_key_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unknown_extended_key_usages`.\n"]
    pub fn set_unknown_extended_key_usages(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unknown_extended_key_usages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unknown_extended_key_usages = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElKeyUsageEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageEl {}

impl BuildPrivatecaCertificateConfigElX509ConfigElKeyUsageEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageEl {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElKeyUsageElRef {
        PrivatecaCertificateConfigElX509ConfigElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
    critical: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
    #[doc =
        "Set the field `excluded_dns_names`.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_excluded_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_email_addresses`.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_excluded_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_ip_ranges`.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_excluded_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_uris`.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_excluded_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_dns_names`.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn set_permitted_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_dns_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_email_addresses`.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn set_permitted_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_ip_ranges`.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn set_permitted_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permitted_uris`.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn set_permitted_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.permitted_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
    #[doc = "Indicates whether or not the name constraints are marked critical."]
    pub critical: terrars::PrimField<bool>,
}

impl BuildPrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
        PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl {
            critical: self.critical,
            excluded_dns_names: core::default::Default::default(),
            excluded_email_addresses: core::default::Default::default(),
            excluded_ip_ranges: core::default::Default::default(),
            excluded_uris: core::default::Default::default(),
            permitted_dns_names: core::default::Default::default(),
            permitted_email_addresses: core::default::Default::default(),
            permitted_ip_ranges: core::default::Default::default(),
            permitted_uris: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElNameConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElNameConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElNameConstraintsElRef {
        PrivatecaCertificateConfigElX509ConfigElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElNameConstraintsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `critical` after provisioning.\nIndicates whether or not the name constraints are marked critical."]
    pub fn critical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.critical", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_dns_names` after provisioning.\nContains excluded DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn excluded_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_dns_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_email_addresses` after provisioning.\nContains the excluded email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn excluded_email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_ip_ranges` after provisioning.\nContains the excluded IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn excluded_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_uris` after provisioning.\nContains the excluded URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn excluded_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_dns_names` after provisioning.\nContains permitted DNS names. Any DNS name that can be\nconstructed by simply adding zero or more labels to\nthe left-hand side of the name satisfies the name constraint.\nFor example, 'example.com', 'www.example.com', 'www.sub.example.com'\nwould satisfy 'example.com' while 'example1.com' does not."]
    pub fn permitted_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_dns_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_email_addresses` after provisioning.\nContains the permitted email addresses. The value can be a particular\nemail address, a hostname to indicate all email addresses on that host or\na domain with a leading period (e.g. '.example.com') to indicate\nall email addresses in that domain."]
    pub fn permitted_email_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_email_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_ip_ranges` after provisioning.\nContains the permitted IP ranges. For IPv4 addresses, the ranges\nare expressed using CIDR notation as specified in RFC 4632.\nFor IPv6 addresses, the ranges are expressed in similar encoding as IPv4\naddresses."]
    pub fn permitted_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permitted_uris` after provisioning.\nContains the permitted URIs that apply to the host part of the name.\nThe value can be a hostname or a domain with a\nleading period (like '.example.com')"]
    pub fn permitted_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permitted_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl {
    object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl { }

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigElPolicyIdsEl {
    #[doc =
        "An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub object_id_path: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildPrivatecaCertificateConfigElX509ConfigElPolicyIdsEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl {
        PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl { object_id_path: self.object_id_path }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElPolicyIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElPolicyIdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivatecaCertificateConfigElX509ConfigElPolicyIdsElRef {
        PrivatecaCertificateConfigElX509ConfigElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElPolicyIdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_id_path` after provisioning.\nAn ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages."]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateConfigElX509ConfigElDynamic {
    additional_extensions: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl>>,
    ca_options: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElCaOptionsEl>>,
    key_usage: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElKeyUsageEl>>,
    name_constraints: std
    ::option
    ::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl>>,
    policy_ids: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigElX509ConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElCaOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_constraints: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl>>,
    dynamic: PrivatecaCertificateConfigElX509ConfigElDynamic,
}

impl PrivatecaCertificateConfigElX509ConfigEl {
    #[doc =
        "Set the field `aia_ocsp_servers`.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn set_aia_ocsp_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.aia_ocsp_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_extensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_extensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ca_options`.\n"]
    pub fn set_ca_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElCaOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ca_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ca_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `key_usage`.\n"]
    pub fn set_key_usage(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElKeyUsageEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_usage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_usage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `name_constraints`.\n"]
    pub fn set_name_constraints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElNameConstraintsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.name_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.name_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_ids`.\n"]
    pub fn set_policy_ids(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigElPolicyIdsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_ids = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_ids = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigElX509ConfigEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigElX509ConfigEl {}

impl BuildPrivatecaCertificateConfigElX509ConfigEl {
    pub fn build(self) -> PrivatecaCertificateConfigElX509ConfigEl {
        PrivatecaCertificateConfigElX509ConfigEl {
            aia_ocsp_servers: core::default::Default::default(),
            additional_extensions: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            name_constraints: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElX509ConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElX509ConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateConfigElX509ConfigElRef {
        PrivatecaCertificateConfigElX509ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElX509ConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\nDescribes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the\n\"Authority Information Access\" extension in the certificate."]
    pub fn aia_ocsp_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aia_ocsp_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElAdditionalExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(&self) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElCaOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(&self) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElNameConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElPolicyIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivatecaCertificateConfigElDynamic {
    public_key: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElPublicKeyEl>>,
    subject_config: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElSubjectConfigEl>>,
    x509_config: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigElX509ConfigEl>>,
}

#[derive(Serialize)]
pub struct PrivatecaCertificateConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElPublicKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_config: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElSubjectConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_config: std::option::Option<std::vec::Vec<PrivatecaCertificateConfigElX509ConfigEl>>,
    dynamic: PrivatecaCertificateConfigElDynamic,
}

impl PrivatecaCertificateConfigEl {
    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElPublicKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subject_config`.\n"]
    pub fn set_subject_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElSubjectConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `x509_config`.\n"]
    pub fn set_x509_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivatecaCertificateConfigElX509ConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.x509_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.x509_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivatecaCertificateConfigEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateConfigEl {}

impl BuildPrivatecaCertificateConfigEl {
    pub fn build(self) -> PrivatecaCertificateConfigEl {
        PrivatecaCertificateConfigEl {
            public_key: core::default::Default::default(),
            subject_config: core::default::Default::default(),
            x509_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateConfigElRef {
        PrivatecaCertificateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::ListRef<PrivatecaCertificateConfigElPublicKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_config` after provisioning.\n"]
    pub fn subject_config(&self) -> terrars::ListRef<PrivatecaCertificateConfigElSubjectConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_config", self.base))
    }

    #[doc = "Get a reference to the value of field `x509_config` after provisioning.\n"]
    pub fn x509_config(&self) -> terrars::ListRef<PrivatecaCertificateConfigElX509ConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509_config", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivatecaCertificateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivatecaCertificateTimeoutsEl {
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

impl terrars::ToListMappable for PrivatecaCertificateTimeoutsEl {
    type O = terrars::BlockAssignable<PrivatecaCertificateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivatecaCertificateTimeoutsEl {}

impl BuildPrivatecaCertificateTimeoutsEl {
    pub fn build(self) -> PrivatecaCertificateTimeoutsEl {
        PrivatecaCertificateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PrivatecaCertificateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivatecaCertificateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PrivatecaCertificateTimeoutsElRef {
        PrivatecaCertificateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivatecaCertificateTimeoutsElRef {
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
struct PrivatecaCertificateDynamic {
    config: std::option::Option<terrars::DynamicBlock<PrivatecaCertificateConfigEl>>,
}
