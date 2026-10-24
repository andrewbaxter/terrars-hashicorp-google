use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudIdentityGroupMembershipData {
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
    group: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_member_key: std::option::Option<std::vec::Vec<CloudIdentityGroupMembershipPreferredMemberKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<std::vec::Vec<CloudIdentityGroupMembershipRolesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudIdentityGroupMembershipTimeoutsEl>,
    dynamic: CloudIdentityGroupMembershipDynamic,
}

struct CloudIdentityGroupMembership_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudIdentityGroupMembershipData>,
}

#[derive(Clone)]
pub struct CloudIdentityGroupMembership(std::rc::Rc<CloudIdentityGroupMembership_>);

impl CloudIdentityGroupMembership {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_member_key`.\n"]
    pub fn set_preferred_member_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudIdentityGroupMembershipPreferredMemberKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preferred_member_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preferred_member_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudIdentityGroupMembershipRolesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().roles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.roles = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudIdentityGroupMembershipTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Membership was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to create this membership in."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Membership, of the form groups/{group_id}/memberships/{membership_id}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the membership."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Membership was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(&self) -> terrars::ListRef<CloudIdentityGroupMembershipPreferredMemberKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupMembershipTimeoutsElRef {
        CloudIdentityGroupMembershipTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudIdentityGroupMembership {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudIdentityGroupMembership { }

impl terrars::ToListMappable for CloudIdentityGroupMembership {
    type O = terrars::ListRef<CloudIdentityGroupMembershipRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudIdentityGroupMembership_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_identity_group_membership".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudIdentityGroupMembership {
    pub tf_id: std::string::String,
    #[doc = "The name of the Group to create this membership in."]
    pub group: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembership {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudIdentityGroupMembership {
        let out = CloudIdentityGroupMembership(std::rc::Rc::new(CloudIdentityGroupMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudIdentityGroupMembershipData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                group: self.group,
                id: core::default::Default::default(),
                preferred_member_key: core::default::Default::default(),
                roles: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudIdentityGroupMembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembershipRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudIdentityGroupMembershipRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Membership was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group` after provisioning.\nThe name of the Group to create this membership in."]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Membership, of the form groups/{group_id}/memberships/{membership_id}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the membership."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Membership was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_member_key` after provisioning.\n"]
    pub fn preferred_member_key(&self) -> terrars::ListRef<CloudIdentityGroupMembershipPreferredMemberKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupMembershipTimeoutsElRef {
        CloudIdentityGroupMembershipTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipPreferredMemberKeyEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudIdentityGroupMembershipPreferredMemberKeyEl {
    #[doc =
        "Set the field `namespace`.\nThe namespace in which the entity exists.\n\nIf not specified, the EntityKey represents a Google-managed entity\nsuch as a Google user or a Google Group.\n\nIf specified, the EntityKey represents an external-identity-mapped group.\nThe namespace must correspond to an identity source created in Admin Console\nand must be in the form of 'identitysources/{identity_source_id}'."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudIdentityGroupMembershipPreferredMemberKeyEl {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembershipPreferredMemberKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembershipPreferredMemberKeyEl {
    #[doc =
        "The ID of the entity.\n\nFor Google-managed entities, the id must be the email address of an existing\ngroup or user.\n\nFor external-identity-mapped entities, the id must be a string conforming\nto the Identity Source's requirements.\n\nMust be unique within a namespace."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembershipPreferredMemberKeyEl {
    pub fn build(self) -> CloudIdentityGroupMembershipPreferredMemberKeyEl {
        CloudIdentityGroupMembershipPreferredMemberKeyEl {
            id: self.id,
            namespace: core::default::Default::default(),
        }
    }
}

pub struct CloudIdentityGroupMembershipPreferredMemberKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembershipPreferredMemberKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudIdentityGroupMembershipPreferredMemberKeyElRef {
        CloudIdentityGroupMembershipPreferredMemberKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembershipPreferredMemberKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the entity.\n\nFor Google-managed entities, the id must be the email address of an existing\ngroup or user.\n\nFor external-identity-mapped entities, the id must be a string conforming\nto the Identity Source's requirements.\n\nMust be unique within a namespace."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe namespace in which the entity exists.\n\nIf not specified, the EntityKey represents a Google-managed entity\nsuch as a Google user or a Google Group.\n\nIf specified, the EntityKey represents an external-identity-mapped group.\nThe namespace must correspond to an identity source created in Admin Console\nand must be in the form of 'identitysources/{identity_source_id}'."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipRolesElExpiryDetailEl {
    expire_time: terrars::PrimField<std::string::String>,
}

impl CloudIdentityGroupMembershipRolesElExpiryDetailEl { }

impl terrars::ToListMappable for CloudIdentityGroupMembershipRolesElExpiryDetailEl {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembershipRolesElExpiryDetailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembershipRolesElExpiryDetailEl {
    #[doc =
        "The time at which the MembershipRole will expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits.\n\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub expire_time: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembershipRolesElExpiryDetailEl {
    pub fn build(self) -> CloudIdentityGroupMembershipRolesElExpiryDetailEl {
        CloudIdentityGroupMembershipRolesElExpiryDetailEl { expire_time: self.expire_time }
    }
}

pub struct CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
        CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembershipRolesElExpiryDetailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe time at which the MembershipRole will expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits.\n\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudIdentityGroupMembershipRolesElDynamic {
    expiry_detail: std::option::Option<terrars::DynamicBlock<CloudIdentityGroupMembershipRolesElExpiryDetailEl>>,
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipRolesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiry_detail: std::option::Option<std::vec::Vec<CloudIdentityGroupMembershipRolesElExpiryDetailEl>>,
    dynamic: CloudIdentityGroupMembershipRolesElDynamic,
}

impl CloudIdentityGroupMembershipRolesEl {
    #[doc = "Set the field `expiry_detail`.\n"]
    pub fn set_expiry_detail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudIdentityGroupMembershipRolesElExpiryDetailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expiry_detail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expiry_detail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudIdentityGroupMembershipRolesEl {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembershipRolesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembershipRolesEl {
    #[doc =
        "The name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER. Possible values: [\"OWNER\", \"MANAGER\", \"MEMBER\"]"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembershipRolesEl {
    pub fn build(self) -> CloudIdentityGroupMembershipRolesEl {
        CloudIdentityGroupMembershipRolesEl {
            name: self.name,
            expiry_detail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudIdentityGroupMembershipRolesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembershipRolesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudIdentityGroupMembershipRolesElRef {
        CloudIdentityGroupMembershipRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembershipRolesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER. Possible values: [\"OWNER\", \"MANAGER\", \"MEMBER\"]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `expiry_detail` after provisioning.\n"]
    pub fn expiry_detail(&self) -> terrars::ListRef<CloudIdentityGroupMembershipRolesElExpiryDetailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiry_detail", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembershipTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudIdentityGroupMembershipTimeoutsEl {
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

impl terrars::ToListMappable for CloudIdentityGroupMembershipTimeoutsEl {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembershipTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembershipTimeoutsEl {}

impl BuildCloudIdentityGroupMembershipTimeoutsEl {
    pub fn build(self) -> CloudIdentityGroupMembershipTimeoutsEl {
        CloudIdentityGroupMembershipTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudIdentityGroupMembershipTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembershipTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudIdentityGroupMembershipTimeoutsElRef {
        CloudIdentityGroupMembershipTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembershipTimeoutsElRef {
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
struct CloudIdentityGroupMembershipDynamic {
    preferred_member_key: std
    ::option
    ::Option<terrars::DynamicBlock<CloudIdentityGroupMembershipPreferredMemberKeyEl>>,
    roles: std::option::Option<terrars::DynamicBlock<CloudIdentityGroupMembershipRolesEl>>,
}
