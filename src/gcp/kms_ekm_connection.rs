#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct KmsEkmConnectionData_ {
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
    crypto_space_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_management_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_resolvers: std::option::Option<std::vec::Vec<KmsEkmConnection_ServiceResolvers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KmsEkmConnection_Timeouts>,
    dynamic: KmsEkmConnectionDynamic,
}

struct KmsEkmConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KmsEkmConnectionData_>,
}

#[derive(Clone)]
pub struct KmsEkmConnection(std::rc::Rc<KmsEkmConnection_>);

impl KmsEkmConnection {
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
        "Set the field `crypto_space_path`.\nOptional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS."]
    pub fn set_crypto_space_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().crypto_space_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\nOptional. Etag of the currently stored EkmConnection."]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_management_mode`.\nOptional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default value: \"MANUAL\" Possible values: [\"MANUAL\", \"CLOUD_KMS\"]"]
    pub fn set_key_management_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_management_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_resolvers`.\n"]
    pub fn set_service_resolvers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KmsEkmConnection_ServiceResolvers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_resolvers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_resolvers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KmsEkmConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the EkmConnection was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_space_path` after provisioning.\nOptional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS."]
    pub fn crypto_space_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_space_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. Etag of the currently stored EkmConnection."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_management_mode` after provisioning.\nOptional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default value: \"MANUAL\" Possible values: [\"MANUAL\", \"CLOUD_KMS\"]"]
    pub fn key_management_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_management_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the EkmConnection.\nA full list of valid locations can be found by running 'gcloud kms locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the EkmConnection."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_resolvers` after provisioning.\n"]
    pub fn service_resolvers(&self) -> terrars::ListRef<KmsEkmConnection_ServiceResolversRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_resolvers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsEkmConnection_TimeoutsRef {
        KmsEkmConnection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for KmsEkmConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KmsEkmConnection { }

impl terrars::ToListMappable for KmsEkmConnection {
    type O = terrars::ListRef<KmsEkmConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KmsEkmConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_kms_ekm_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsEkmConnection {
    pub tf_id: std::string::String,
    #[doc =
        "The location for the EkmConnection.\nA full list of valid locations can be found by running 'gcloud kms locations list'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The resource name for the EkmConnection."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildKmsEkmConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> KmsEkmConnection {
        let out = KmsEkmConnection(std::rc::Rc::new(KmsEkmConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KmsEkmConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                crypto_space_path: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                key_management_mode: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_resolvers: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsEkmConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsEkmConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KmsEkmConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the EkmConnection was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crypto_space_path` after provisioning.\nOptional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS."]
    pub fn crypto_space_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_space_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. Etag of the currently stored EkmConnection."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `key_management_mode` after provisioning.\nOptional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL Default value: \"MANUAL\" Possible values: [\"MANUAL\", \"CLOUD_KMS\"]"]
    pub fn key_management_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_management_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the EkmConnection.\nA full list of valid locations can be found by running 'gcloud kms locations list'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the EkmConnection."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_resolvers` after provisioning.\n"]
    pub fn service_resolvers(&self) -> terrars::ListRef<KmsEkmConnection_ServiceResolversRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_resolvers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsEkmConnection_TimeoutsRef {
        KmsEkmConnection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct KmsEkmConnectionDynamic {
    service_resolvers: std::option::Option<terrars::DynamicBlock<KmsEkmConnection_ServiceResolvers>>,
}

#[derive(Serialize)]
pub struct KmsEkmConnection_ServiceResolvers_ServerCertificates {
    raw_der: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_dns_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl KmsEkmConnection_ServiceResolvers_ServerCertificates {
    #[doc =
        "Set the field `subject_alternative_dns_names`.\nOutput only. The subject Alternative DNS names. Only present if parsed is true."]
    pub fn set_subject_alternative_dns_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subject_alternative_dns_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KmsEkmConnection_ServiceResolvers_ServerCertificates {
    type O = terrars::BlockAssignable<KmsEkmConnection_ServiceResolvers_ServerCertificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsEkmConnection_ServiceResolvers_ServerCertificates {
    #[doc = "Required. The raw certificate bytes in DER format. A base64-encoded string."]
    pub raw_der: terrars::PrimField<std::string::String>,
}

impl BuildKmsEkmConnection_ServiceResolvers_ServerCertificates {
    pub fn build(self) -> KmsEkmConnection_ServiceResolvers_ServerCertificates {
        KmsEkmConnection_ServiceResolvers_ServerCertificates {
            raw_der: self.raw_der,
            subject_alternative_dns_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KmsEkmConnection_ServiceResolvers_ServerCertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsEkmConnection_ServiceResolvers_ServerCertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KmsEkmConnection_ServiceResolvers_ServerCertificatesRef {
        KmsEkmConnection_ServiceResolvers_ServerCertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsEkmConnection_ServiceResolvers_ServerCertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nOutput only. The issuer distinguished name in RFC 2253 format. Only present if parsed is true."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_after_time` after provisioning.\nOutput only. The certificate is not valid after this time. Only present if parsed is true.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn not_after_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `not_before_time` after provisioning.\nOutput only. The certificate is not valid before this time. Only present if parsed is true.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn not_before_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parsed` after provisioning.\nOutput only. True if the certificate was parsed successfully."]
    pub fn parsed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parsed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_der` after provisioning.\nRequired. The raw certificate bytes in DER format. A base64-encoded string."]
    pub fn raw_der(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_der", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serial_number` after provisioning.\nOutput only. The certificate serial number as a hex string. Only present if parsed is true."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha256_fingerprint` after provisioning.\nOutput only. The SHA-256 certificate fingerprint as a hex string. Only present if parsed is true."]
    pub fn sha256_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_fingerprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject` after provisioning.\nOutput only. The subject distinguished name in RFC 2253 format. Only present if parsed is true."]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject_alternative_dns_names` after provisioning.\nOutput only. The subject Alternative DNS names. Only present if parsed is true."]
    pub fn subject_alternative_dns_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_dns_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct KmsEkmConnection_ServiceResolversDynamic {
    server_certificates: std
    ::option
    ::Option<terrars::DynamicBlock<KmsEkmConnection_ServiceResolvers_ServerCertificates>>,
}

#[derive(Serialize)]
pub struct KmsEkmConnection_ServiceResolvers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    hostname: terrars::PrimField<std::string::String>,
    service_directory_service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificates: std::option::Option<std::vec::Vec<KmsEkmConnection_ServiceResolvers_ServerCertificates>>,
    dynamic: KmsEkmConnection_ServiceResolversDynamic,
}

impl KmsEkmConnection_ServiceResolvers {
    #[doc =
        "Set the field `endpoint_filter`.\nOptional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest."]
    pub fn set_endpoint_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_certificates`.\n"]
    pub fn set_server_certificates(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<KmsEkmConnection_ServiceResolvers_ServerCertificates>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_certificates = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_certificates = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KmsEkmConnection_ServiceResolvers {
    type O = terrars::BlockAssignable<KmsEkmConnection_ServiceResolvers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsEkmConnection_ServiceResolvers {
    #[doc = "Required. The hostname of the EKM replica used at TLS and HTTP layers."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The resource name of the Service Directory service pointing to an EKM replica, in the format projects/*/locations/*/namespaces/*/services/*"]
    pub service_directory_service: terrars::PrimField<std::string::String>,
}

impl BuildKmsEkmConnection_ServiceResolvers {
    pub fn build(self) -> KmsEkmConnection_ServiceResolvers {
        KmsEkmConnection_ServiceResolvers {
            endpoint_filter: core::default::Default::default(),
            hostname: self.hostname,
            service_directory_service: self.service_directory_service,
            server_certificates: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KmsEkmConnection_ServiceResolversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsEkmConnection_ServiceResolversRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsEkmConnection_ServiceResolversRef {
        KmsEkmConnection_ServiceResolversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsEkmConnection_ServiceResolversRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint_filter` after provisioning.\nOptional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest."]
    pub fn endpoint_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nRequired. The hostname of the EKM replica used at TLS and HTTP layers."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_directory_service` after provisioning.\nRequired. The resource name of the Service Directory service pointing to an EKM replica, in the format projects/*/locations/*/namespaces/*/services/*"]
    pub fn service_directory_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_directory_service", self.base))
    }

    #[doc = "Get a reference to the value of field `server_certificates` after provisioning.\n"]
    pub fn server_certificates(&self) -> terrars::ListRef<KmsEkmConnection_ServiceResolvers_ServerCertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_certificates", self.base))
    }
}

#[derive(Serialize)]
pub struct KmsEkmConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsEkmConnection_Timeouts {
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

impl terrars::ToListMappable for KmsEkmConnection_Timeouts {
    type O = terrars::BlockAssignable<KmsEkmConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsEkmConnection_Timeouts {}

impl BuildKmsEkmConnection_Timeouts {
    pub fn build(self) -> KmsEkmConnection_Timeouts {
        KmsEkmConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KmsEkmConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsEkmConnection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsEkmConnection_TimeoutsRef {
        KmsEkmConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsEkmConnection_TimeoutsRef {
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
