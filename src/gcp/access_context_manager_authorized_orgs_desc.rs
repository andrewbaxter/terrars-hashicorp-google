#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerAuthorizedOrgsDescData_ {
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
    asset_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orgs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerAuthorizedOrgsDesc_Timeouts>,
}

struct AccessContextManagerAuthorizedOrgsDesc_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerAuthorizedOrgsDescData_>,
}

#[derive(Clone)]
pub struct AccessContextManagerAuthorizedOrgsDesc(std::rc::Rc<AccessContextManagerAuthorizedOrgsDesc_>);

impl AccessContextManagerAuthorizedOrgsDesc {
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
        "Set the field `asset_type`.\nThe type of entities that need to use the authorization relationship during\nevaluation, such as a device. Valid values are \"ASSET_TYPE_DEVICE\" and\n\"ASSET_TYPE_CREDENTIAL_STRENGTH\". Possible values: [\"ASSET_TYPE_DEVICE\", \"ASSET_TYPE_CREDENTIAL_STRENGTH\"]"]
    pub fn set_asset_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().asset_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `authorization_direction`.\nThe direction of the authorization relationship between this organization\nand the organizations listed in the \"orgs\" field. The valid values for this\nfield include the following:\n\nAUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic\nin the organizations listed in the 'orgs' field.\n\nAUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the 'orgs'\nfield to evaluate the traffic in this organization.\n\nFor the authorization relationship to take effect, all of the organizations\nmust authorize and specify the appropriate relationship direction. For\nexample, if organization A authorized organization B and C to evaluate its\ntraffic, by specifying \"AUTHORIZATION_DIRECTION_TO\" as the authorization\ndirection, organizations B and C must specify\n\"AUTHORIZATION_DIRECTION_FROM\" as the authorization direction in their\n\"AuthorizedOrgsDesc\" resource. Possible values: [\"AUTHORIZATION_DIRECTION_TO\", \"AUTHORIZATION_DIRECTION_FROM\"]"]
    pub fn set_authorization_direction(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().authorization_direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `authorization_type`.\nA granular control type for authorization levels. Valid value is \"AUTHORIZATION_TYPE_TRUST\". Possible values: [\"AUTHORIZATION_TYPE_TRUST\"]"]
    pub fn set_authorization_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorization_type = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `orgs`.\nThe list of organization ids in this AuthorizedOrgsDesc.\nFormat: 'organizations/<org_number>'\nExample: 'organizations/123456'"]
    pub fn set_orgs(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().orgs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AccessContextManagerAuthorizedOrgsDesc_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `asset_type` after provisioning.\nThe type of entities that need to use the authorization relationship during\nevaluation, such as a device. Valid values are \"ASSET_TYPE_DEVICE\" and\n\"ASSET_TYPE_CREDENTIAL_STRENGTH\". Possible values: [\"ASSET_TYPE_DEVICE\", \"ASSET_TYPE_CREDENTIAL_STRENGTH\"]"]
    pub fn asset_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_direction` after provisioning.\nThe direction of the authorization relationship between this organization\nand the organizations listed in the \"orgs\" field. The valid values for this\nfield include the following:\n\nAUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic\nin the organizations listed in the 'orgs' field.\n\nAUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the 'orgs'\nfield to evaluate the traffic in this organization.\n\nFor the authorization relationship to take effect, all of the organizations\nmust authorize and specify the appropriate relationship direction. For\nexample, if organization A authorized organization B and C to evaluate its\ntraffic, by specifying \"AUTHORIZATION_DIRECTION_TO\" as the authorization\ndirection, organizations B and C must specify\n\"AUTHORIZATION_DIRECTION_FROM\" as the authorization direction in their\n\"AuthorizedOrgsDesc\" resource. Possible values: [\"AUTHORIZATION_DIRECTION_TO\", \"AUTHORIZATION_DIRECTION_FROM\"]"]
    pub fn authorization_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_type` after provisioning.\nA granular control type for authorization levels. Valid value is \"AUTHORIZATION_TYPE_TRUST\". Possible values: [\"AUTHORIZATION_TYPE_TRUST\"]"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the AuthorizedOrgsDesc was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the 'AuthorizedOrgsDesc'. Format:\n'accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}'.\nThe 'authorized_orgs_desc' component must begin with a letter, followed by\nalphanumeric characters or '_'.\nAfter you create an 'AuthorizedOrgsDesc', you cannot change its 'name'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `orgs` after provisioning.\nThe list of organization ids in this AuthorizedOrgsDesc.\nFormat: 'organizations/<org_number>'\nExample: 'organizations/123456'"]
    pub fn orgs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orgs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nRequired. Resource name for the access policy which owns this 'AuthorizedOrgsDesc'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the AuthorizedOrgsDesc was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
        AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerAuthorizedOrgsDesc {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerAuthorizedOrgsDesc { }

impl terrars::ToListMappable for AccessContextManagerAuthorizedOrgsDesc {
    type O = terrars::ListRef<AccessContextManagerAuthorizedOrgsDescRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerAuthorizedOrgsDesc_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_authorized_orgs_desc".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerAuthorizedOrgsDesc {
    pub tf_id: std::string::String,
    #[doc =
        "Resource name for the 'AuthorizedOrgsDesc'. Format:\n'accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}'.\nThe 'authorized_orgs_desc' component must begin with a letter, followed by\nalphanumeric characters or '_'.\nAfter you create an 'AuthorizedOrgsDesc', you cannot change its 'name'."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Required. Resource name for the access policy which owns this 'AuthorizedOrgsDesc'."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerAuthorizedOrgsDesc {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerAuthorizedOrgsDesc {
        let out = AccessContextManagerAuthorizedOrgsDesc(std::rc::Rc::new(AccessContextManagerAuthorizedOrgsDesc_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessContextManagerAuthorizedOrgsDescData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                asset_type: core::default::Default::default(),
                authorization_direction: core::default::Default::default(),
                authorization_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                orgs: core::default::Default::default(),
                parent: self.parent,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerAuthorizedOrgsDescRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAuthorizedOrgsDescRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerAuthorizedOrgsDescRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_type` after provisioning.\nThe type of entities that need to use the authorization relationship during\nevaluation, such as a device. Valid values are \"ASSET_TYPE_DEVICE\" and\n\"ASSET_TYPE_CREDENTIAL_STRENGTH\". Possible values: [\"ASSET_TYPE_DEVICE\", \"ASSET_TYPE_CREDENTIAL_STRENGTH\"]"]
    pub fn asset_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_direction` after provisioning.\nThe direction of the authorization relationship between this organization\nand the organizations listed in the \"orgs\" field. The valid values for this\nfield include the following:\n\nAUTHORIZATION_DIRECTION_FROM: Allows this organization to evaluate traffic\nin the organizations listed in the 'orgs' field.\n\nAUTHORIZATION_DIRECTION_TO: Allows the organizations listed in the 'orgs'\nfield to evaluate the traffic in this organization.\n\nFor the authorization relationship to take effect, all of the organizations\nmust authorize and specify the appropriate relationship direction. For\nexample, if organization A authorized organization B and C to evaluate its\ntraffic, by specifying \"AUTHORIZATION_DIRECTION_TO\" as the authorization\ndirection, organizations B and C must specify\n\"AUTHORIZATION_DIRECTION_FROM\" as the authorization direction in their\n\"AuthorizedOrgsDesc\" resource. Possible values: [\"AUTHORIZATION_DIRECTION_TO\", \"AUTHORIZATION_DIRECTION_FROM\"]"]
    pub fn authorization_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorization_type` after provisioning.\nA granular control type for authorization levels. Valid value is \"AUTHORIZATION_TYPE_TRUST\". Possible values: [\"AUTHORIZATION_TYPE_TRUST\"]"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the AuthorizedOrgsDesc was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the 'AuthorizedOrgsDesc'. Format:\n'accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}'.\nThe 'authorized_orgs_desc' component must begin with a letter, followed by\nalphanumeric characters or '_'.\nAfter you create an 'AuthorizedOrgsDesc', you cannot change its 'name'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `orgs` after provisioning.\nThe list of organization ids in this AuthorizedOrgsDesc.\nFormat: 'organizations/<org_number>'\nExample: 'organizations/123456'"]
    pub fn orgs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orgs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nRequired. Resource name for the access policy which owns this 'AuthorizedOrgsDesc'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the AuthorizedOrgsDesc was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
        AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerAuthorizedOrgsDesc_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerAuthorizedOrgsDesc_Timeouts {
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

impl terrars::ToListMappable for AccessContextManagerAuthorizedOrgsDesc_Timeouts {
    type O = terrars::BlockAssignable<AccessContextManagerAuthorizedOrgsDesc_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerAuthorizedOrgsDesc_Timeouts {}

impl BuildAccessContextManagerAuthorizedOrgsDesc_Timeouts {
    pub fn build(self) -> AccessContextManagerAuthorizedOrgsDesc_Timeouts {
        AccessContextManagerAuthorizedOrgsDesc_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
        AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerAuthorizedOrgsDesc_TimeoutsRef {
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
