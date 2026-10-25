#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataIamWorkloadIdentityPoolOpenidConfigData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_name: terrars::PrimField<std::string::String>,
}

struct DataIamWorkloadIdentityPoolOpenidConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamWorkloadIdentityPoolOpenidConfigData_>,
}

#[derive(Clone)]
pub struct DataIamWorkloadIdentityPoolOpenidConfig(std::rc::Rc<DataIamWorkloadIdentityPoolOpenidConfig_>);

impl DataIamWorkloadIdentityPoolOpenidConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `authorization_endpoint` after provisioning.\nURL pointing to an authorization endpoint under this issuer."]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id_token_signing_alg_values_supported` after provisioning.\nJSON array containing a list of the JWS signing algorithms (alg values) supported by the OP for the ID token to encode the claims in a JWT [JWT]."]
    pub fn id_token_signing_alg_values_supported(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.id_token_signing_alg_values_supported", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nURL using the https scheme with no query or fragment components that the OP asserts as its issuer identifier."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jwks_uri` after provisioning.\nURL of the OP's JWK Set [JWK] document, which MUST use the https scheme."]
    pub fn jwks_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\nThe OIDC discovery URI."]
    pub fn resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_types_supported` after provisioning.\nJSON array containing a list of the OAuth 2.0 response_type values that this OP supports."]
    pub fn response_types_supported(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_types_supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subject_types_supported` after provisioning.\nJSON array containing a list of the subject identifier types that this OP supports."]
    pub fn subject_types_supported(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_types_supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nURL pointing to a token endpoint under this issuer."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamWorkloadIdentityPoolOpenidConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamWorkloadIdentityPoolOpenidConfig { }

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolOpenidConfig {
    type O = terrars::ListRef<DataIamWorkloadIdentityPoolOpenidConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamWorkloadIdentityPoolOpenidConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool_openid_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamWorkloadIdentityPoolOpenidConfig {
    pub tf_id: std::string::String,
    #[doc = "The OIDC discovery URI."]
    pub resource_name: terrars::PrimField<std::string::String>,
}

impl BuildDataIamWorkloadIdentityPoolOpenidConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamWorkloadIdentityPoolOpenidConfig {
        let out = DataIamWorkloadIdentityPoolOpenidConfig(std::rc::Rc::new(DataIamWorkloadIdentityPoolOpenidConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamWorkloadIdentityPoolOpenidConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                resource_name: self.resource_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamWorkloadIdentityPoolOpenidConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolOpenidConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamWorkloadIdentityPoolOpenidConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `authorization_endpoint` after provisioning.\nURL pointing to an authorization endpoint under this issuer."]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id_token_signing_alg_values_supported` after provisioning.\nJSON array containing a list of the JWS signing algorithms (alg values) supported by the OP for the ID token to encode the claims in a JWT [JWT]."]
    pub fn id_token_signing_alg_values_supported(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.id_token_signing_alg_values_supported", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nURL using the https scheme with no query or fragment components that the OP asserts as its issuer identifier."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jwks_uri` after provisioning.\nURL of the OP's JWK Set [JWK] document, which MUST use the https scheme."]
    pub fn jwks_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\nThe OIDC discovery URI."]
    pub fn resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_types_supported` after provisioning.\nJSON array containing a list of the OAuth 2.0 response_type values that this OP supports."]
    pub fn response_types_supported(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_types_supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subject_types_supported` after provisioning.\nJSON array containing a list of the subject identifier types that this OP supports."]
    pub fn subject_types_supported(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_types_supported", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nURL pointing to a token endpoint under this issuer."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.extract_ref()))
    }
}
