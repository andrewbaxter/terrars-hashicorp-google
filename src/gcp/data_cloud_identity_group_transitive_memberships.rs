#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudIdentityGroupTransitiveMembershipsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    group: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudIdentityGroupTransitiveMemberships_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudIdentityGroupTransitiveMembershipsData_>,
}

#[derive(Clone)]
pub struct DataCloudIdentityGroupTransitiveMemberships(std::rc::Rc<DataCloudIdentityGroupTransitiveMemberships_>);

impl DataCloudIdentityGroupTransitiveMemberships {
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
        "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to get memberships from."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memberships` after provisioning.\nList of Cloud Identity group memberships."]
    pub fn memberships(&self) -> terrars::ListRef<DataCloudIdentityGroupTransitiveMemberships_MembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudIdentityGroupTransitiveMemberships {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudIdentityGroupTransitiveMemberships { }

impl terrars::ToListMappable for DataCloudIdentityGroupTransitiveMemberships {
    type O = terrars::ListRef<DataCloudIdentityGroupTransitiveMembershipsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudIdentityGroupTransitiveMemberships_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_identity_group_transitive_memberships".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudIdentityGroupTransitiveMemberships {
    pub tf_id: std::string::String,
    #[doc = "The name of the Group to get memberships from."]
    pub group: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudIdentityGroupTransitiveMemberships {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudIdentityGroupTransitiveMemberships {
        let out =
            DataCloudIdentityGroupTransitiveMemberships(
                std::rc::Rc::new(DataCloudIdentityGroupTransitiveMemberships_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataCloudIdentityGroupTransitiveMembershipsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        group: self.group,
                        id: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudIdentityGroupTransitiveMembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupTransitiveMembershipsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudIdentityGroupTransitiveMembershipsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to get memberships from."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `memberships` after provisioning.\nList of Cloud Identity group memberships."]
    pub fn memberships(&self) -> terrars::ListRef<DataCloudIdentityGroupTransitiveMemberships_MembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {}

impl BuildDataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {
    pub fn build(self) -> DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {
        DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKeyRef {
        DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles {
    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupTransitiveMemberships_Memberships_Roles {}

impl BuildDataCloudIdentityGroupTransitiveMemberships_Memberships_Roles {
    pub fn build(self) -> DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles {
        DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles { role: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupTransitiveMemberships_Memberships_RolesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupTransitiveMemberships_Memberships_RolesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupTransitiveMemberships_Memberships_RolesRef {
        DataCloudIdentityGroupTransitiveMemberships_Memberships_RolesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupTransitiveMemberships_Memberships_RolesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupTransitiveMemberships_Memberships {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_member_key: std
    ::option
    ::Option<terrars::ListField<DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles>>,
}

impl DataCloudIdentityGroupTransitiveMemberships_Memberships {
    #[doc = "Set the field `member`.\n"]
    pub fn set_member(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_member_key`.\n"]
    pub fn set_preferred_member_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKey>,
                    >,
    ) -> Self {
        self.preferred_member_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `relation_type`.\n"]
    pub fn set_relation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.relation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCloudIdentityGroupTransitiveMemberships_Memberships_Roles>>,
    ) -> Self {
        self.roles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupTransitiveMemberships_Memberships {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupTransitiveMemberships_Memberships>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupTransitiveMemberships_Memberships {}

impl BuildDataCloudIdentityGroupTransitiveMemberships_Memberships {
    pub fn build(self) -> DataCloudIdentityGroupTransitiveMemberships_Memberships {
        DataCloudIdentityGroupTransitiveMemberships_Memberships {
            member: core::default::Default::default(),
            preferred_member_key: core::default::Default::default(),
            relation_type: core::default::Default::default(),
            roles: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupTransitiveMemberships_MembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupTransitiveMemberships_MembershipsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupTransitiveMemberships_MembershipsRef {
        DataCloudIdentityGroupTransitiveMemberships_MembershipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupTransitiveMemberships_MembershipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `member` after provisioning.\n"]
    pub fn member(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.member", self.base))
    }

    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(
        &self,
    ) -> terrars::ListRef<DataCloudIdentityGroupTransitiveMemberships_Memberships_PreferredMemberKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.base))
    }

    #[doc = "Get a reference to the value of field `relation_type` after provisioning.\n"]
    pub fn relation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::SetRef<DataCloudIdentityGroupTransitiveMemberships_Memberships_RolesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
}
