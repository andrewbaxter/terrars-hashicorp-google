#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDnsKeysData_ {
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
    managed_zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDnsKeys_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDnsKeysData_>,
}

#[derive(Clone)]
pub struct DataDnsKeys(std::rc::Rc<DataDnsKeys_>);

impl DataDnsKeys {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_signing_keys` after provisioning.\n"]
    pub fn key_signing_keys(&self) -> terrars::ListRef<DataDnsKeys_KeySigningKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_signing_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_zone` after provisioning.\n"]
    pub fn managed_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_signing_keys` after provisioning.\n"]
    pub fn zone_signing_keys(&self) -> terrars::ListRef<DataDnsKeys_ZoneSigningKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_signing_keys", self.extract_ref()))
    }
}

impl terrars::Referable for DataDnsKeys {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDnsKeys { }

impl terrars::ToListMappable for DataDnsKeys {
    type O = terrars::ListRef<DataDnsKeysRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDnsKeys_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_dns_keys".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDnsKeys {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub managed_zone: terrars::PrimField<std::string::String>,
}

impl BuildDataDnsKeys {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDnsKeys {
        let out = DataDnsKeys(std::rc::Rc::new(DataDnsKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDnsKeysData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                managed_zone: self.managed_zone,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDnsKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDnsKeysRef {
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

    #[doc = "Get a reference to the value of field `key_signing_keys` after provisioning.\n"]
    pub fn key_signing_keys(&self) -> terrars::ListRef<DataDnsKeys_KeySigningKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_signing_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_zone` after provisioning.\n"]
    pub fn managed_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_signing_keys` after provisioning.\n"]
    pub fn zone_signing_keys(&self) -> terrars::ListRef<DataDnsKeys_ZoneSigningKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zone_signing_keys", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDnsKeys_KeySigningKeys_Digests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsKeys_KeySigningKeys_Digests {
    #[doc = "Set the field `digest`.\n"]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsKeys_KeySigningKeys_Digests {
    type O = terrars::BlockAssignable<DataDnsKeys_KeySigningKeys_Digests>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsKeys_KeySigningKeys_Digests {}

impl BuildDataDnsKeys_KeySigningKeys_Digests {
    pub fn build(self) -> DataDnsKeys_KeySigningKeys_Digests {
        DataDnsKeys_KeySigningKeys_Digests {
            digest: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsKeys_KeySigningKeys_DigestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsKeys_KeySigningKeys_DigestsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsKeys_KeySigningKeys_DigestsRef {
        DataDnsKeys_KeySigningKeys_DigestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsKeys_KeySigningKeys_DigestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsKeys_KeySigningKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digests: std::option::Option<terrars::ListField<DataDnsKeys_KeySigningKeys_Digests>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ds_record: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_tag: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsKeys_KeySigningKeys {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.creation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digests`.\n"]
    pub fn set_digests(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDnsKeys_KeySigningKeys_Digests>>,
    ) -> Self {
        self.digests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ds_record`.\n"]
    pub fn set_ds_record(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ds_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_length`.\n"]
    pub fn set_key_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_tag`.\n"]
    pub fn set_key_tag(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsKeys_KeySigningKeys {
    type O = terrars::BlockAssignable<DataDnsKeys_KeySigningKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsKeys_KeySigningKeys {}

impl BuildDataDnsKeys_KeySigningKeys {
    pub fn build(self) -> DataDnsKeys_KeySigningKeys {
        DataDnsKeys_KeySigningKeys {
            algorithm: core::default::Default::default(),
            creation_time: core::default::Default::default(),
            description: core::default::Default::default(),
            digests: core::default::Default::default(),
            ds_record: core::default::Default::default(),
            id: core::default::Default::default(),
            is_active: core::default::Default::default(),
            key_length: core::default::Default::default(),
            key_tag: core::default::Default::default(),
            public_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsKeys_KeySigningKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsKeys_KeySigningKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsKeys_KeySigningKeysRef {
        DataDnsKeys_KeySigningKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsKeys_KeySigningKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `digests` after provisioning.\n"]
    pub fn digests(&self) -> terrars::ListRef<DataDnsKeys_KeySigningKeys_DigestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.digests", self.base))
    }

    #[doc = "Get a reference to the value of field `ds_record` after provisioning.\n"]
    pub fn ds_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ds_record", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }

    #[doc = "Get a reference to the value of field `key_length` after provisioning.\n"]
    pub fn key_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_length", self.base))
    }

    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDnsKeys_ZoneSigningKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digests: std::option::Option<terrars::ListField<DataDnsKeys_KeySigningKeys_Digests>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_tag: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDnsKeys_ZoneSigningKeys {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_time`.\n"]
    pub fn set_creation_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.creation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digests`.\n"]
    pub fn set_digests(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDnsKeys_KeySigningKeys_Digests>>,
    ) -> Self {
        self.digests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_active`.\n"]
    pub fn set_is_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_length`.\n"]
    pub fn set_key_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_tag`.\n"]
    pub fn set_key_tag(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDnsKeys_ZoneSigningKeys {
    type O = terrars::BlockAssignable<DataDnsKeys_ZoneSigningKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDnsKeys_ZoneSigningKeys {}

impl BuildDataDnsKeys_ZoneSigningKeys {
    pub fn build(self) -> DataDnsKeys_ZoneSigningKeys {
        DataDnsKeys_ZoneSigningKeys {
            algorithm: core::default::Default::default(),
            creation_time: core::default::Default::default(),
            description: core::default::Default::default(),
            digests: core::default::Default::default(),
            id: core::default::Default::default(),
            is_active: core::default::Default::default(),
            key_length: core::default::Default::default(),
            key_tag: core::default::Default::default(),
            public_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDnsKeys_ZoneSigningKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDnsKeys_ZoneSigningKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDnsKeys_ZoneSigningKeysRef {
        DataDnsKeys_ZoneSigningKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDnsKeys_ZoneSigningKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `digests` after provisioning.\n"]
    pub fn digests(&self) -> terrars::ListRef<DataDnsKeys_KeySigningKeys_DigestsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.digests", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_active` after provisioning.\n"]
    pub fn is_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_active", self.base))
    }

    #[doc = "Get a reference to the value of field `key_length` after provisioning.\n"]
    pub fn key_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_length", self.base))
    }

    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\n"]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}
