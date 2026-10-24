use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataPrivatecaCertificateAuthorityData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPrivatecaCertificateAuthority_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPrivatecaCertificateAuthorityData>,
}

#[derive(Clone)]
pub struct DataPrivatecaCertificateAuthority(std::rc::Rc<DataPrivatecaCertificateAuthority_>);

impl DataPrivatecaCertificateAuthority {
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

    #[doc = "Set the field `certificate_authority_id`.\nThe user provided Resource ID for this Certificate Authority."]
    pub fn set_certificate_authority_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().certificate_authority_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nLocation of the CertificateAuthority. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool`.\nThe name of the CaPool this Certificate Authority belongs to."]
    pub fn set_pool(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_urls` after provisioning.\nURLs for accessing content published by this CA, such as the CA certificate and CRLs."]
    pub fn access_urls(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityAccessUrlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority_id` after provisioning.\nThe user provided Resource ID for this Certificate Authority."]
    pub fn certificate_authority_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe config used to create a self-signed X.509 certificate or CSR."]
    pub fn config(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this CertificateAuthority was created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the CertificateAuthority. Unless this field is set to false\nin Terraform state, a 'terraform destroy' or 'terraform apply' that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcs_bucket` after provisioning.\nThe name of a Cloud Storage bucket where this CertificateAuthority will publish content,\nsuch as the CA certificate and CRLs. This must be a bucket name, without any prefixes\n(such as 'gs://') or suffixes (such as '.googleapis.com'). For example, to use a bucket named\nmy-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will be\ncreated."]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_active_certificates_on_deletion` after provisioning.\nThis field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and unexpired certs.\nUse with care. Defaults to 'false'."]
    pub fn ignore_active_certificates_on_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_active_certificates_on_deletion", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `key_spec` after provisioning.\nUsed when issuing certificates for this CertificateAuthority. If this CertificateAuthority\nis a self-signed CertificateAuthority, this key is also used to sign the self-signed CA\ncertificate. Otherwise, it is used to sign a CSR."]
    pub fn key_spec(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityKeySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the CertificateAuthority. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateAuthority in the format\nprojects/*/locations/*/certificateAuthorities/*."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_ca_certificate` after provisioning.\nThe signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with a third party issuer."]
    pub fn pem_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_ca_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_ca_certificates` after provisioning.\nThis CertificateAuthority's certificate chain, including the current\nCertificateAuthority's certificate. Ordered such that the root issuer is the final\nelement (consistent with RFC 5246). For a self-signed CA, this will only list the current\nCertificateAuthority's certificate."]
    pub fn pem_ca_certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pem_ca_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pem_csr` after provisioning.\n"]
    pub fn pem_csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_csr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool` after provisioning.\nThe name of the CaPool this Certificate Authority belongs to."]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_grace_period` after provisioning.\nIf this flag is set, the Certificate Authority will be deleted as soon as\npossible without a 30-day grace period where undeletion would have been\nallowed. If you proceed, there will be no way to recover this CA.\nUse with care. Defaults to 'false'."]
    pub fn skip_grace_period(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_grace_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe State for this CertificateAuthority."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subordinate_config` after provisioning.\nIf this is a subordinate CertificateAuthority, this field will be set\nwith the subordinate configuration, which describes its issuers."]
    pub fn subordinate_config(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthoritySubordinateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subordinate_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe Type of this CertificateAuthority.\n\n~> **Note:** For 'SUBORDINATE' Certificate Authorities, they need to\nbe activated before they can issue certificates. Default value: \"SELF_SIGNED\" Possible values: [\"SELF_SIGNED\", \"SUBORDINATE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this CertificateAuthority was updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataPrivatecaCertificateAuthority {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPrivatecaCertificateAuthority { }

impl terrars::ToListMappable for DataPrivatecaCertificateAuthority {
    type O = terrars::ListRef<DataPrivatecaCertificateAuthorityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPrivatecaCertificateAuthority_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_privateca_certificate_authority".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPrivatecaCertificateAuthority {
    pub tf_id: std::string::String,
}

impl BuildDataPrivatecaCertificateAuthority {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPrivatecaCertificateAuthority {
        let out = DataPrivatecaCertificateAuthority(std::rc::Rc::new(DataPrivatecaCertificateAuthority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPrivatecaCertificateAuthorityData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                certificate_authority_id: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                pool: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPrivatecaCertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPrivatecaCertificateAuthorityRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `access_urls` after provisioning.\nURLs for accessing content published by this CA, such as the CA certificate and CRLs."]
    pub fn access_urls(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityAccessUrlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `certificate_authority_id` after provisioning.\nThe user provided Resource ID for this Certificate Authority."]
    pub fn certificate_authority_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe config used to create a self-signed X.509 certificate or CSR."]
    pub fn config(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which this CertificateAuthority was created.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the CertificateAuthority. Unless this field is set to false\nin Terraform state, a 'terraform destroy' or 'terraform apply' that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcs_bucket` after provisioning.\nThe name of a Cloud Storage bucket where this CertificateAuthority will publish content,\nsuch as the CA certificate and CRLs. This must be a bucket name, without any prefixes\n(such as 'gs://') or suffixes (such as '.googleapis.com'). For example, to use a bucket named\nmy-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will be\ncreated."]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_active_certificates_on_deletion` after provisioning.\nThis field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and unexpired certs.\nUse with care. Defaults to 'false'."]
    pub fn ignore_active_certificates_on_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_active_certificates_on_deletion", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `key_spec` after provisioning.\nUsed when issuing certificates for this CertificateAuthority. If this CertificateAuthority\nis a self-signed CertificateAuthority, this key is also used to sign the self-signed CA\ncertificate. Otherwise, it is used to sign a CSR."]
    pub fn key_spec(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityKeySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels with user-defined metadata.\n\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\":\n\"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lifetime` after provisioning.\nThe desired lifetime of the CA certificate. Used to create the \"notBeforeTime\" and\n\"notAfterTime\" fields inside an X.509 certificate. A duration in seconds with up to nine\nfractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn lifetime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifetime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the CertificateAuthority. A full list of valid locations can be found by\nrunning 'gcloud privateca locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for this CertificateAuthority in the format\nprojects/*/locations/*/certificateAuthorities/*."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_ca_certificate` after provisioning.\nThe signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with a third party issuer."]
    pub fn pem_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_ca_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pem_ca_certificates` after provisioning.\nThis CertificateAuthority's certificate chain, including the current\nCertificateAuthority's certificate. Ordered such that the root issuer is the final\nelement (consistent with RFC 5246). For a self-signed CA, this will only list the current\nCertificateAuthority's certificate."]
    pub fn pem_ca_certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pem_ca_certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pem_csr` after provisioning.\n"]
    pub fn pem_csr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pem_csr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pool` after provisioning.\nThe name of the CaPool this Certificate Authority belongs to."]
    pub fn pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_grace_period` after provisioning.\nIf this flag is set, the Certificate Authority will be deleted as soon as\npossible without a 30-day grace period where undeletion would have been\nallowed. If you proceed, there will be no way to recover this CA.\nUse with care. Defaults to 'false'."]
    pub fn skip_grace_period(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_grace_period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe State for this CertificateAuthority."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subordinate_config` after provisioning.\nIf this is a subordinate CertificateAuthority, this field will be set\nwith the subordinate configuration, which describes its issuers."]
    pub fn subordinate_config(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthoritySubordinateConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subordinate_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe Type of this CertificateAuthority.\n\n~> **Note:** For 'SUBORDINATE' Certificate Authorities, they need to\nbe activated before they can issue certificates. Default value: \"SELF_SIGNED\" Possible values: [\"SELF_SIGNED\", \"SUBORDINATE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which this CertificateAuthority was updated.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityAccessUrlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate_access_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_access_urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataPrivatecaCertificateAuthorityAccessUrlsEl {
    #[doc = "Set the field `ca_certificate_access_url`.\n"]
    pub fn set_ca_certificate_access_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ca_certificate_access_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crl_access_urls`.\n"]
    pub fn set_crl_access_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.crl_access_urls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityAccessUrlsEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityAccessUrlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityAccessUrlsEl {}

impl BuildDataPrivatecaCertificateAuthorityAccessUrlsEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityAccessUrlsEl {
        DataPrivatecaCertificateAuthorityAccessUrlsEl {
            ca_certificate_access_url: core::default::Default::default(),
            crl_access_urls: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityAccessUrlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityAccessUrlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityAccessUrlsElRef {
        DataPrivatecaCertificateAuthorityAccessUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityAccessUrlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certificate_access_url` after provisioning.\n"]
    pub fn ca_certificate_access_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_access_url", self.base))
    }

    #[doc = "Get a reference to the value of field `crl_access_urls` after provisioning.\n"]
    pub fn crl_access_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crl_access_urls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
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

impl DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
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

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
        DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl {
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

pub struct DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
        DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef {
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
pub struct DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
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

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
        DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl {
            dns_names: core::default::Default::default(),
            email_addresses: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
            uris: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
        DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alt_name: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>>,
}

impl DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectEl>>,
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
                        terrars::ListField<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameEl>,
                    >,
    ) -> Self {
        self.subject_alt_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
        DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl {
            subject: core::default::Default::default(),
            subject_alt_name: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
        DataPrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElSubjectConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_alt_name` after provisioning.\n"]
    pub fn subject_alt_name(
        &self,
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElSubjectAltNameElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alt_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    #[doc = "Set the field `object_id_path`.\n"]
    pub fn set_object_id_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.object_id_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    type O =
        terrars
        ::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    critical: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id: std
    ::option
    ::Option<
        terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
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
                            DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdEl,
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

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl {
            critical: core::default::Default::default(),
            object_id: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef {
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
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElObjectIdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_issuer_path_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_ca: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zero_max_issuer_path_length: std::option::Option<terrars::PrimField<bool>>,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
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

    #[doc = "Set the field `non_ca`.\n"]
    pub fn set_non_ca(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.non_ca = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zero_max_issuer_path_length`.\n"]
    pub fn set_zero_max_issuer_path_length(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.zero_max_issuer_path_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl {
            is_ca: core::default::Default::default(),
            max_issuer_path_length: core::default::Default::default(),
            non_ca: core::default::Default::default(),
            zero_max_issuer_path_length: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef {
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

    #[doc = "Get a reference to the value of field `non_ca` after provisioning.\n"]
    pub fn non_ca(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_ca", self.base))
    }

    #[doc = "Get a reference to the value of field `zero_max_issuer_path_length` after provisioning.\n"]
    pub fn zero_max_issuer_path_length(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zero_max_issuer_path_length", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
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

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
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

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl {
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

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef {
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
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
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

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
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

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    type O =
        terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl {
            client_auth: core::default::Default::default(),
            code_signing: core::default::Default::default(),
            email_protection: core::default::Default::default(),
            ocsp_signing: core::default::Default::default(),
            server_auth: core::default::Default::default(),
            time_stamping: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef {
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
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
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
::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    type O =
        terrars
        ::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_key_usage: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_key_usage: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_extended_key_usages: std
    ::option
    ::Option<
        terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl>,
    >,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    #[doc = "Set the field `base_key_usage`.\n"]
    pub fn set_base_key_usage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageEl>,
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
                        ::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageEl>,
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
                            DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesEl,
                        >,
                    >,
    ) -> Self {
        self.unknown_extended_key_usages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl {
            base_key_usage: core::default::Default::default(),
            extended_key_usage: core::default::Default::default(),
            unknown_extended_key_usages: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_key_usage` after provisioning.\n"]
    pub fn base_key_usage(
        &self,
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElBaseKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `extended_key_usage` after provisioning.\n"]
    pub fn extended_key_usage(
        &self,
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElExtendedKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_extended_key_usages` after provisioning.\n"]
    pub fn unknown_extended_key_usages(
        &self,
    ) -> terrars
    ::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElUnknownExtendedKeyUsagesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unknown_extended_key_usages", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
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

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
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

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl {
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

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef {
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
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_id_path: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    #[doc = "Set the field `object_id_path`.\n"]
    pub fn set_object_id_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.object_id_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl {
            object_id_path: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_id_path` after provisioning.\n"]
    pub fn object_id_path(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_id_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_extensions: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aia_ocsp_servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_options: std::option::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_usage: std::option::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_constraints: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_ids: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>>,
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigEl {
    #[doc = "Set the field `additional_extensions`.\n"]
    pub fn set_additional_extensions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsEl>,
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
                    ::Into<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsEl>>,
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
                    ::Into<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageEl>>,
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
                        terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsEl>,
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
                    ::Into<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsEl>>,
    ) -> Self {
        self.policy_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigElX509ConfigEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigElX509ConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigElX509ConfigEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigEl {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigEl {
            additional_extensions: core::default::Default::default(),
            aia_ocsp_servers: core::default::Default::default(),
            ca_options: core::default::Default::default(),
            key_usage: core::default::Default::default(),
            name_constraints: core::default::Default::default(),
            policy_ids: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElX509ConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElX509ConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthorityConfigElX509ConfigElRef {
        DataPrivatecaCertificateAuthorityConfigElX509ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElX509ConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_extensions` after provisioning.\n"]
    pub fn additional_extensions(
        &self,
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElAdditionalExtensionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_extensions", self.base))
    }

    #[doc = "Get a reference to the value of field `aia_ocsp_servers` after provisioning.\n"]
    pub fn aia_ocsp_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aia_ocsp_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `ca_options` after provisioning.\n"]
    pub fn ca_options(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElCaOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_options", self.base))
    }

    #[doc = "Get a reference to the value of field `key_usage` after provisioning.\n"]
    pub fn key_usage(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElKeyUsageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `name_constraints` after provisioning.\n"]
    pub fn name_constraints(
        &self,
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElNameConstraintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_constraints", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_ids` after provisioning.\n"]
    pub fn policy_ids(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElPolicyIdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_config: std::option::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_config: std::option::Option<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigEl>>,
}

impl DataPrivatecaCertificateAuthorityConfigEl {
    #[doc = "Set the field `subject_config`.\n"]
    pub fn set_subject_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElSubjectConfigEl>>,
    ) -> Self {
        self.subject_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x509_config`.\n"]
    pub fn set_x509_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPrivatecaCertificateAuthorityConfigElX509ConfigEl>>,
    ) -> Self {
        self.x509_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityConfigEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityConfigEl {}

impl BuildDataPrivatecaCertificateAuthorityConfigEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityConfigEl {
        DataPrivatecaCertificateAuthorityConfigEl {
            subject_config: core::default::Default::default(),
            x509_config: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPrivatecaCertificateAuthorityConfigElRef {
        DataPrivatecaCertificateAuthorityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_config` after provisioning.\n"]
    pub fn subject_config(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElSubjectConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_config", self.base))
    }

    #[doc = "Get a reference to the value of field `x509_config` after provisioning.\n"]
    pub fn x509_config(&self) -> terrars::ListRef<DataPrivatecaCertificateAuthorityConfigElX509ConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.x509_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthorityKeySpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_kms_key_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPrivatecaCertificateAuthorityKeySpecEl {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_kms_key_version`.\n"]
    pub fn set_cloud_kms_key_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_kms_key_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthorityKeySpecEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthorityKeySpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthorityKeySpecEl {}

impl BuildDataPrivatecaCertificateAuthorityKeySpecEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthorityKeySpecEl {
        DataPrivatecaCertificateAuthorityKeySpecEl {
            algorithm: core::default::Default::default(),
            cloud_kms_key_version: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthorityKeySpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthorityKeySpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPrivatecaCertificateAuthorityKeySpecElRef {
        DataPrivatecaCertificateAuthorityKeySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthorityKeySpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_kms_key_version` after provisioning.\n"]
    pub fn cloud_kms_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_kms_key_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_certificates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    #[doc = "Set the field `pem_certificates`.\n"]
    pub fn set_pem_certificates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.pem_certificates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {}

impl BuildDataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
        DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl {
            pem_certificates: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
        DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pem_certificates` after provisioning.\n"]
    pub fn pem_certificates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pem_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivatecaCertificateAuthoritySubordinateConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pem_issuer_chain: std
    ::option
    ::Option<terrars::ListField<DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>>,
}

impl DataPrivatecaCertificateAuthoritySubordinateConfigEl {
    #[doc = "Set the field `certificate_authority`.\n"]
    pub fn set_certificate_authority(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_authority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pem_issuer_chain`.\n"]
    pub fn set_pem_issuer_chain(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainEl>>,
    ) -> Self {
        self.pem_issuer_chain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivatecaCertificateAuthoritySubordinateConfigEl {
    type O = terrars::BlockAssignable<DataPrivatecaCertificateAuthoritySubordinateConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivatecaCertificateAuthoritySubordinateConfigEl {}

impl BuildDataPrivatecaCertificateAuthoritySubordinateConfigEl {
    pub fn build(self) -> DataPrivatecaCertificateAuthoritySubordinateConfigEl {
        DataPrivatecaCertificateAuthoritySubordinateConfigEl {
            certificate_authority: core::default::Default::default(),
            pem_issuer_chain: core::default::Default::default(),
        }
    }
}

pub struct DataPrivatecaCertificateAuthoritySubordinateConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivatecaCertificateAuthoritySubordinateConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivatecaCertificateAuthoritySubordinateConfigElRef {
        DataPrivatecaCertificateAuthoritySubordinateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivatecaCertificateAuthoritySubordinateConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority", self.base))
    }

    #[doc = "Get a reference to the value of field `pem_issuer_chain` after provisioning.\n"]
    pub fn pem_issuer_chain(
        &self,
    ) -> terrars::ListRef<DataPrivatecaCertificateAuthoritySubordinateConfigElPemIssuerChainElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pem_issuer_chain", self.base))
    }
}
