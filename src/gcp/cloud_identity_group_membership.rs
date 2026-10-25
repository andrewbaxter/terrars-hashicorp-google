#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudIdentityGroupMembershipData_ {
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
    create_ignore_already_exists: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    group: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_member_key: std::option::Option<std::vec::Vec<CloudIdentityGroupMembership_PreferredMemberKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<std::vec::Vec<CloudIdentityGroupMembership_Roles>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudIdentityGroupMembership_Timeouts>,
    dynamic: CloudIdentityGroupMembershipDynamic,
}

struct CloudIdentityGroupMembership_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudIdentityGroupMembershipData_>,
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

    #[doc =
        "Set the field `create_ignore_already_exists`.\nIf set to true, skip group member creation if a membership with the same name already exists. Defaults to false."]
    pub fn set_create_ignore_already_exists(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_ignore_already_exists = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudIdentityGroupMembership_PreferredMemberKey>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudIdentityGroupMembership_Roles>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudIdentityGroupMembership_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\nIf set to true, skip group member creation if a membership with the same name already exists. Defaults to false."]
    pub fn create_ignore_already_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_ignore_already_exists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Membership was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn preferred_member_key(&self) -> terrars::ListRef<CloudIdentityGroupMembership_PreferredMemberKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupMembership_TimeoutsRef {
        CloudIdentityGroupMembership_TimeoutsRef::new(
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
            data: std::cell::RefCell::new(CloudIdentityGroupMembershipData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                create_ignore_already_exists: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `create_ignore_already_exists` after provisioning.\nIf set to true, skip group member creation if a membership with the same name already exists. Defaults to false."]
    pub fn create_ignore_already_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_ignore_already_exists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Membership was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn preferred_member_key(&self) -> terrars::ListRef<CloudIdentityGroupMembership_PreferredMemberKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_member_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudIdentityGroupMembership_TimeoutsRef {
        CloudIdentityGroupMembership_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudIdentityGroupMembershipDynamic {
    preferred_member_key: std::option::Option<terrars::DynamicBlock<CloudIdentityGroupMembership_PreferredMemberKey>>,
    roles: std::option::Option<terrars::DynamicBlock<CloudIdentityGroupMembership_Roles>>,
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembership_PreferredMemberKey {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudIdentityGroupMembership_PreferredMemberKey {
    #[doc =
        "Set the field `namespace`.\nThe namespace in which the entity exists.\n\nIf not specified, the EntityKey represents a Google-managed entity\nsuch as a Google user or a Google Group.\n\nIf specified, the EntityKey represents an external-identity-mapped group.\nThe namespace must correspond to an identity source created in Admin Console\nand must be in the form of 'identitysources/{identity_source_id}'."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudIdentityGroupMembership_PreferredMemberKey {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembership_PreferredMemberKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembership_PreferredMemberKey {
    #[doc =
        "The ID of the entity.\n\nFor Google-managed entities, the id must be the email address of an existing\ngroup or user.\n\nFor external-identity-mapped entities, the id must be a string conforming\nto the Identity Source's requirements.\n\nMust be unique within a namespace."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembership_PreferredMemberKey {
    pub fn build(self) -> CloudIdentityGroupMembership_PreferredMemberKey {
        CloudIdentityGroupMembership_PreferredMemberKey {
            id: self.id,
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudIdentityGroupMembership_PreferredMemberKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembership_PreferredMemberKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudIdentityGroupMembership_PreferredMemberKeyRef {
        CloudIdentityGroupMembership_PreferredMemberKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembership_PreferredMemberKeyRef {
    #[allow(dead_code)]
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
pub struct CloudIdentityGroupMembership_Roles_ExpiryDetail {
    expire_time: terrars::PrimField<std::string::String>,
}

impl CloudIdentityGroupMembership_Roles_ExpiryDetail { }

impl terrars::ToListMappable for CloudIdentityGroupMembership_Roles_ExpiryDetail {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembership_Roles_ExpiryDetail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembership_Roles_ExpiryDetail {
    #[doc =
        "The time at which the MembershipRole will expire.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond\nresolution and up to nine fractional digits.\n\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub expire_time: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembership_Roles_ExpiryDetail {
    pub fn build(self) -> CloudIdentityGroupMembership_Roles_ExpiryDetail {
        CloudIdentityGroupMembership_Roles_ExpiryDetail { expire_time: self.expire_time }
    }
}

#[allow(dead_code)]
pub struct CloudIdentityGroupMembership_Roles_ExpiryDetailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembership_Roles_ExpiryDetailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudIdentityGroupMembership_Roles_ExpiryDetailRef {
        CloudIdentityGroupMembership_Roles_ExpiryDetailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembership_Roles_ExpiryDetailRef {
    #[allow(dead_code)]
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
struct CloudIdentityGroupMembership_RolesDynamic {
    expiry_detail: std::option::Option<terrars::DynamicBlock<CloudIdentityGroupMembership_Roles_ExpiryDetail>>,
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembership_Roles {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiry_detail: std::option::Option<std::vec::Vec<CloudIdentityGroupMembership_Roles_ExpiryDetail>>,
    dynamic: CloudIdentityGroupMembership_RolesDynamic,
}

impl CloudIdentityGroupMembership_Roles {
    #[doc = "Set the field `expiry_detail`.\n"]
    pub fn set_expiry_detail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudIdentityGroupMembership_Roles_ExpiryDetail>>,
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

impl terrars::ToListMappable for CloudIdentityGroupMembership_Roles {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembership_Roles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembership_Roles {
    #[doc =
        "The name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER. Possible values: [\"OWNER\", \"MANAGER\", \"MEMBER\"]"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudIdentityGroupMembership_Roles {
    pub fn build(self) -> CloudIdentityGroupMembership_Roles {
        CloudIdentityGroupMembership_Roles {
            name: self.name,
            expiry_detail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudIdentityGroupMembership_RolesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembership_RolesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudIdentityGroupMembership_RolesRef {
        CloudIdentityGroupMembership_RolesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembership_RolesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the MembershipRole. Must be one of OWNER, MANAGER, MEMBER. Possible values: [\"OWNER\", \"MANAGER\", \"MEMBER\"]"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `expiry_detail` after provisioning.\n"]
    pub fn expiry_detail(&self) -> terrars::ListRef<CloudIdentityGroupMembership_Roles_ExpiryDetailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiry_detail", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudIdentityGroupMembership_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudIdentityGroupMembership_Timeouts {
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

impl terrars::ToListMappable for CloudIdentityGroupMembership_Timeouts {
    type O = terrars::BlockAssignable<CloudIdentityGroupMembership_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudIdentityGroupMembership_Timeouts {}

impl BuildCloudIdentityGroupMembership_Timeouts {
    pub fn build(self) -> CloudIdentityGroupMembership_Timeouts {
        CloudIdentityGroupMembership_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudIdentityGroupMembership_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudIdentityGroupMembership_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudIdentityGroupMembership_TimeoutsRef {
        CloudIdentityGroupMembership_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudIdentityGroupMembership_TimeoutsRef {
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
