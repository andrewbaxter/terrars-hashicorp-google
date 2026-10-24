use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudIdentityGroupMembershipsData {
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
    data: std::cell::RefCell<DataCloudIdentityGroupMembershipsData>,
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
    pub fn memberships(&self) -> terrars::ListRef<DataCloudIdentityGroupMembershipsMembershipsElRef> {
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
            data: std::cell::RefCell::new(DataCloudIdentityGroupMembershipsData {
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
    pub fn memberships(&self) -> terrars::ListRef<DataCloudIdentityGroupMembershipsMembershipsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memberships", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
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

impl terrars::ToListMappable for DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {}

impl BuildDataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
        DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
        DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef {
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
pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {}

impl BuildDataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
        DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl {
            expire_time: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
        DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiry_detail: std
    ::option
    ::Option<terrars::ListField<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    #[doc = "Set the field `expiry_detail`.\n"]
    pub fn set_expiry_detail(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailEl>>,
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

impl terrars::ToListMappable for DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsElRolesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMembershipsMembershipsElRolesEl {}

impl BuildDataCloudIdentityGroupMembershipsMembershipsElRolesEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
        DataCloudIdentityGroupMembershipsMembershipsElRolesEl {
            expiry_detail: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
        DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMembershipsMembershipsElRolesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiry_detail` after provisioning.\n"]
    pub fn expiry_detail(
        &self,
    ) -> terrars::ListRef<DataCloudIdentityGroupMembershipsMembershipsElRolesElExpiryDetailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiry_detail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupMembershipsMembershipsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_member_key: std
    ::option
    ::Option<terrars::ListField<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<DataCloudIdentityGroupMembershipsMembershipsElRolesEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupMembershipsMembershipsEl {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
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
                    ::Into<terrars::ListField<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyEl>>,
    ) -> Self {
        self.preferred_member_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataCloudIdentityGroupMembershipsMembershipsElRolesEl>>,
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

impl terrars::ToListMappable for DataCloudIdentityGroupMembershipsMembershipsEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupMembershipsMembershipsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupMembershipsMembershipsEl {}

impl BuildDataCloudIdentityGroupMembershipsMembershipsEl {
    pub fn build(self) -> DataCloudIdentityGroupMembershipsMembershipsEl {
        DataCloudIdentityGroupMembershipsMembershipsEl {
            create_time: core::default::Default::default(),
            group: core::default::Default::default(),
            name: core::default::Default::default(),
            preferred_member_key: core::default::Default::default(),
            roles: core::default::Default::default(),
            type_: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupMembershipsMembershipsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupMembershipsMembershipsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupMembershipsMembershipsElRef {
        DataCloudIdentityGroupMembershipsMembershipsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupMembershipsMembershipsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
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
    ) -> terrars::ListRef<DataCloudIdentityGroupMembershipsMembershipsElPreferredMemberKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::SetRef<DataCloudIdentityGroupMembershipsMembershipsElRolesElRef> {
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
