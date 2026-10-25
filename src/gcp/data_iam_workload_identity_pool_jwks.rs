#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataIamWorkloadIdentityPoolJwksData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataIamWorkloadIdentityPoolJwks_Timeouts>,
}

struct DataIamWorkloadIdentityPoolJwks_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamWorkloadIdentityPoolJwksData_>,
}

#[derive(Clone)]
pub struct DataIamWorkloadIdentityPoolJwks(std::rc::Rc<DataIamWorkloadIdentityPoolJwks_>);

impl DataIamWorkloadIdentityPoolJwks {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataIamWorkloadIdentityPoolJwks_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jwks_json` after provisioning.\nThe raw JSON string representation of the JWKS response."]
    pub fn jwks_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keys` after provisioning.\nThe list of public keys in the JSON Web Key Set."]
    pub fn keys(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolJwks_KeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_name` after provisioning.\nThe JWKS URI to retrieve the public keys from (e.g. from google_iam_workload_identity_pool_openid_config.jwks_uri)."]
    pub fn resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
        DataIamWorkloadIdentityPoolJwks_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataIamWorkloadIdentityPoolJwks {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamWorkloadIdentityPoolJwks { }

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolJwks {
    type O = terrars::ListRef<DataIamWorkloadIdentityPoolJwksRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamWorkloadIdentityPoolJwks_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool_jwks".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamWorkloadIdentityPoolJwks {
    pub tf_id: std::string::String,
    #[doc =
        "The JWKS URI to retrieve the public keys from (e.g. from google_iam_workload_identity_pool_openid_config.jwks_uri)."]
    pub resource_name: terrars::PrimField<std::string::String>,
}

impl BuildDataIamWorkloadIdentityPoolJwks {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamWorkloadIdentityPoolJwks {
        let out = DataIamWorkloadIdentityPoolJwks(std::rc::Rc::new(DataIamWorkloadIdentityPoolJwks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamWorkloadIdentityPoolJwksData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                resource_name: self.resource_name,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamWorkloadIdentityPoolJwksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolJwksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamWorkloadIdentityPoolJwksRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jwks_json` after provisioning.\nThe raw JSON string representation of the JWKS response."]
    pub fn jwks_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwks_json", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `keys` after provisioning.\nThe list of public keys in the JSON Web Key Set."]
    pub fn keys(&self) -> terrars::ListRef<DataIamWorkloadIdentityPoolJwks_KeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keys", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_name` after provisioning.\nThe JWKS URI to retrieve the public keys from (e.g. from google_iam_workload_identity_pool_openid_config.jwks_uri)."]
    pub fn resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
        DataIamWorkloadIdentityPoolJwks_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolJwks_Keys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alg: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    e: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kty: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    n: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "use", skip_serializing_if = "std::option::Option::is_none")]
    use_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolJwks_Keys {
    #[doc = "Set the field `alg`.\n"]
    pub fn set_alg(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alg = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `e`.\n"]
    pub fn set_e(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.e = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kid`.\n"]
    pub fn set_kid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kty`.\n"]
    pub fn set_kty(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kty = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `n`.\n"]
    pub fn set_n(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.n = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_`.\n"]
    pub fn set_use(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.use_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolJwks_Keys {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolJwks_Keys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolJwks_Keys {}

impl BuildDataIamWorkloadIdentityPoolJwks_Keys {
    pub fn build(self) -> DataIamWorkloadIdentityPoolJwks_Keys {
        DataIamWorkloadIdentityPoolJwks_Keys {
            alg: core::default::Default::default(),
            e: core::default::Default::default(),
            kid: core::default::Default::default(),
            kty: core::default::Default::default(),
            n: core::default::Default::default(),
            use_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolJwks_KeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolJwks_KeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamWorkloadIdentityPoolJwks_KeysRef {
        DataIamWorkloadIdentityPoolJwks_KeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolJwks_KeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alg` after provisioning.\n"]
    pub fn alg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alg", self.base))
    }

    #[doc = "Get a reference to the value of field `e` after provisioning.\n"]
    pub fn e(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.e", self.base))
    }

    #[doc = "Get a reference to the value of field `kid` after provisioning.\n"]
    pub fn kid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kid", self.base))
    }

    #[doc = "Get a reference to the value of field `kty` after provisioning.\n"]
    pub fn kty(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kty", self.base))
    }

    #[doc = "Get a reference to the value of field `n` after provisioning.\n"]
    pub fn n(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.n", self.base))
    }

    #[doc = "Get a reference to the value of field `use_` after provisioning.\n"]
    pub fn use_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamWorkloadIdentityPoolJwks_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamWorkloadIdentityPoolJwks_Timeouts {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamWorkloadIdentityPoolJwks_Timeouts {
    type O = terrars::BlockAssignable<DataIamWorkloadIdentityPoolJwks_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamWorkloadIdentityPoolJwks_Timeouts {}

impl BuildDataIamWorkloadIdentityPoolJwks_Timeouts {
    pub fn build(self) -> DataIamWorkloadIdentityPoolJwks_Timeouts {
        DataIamWorkloadIdentityPoolJwks_Timeouts { read: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
        DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamWorkloadIdentityPoolJwks_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
