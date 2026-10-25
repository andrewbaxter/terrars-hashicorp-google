#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudIdentityGroupMembershipsData_ {
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

struct DataCloudIdentityGroupMemberships_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudIdentityGroupMembershipsData_>,
}

#[derive(Clone)]
pub struct DataCloudIdentityGroupMemberships(std::rc::Rc<DataCloudIdentityGroupMemberships_>);

impl DataCloudIdentityGroupMemberships {
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
    pub fn memberships(&self) -> terrars::ListRef<DataCloudIdentityGroupMemberships_MembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudIdentityGroupMemberships {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudIdentityGroupMemberships { }

impl terrars::ToListMappable for DataCloudIdentityGroupMemberships {
    type O = terrars::ListRef<DataCloudIdentityGroupMembershipsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudIdentityGroupMemberships_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_identity_group_memberships".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudIdentityGroupMemberships {
    pub tf_id: std::string::String,
    #[doc = "The name of the Group to get memberships from."]
    pub group: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudIdentityGroupMemberships {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudIdentityGroupMemberships {
        let out = DataCloudIdentityGroupMemberships(std::rc::Rc::new(DataCloudIdentityGroupMemberships_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudIdentityGroupMembershipsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                group: self.group,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudIdentityGroupMembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMembershipsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudIdentityGroupMembershipsRef {
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
    pub fn memberships(&self) -> terrars::ListRef<DataCloudIdentityGroupMemberships_MembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {
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

impl terrars::ToListMappable for DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {}

impl BuildDataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {
    pub fn build(self) -> DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {
        DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKeyRef {
        DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKeyRef {
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
pub struct DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {}

impl BuildDataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {
    pub fn build(self) -> DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {
        DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail {
            expire_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetailRef {
        DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupMemberships_Memberships_Roles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiry_detail: std
    ::option
    ::Option<terrars::ListField<DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMemberships_Memberships_Roles {
    #[doc = "Set the field `expiry_detail`.\n"]
    pub fn set_expiry_detail(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetail>>,
    ) -> Self {
        self.expiry_detail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupMemberships_Memberships_Roles {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMemberships_Memberships_Roles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMemberships_Memberships_Roles {}

impl BuildDataCloudIdentityGroupMemberships_Memberships_Roles {
    pub fn build(self) -> DataCloudIdentityGroupMemberships_Memberships_Roles {
        DataCloudIdentityGroupMemberships_Memberships_Roles {
            expiry_detail: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupMemberships_Memberships_RolesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMemberships_Memberships_RolesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMemberships_Memberships_RolesRef {
        DataCloudIdentityGroupMemberships_Memberships_RolesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMemberships_Memberships_RolesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiry_detail` after provisioning.\n"]
    pub fn expiry_detail(&self) -> terrars::ListRef<DataCloudIdentityGroupMemberships_Memberships_Roles_ExpiryDetailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiry_detail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupMemberships_Memberships {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_ignore_already_exists: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_member_key: std
    ::option
    ::Option<terrars::ListField<DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<DataCloudIdentityGroupMemberships_Memberships_Roles>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMemberships_Memberships {
    #[doc = "Set the field `create_ignore_already_exists`.\n"]
    pub fn set_create_ignore_already_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.create_ignore_already_exists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_member_key`.\n"]
    pub fn set_preferred_member_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKey>>,
    ) -> Self {
        self.preferred_member_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCloudIdentityGroupMemberships_Memberships_Roles>>,
    ) -> Self {
        self.roles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupMemberships_Memberships {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMemberships_Memberships>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMemberships_Memberships {}

impl BuildDataCloudIdentityGroupMemberships_Memberships {
    pub fn build(self) -> DataCloudIdentityGroupMemberships_Memberships {
        DataCloudIdentityGroupMemberships_Memberships {
            create_ignore_already_exists: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            group: core::default::Default::default(),
            name: core::default::Default::default(),
            preferred_member_key: core::default::Default::default(),
            roles: core::default::Default::default(),
            type_: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityGroupMemberships_MembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMemberships_MembershipsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMemberships_MembershipsRef {
        DataCloudIdentityGroupMemberships_MembershipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMemberships_MembershipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\n"]
    pub fn create_ignore_already_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_ignore_already_exists", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(
        &self,
    ) -> terrars::ListRef<DataCloudIdentityGroupMemberships_Memberships_PreferredMemberKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::SetRef<DataCloudIdentityGroupMemberships_Memberships_RolesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
