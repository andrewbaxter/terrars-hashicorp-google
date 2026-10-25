#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudIdentityGroupLookupData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_key: std::option::Option<std::vec::Vec<DataCloudIdentityGroupLookup_GroupKey>>,
    dynamic: DataCloudIdentityGroupLookupDynamic,
}

struct DataCloudIdentityGroupLookup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudIdentityGroupLookupData_>,
}

#[derive(Clone)]
pub struct DataCloudIdentityGroupLookup(std::rc::Rc<DataCloudIdentityGroupLookup_>);

impl DataCloudIdentityGroupLookup {
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

    #[doc = "Set the field `group_key`.\n"]
    pub fn set_group_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCloudIdentityGroupLookup_GroupKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().group_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.group_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up Group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> terrars::ListRef<DataCloudIdentityGroupLookup_GroupKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_key", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudIdentityGroupLookup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudIdentityGroupLookup { }

impl terrars::ToListMappable for DataCloudIdentityGroupLookup {
    type O = terrars::ListRef<DataCloudIdentityGroupLookupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudIdentityGroupLookup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_identity_group_lookup".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudIdentityGroupLookup {
    pub tf_id: std::string::String,
}

impl BuildDataCloudIdentityGroupLookup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudIdentityGroupLookup {
        let out = DataCloudIdentityGroupLookup(std::rc::Rc::new(DataCloudIdentityGroupLookup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudIdentityGroupLookupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                group_key: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudIdentityGroupLookupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupLookupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudIdentityGroupLookupRef {
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
        "Get a reference to the value of field `name` after provisioning.\nThe [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up Group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> terrars::ListRef<DataCloudIdentityGroupLookup_GroupKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_key", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataCloudIdentityGroupLookupDynamic {
    group_key: std::option::Option<terrars::DynamicBlock<DataCloudIdentityGroupLookup_GroupKey>>,
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupLookup_GroupKey {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupLookup_GroupKey {
    #[doc =
        "Set the field `namespace`.\nThe namespace in which the entity exists. If not specified, the EntityKey represents a Google-managed entity such as a Google user or a Google Group.\nIf specified, the EntityKey represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of identitysources/{identity_source}."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupLookup_GroupKey {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupLookup_GroupKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupLookup_GroupKey {
    #[doc =
        "The ID of the entity. For Google-managed entities, the id should be the email address of an existing group or user.\nFor external-identity-mapped entities, the id must be a string conforming to the Identity Source's requirements.\nMust be unique within a namespace."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudIdentityGroupLookup_GroupKey {
    pub fn build(self) -> DataCloudIdentityGroupLookup_GroupKey {
        DataCloudIdentityGroupLookup_GroupKey {
            id: self.id,
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupLookup_GroupKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupLookup_GroupKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudIdentityGroupLookup_GroupKeyRef {
        DataCloudIdentityGroupLookup_GroupKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupLookup_GroupKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the entity. For Google-managed entities, the id should be the email address of an existing group or user.\nFor external-identity-mapped entities, the id must be a string conforming to the Identity Source's requirements.\nMust be unique within a namespace."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe namespace in which the entity exists. If not specified, the EntityKey represents a Google-managed entity such as a Google user or a Google Group.\nIf specified, the EntityKey represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of identitysources/{identity_source}."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
