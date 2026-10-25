#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PrivilegedAccessManagerEntitlementData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    entitlement_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    max_request_duration: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_notification_targets: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_workflow: std::option::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_ApprovalWorkflow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eligible_users: std::option::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_EligibleUsers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    privileged_access: std::option::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_PrivilegedAccess>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_justification_config: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PrivilegedAccessManagerEntitlement_Timeouts>,
    dynamic: PrivilegedAccessManagerEntitlementDynamic,
}

struct PrivilegedAccessManagerEntitlement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PrivilegedAccessManagerEntitlementData_>,
}

#[derive(Clone)]
pub struct PrivilegedAccessManagerEntitlement(std::rc::Rc<PrivilegedAccessManagerEntitlement_>);

impl PrivilegedAccessManagerEntitlement {
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

    #[doc = "Set the field `additional_notification_targets`.\n"]
    pub fn set_additional_notification_targets(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().additional_notification_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.additional_notification_targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `approval_workflow`.\n"]
    pub fn set_approval_workflow(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_workflow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_workflow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eligible_users`.\n"]
    pub fn set_eligible_users(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_EligibleUsers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eligible_users = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eligible_users = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `privileged_access`.\n"]
    pub fn set_privileged_access(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_PrivilegedAccess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().privileged_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.privileged_access = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requester_justification_config`.\n"]
    pub fn set_requester_justification_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requester_justification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requester_justification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PrivilegedAccessManagerEntitlement_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\""]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe region of the Entitlement resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_request_duration` after provisioning.\nThe maximum amount of time for which access would be granted for a request.\nA requester can choose to ask for access for less than this duration but never more.\nFormat: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = \"7200s\", 45 minutes = \"2700s\""]
    pub fn max_request_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_request_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput Only. The entitlement's name follows a hierarchical structure, comprising the organization, folder, or project, alongside the region and a unique entitlement ID.\nFormats: organizations/{organization-number}/locations/{region}/entitlements/{entitlement-id}, folders/{folder-number}/locations/{region}/entitlements/{entitlement-id}, and projects/{project-id|project-number}/locations/{region}/entitlements/{entitlement-id}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nFormat: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Entitlement."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_notification_targets` after provisioning.\n"]
    pub fn additional_notification_targets(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approval_workflow` after provisioning.\n"]
    pub fn approval_workflow(&self) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_workflow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eligible_users` after provisioning.\n"]
    pub fn eligible_users(&self) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_EligibleUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eligible_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `privileged_access` after provisioning.\n"]
    pub fn privileged_access(&self) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_PrivilegedAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.privileged_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_justification_config` after provisioning.\n"]
    pub fn requester_justification_config(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivilegedAccessManagerEntitlement_TimeoutsRef {
        PrivilegedAccessManagerEntitlement_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for PrivilegedAccessManagerEntitlement {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PrivilegedAccessManagerEntitlement { }

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement {
    type O = terrars::ListRef<PrivilegedAccessManagerEntitlementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PrivilegedAccessManagerEntitlement_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_privileged_access_manager_entitlement".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub entitlement_id: terrars::PrimField<std::string::String>,
    #[doc = "The region of the Entitlement resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The maximum amount of time for which access would be granted for a request.\nA requester can choose to ask for access for less than this duration but never more.\nFormat: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = \"7200s\", 45 minutes = \"2700s\""]
    pub max_request_duration: terrars::PrimField<std::string::String>,
    #[doc =
        "Format: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildPrivilegedAccessManagerEntitlement {
    pub fn build(self, stack: &mut terrars::Stack) -> PrivilegedAccessManagerEntitlement {
        let out = PrivilegedAccessManagerEntitlement(std::rc::Rc::new(PrivilegedAccessManagerEntitlement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PrivilegedAccessManagerEntitlementData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                entitlement_id: self.entitlement_id,
                id: core::default::Default::default(),
                location: self.location,
                max_request_duration: self.max_request_duration,
                parent: self.parent,
                additional_notification_targets: core::default::Default::default(),
                approval_workflow: core::default::Default::default(),
                eligible_users: core::default::Default::default(),
                privileged_access: core::default::Default::default(),
                requester_justification_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PrivilegedAccessManagerEntitlementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PrivilegedAccessManagerEntitlementRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\""]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe region of the Entitlement resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_request_duration` after provisioning.\nThe maximum amount of time for which access would be granted for a request.\nA requester can choose to ask for access for less than this duration but never more.\nFormat: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = \"7200s\", 45 minutes = \"2700s\""]
    pub fn max_request_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_request_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput Only. The entitlement's name follows a hierarchical structure, comprising the organization, folder, or project, alongside the region and a unique entitlement ID.\nFormats: organizations/{organization-number}/locations/{region}/entitlements/{entitlement-id}, folders/{folder-number}/locations/{region}/entitlements/{entitlement-id}, and projects/{project-id|project-number}/locations/{region}/entitlements/{entitlement-id}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nFormat: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Entitlement."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time stamp. A timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `additional_notification_targets` after provisioning.\n"]
    pub fn additional_notification_targets(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approval_workflow` after provisioning.\n"]
    pub fn approval_workflow(&self) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_workflow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eligible_users` after provisioning.\n"]
    pub fn eligible_users(&self) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_EligibleUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eligible_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `privileged_access` after provisioning.\n"]
    pub fn privileged_access(&self) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_PrivilegedAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.privileged_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_justification_config` after provisioning.\n"]
    pub fn requester_justification_config(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PrivilegedAccessManagerEntitlement_TimeoutsRef {
        PrivilegedAccessManagerEntitlement_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlementDynamic {
    additional_notification_targets: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets>>,
    approval_workflow: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_ApprovalWorkflow>>,
    eligible_users: std::option::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_EligibleUsers>>,
    privileged_access: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_PrivilegedAccess>>,
    requester_justification_config: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig>>,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_email_recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_email_recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    #[doc =
        "Set the field `admin_email_recipients`.\nOptional. Additional email addresses to be notified when a principal(requester) is granted access."]
    pub fn set_admin_email_recipients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.admin_email_recipients = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requester_email_recipients`.\nOptional. Additional email address to be notified about an eligible entitlement."]
    pub fn set_requester_email_recipients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.requester_email_recipients = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {}

impl BuildPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
        PrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
            admin_email_recipients: core::default::Default::default(),
            requester_email_recipients: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
        PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_email_recipients` after provisioning.\nOptional. Additional email addresses to be notified when a principal(requester) is granted access."]
    pub fn admin_email_recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.admin_email_recipients", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requester_email_recipients` after provisioning.\nOptional. Additional email address to be notified about an eligible entitlement."]
    pub fn requester_email_recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.requester_email_recipients", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    principals: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers { }

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    type O =
        terrars
        ::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    #[doc =
        "Users who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at: https://cloud.google.com/iam/docs/principal-identifiers#v1"]
    pub principals: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
            principals: self.principals,
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `principals` after provisioning.\nUsers who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at: https://cloud.google.com/iam/docs/principal-identifiers#v1"]
    pub fn principals(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsDynamic {
    approvers: std
    ::option
    ::Option<
        terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers>,
    >,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approvals_needed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approver_email_recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approvers: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers>>,
    dynamic: PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsDynamic,
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    #[doc =
        "Set the field `approvals_needed`.\nHow many users from the above list need to approve.\nIf there are not enough distinct users in the list above then the workflow\nwill indefinitely block. Should always be greater than 0. Currently 1 is the only\nsupported value."]
    pub fn set_approvals_needed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.approvals_needed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `approver_email_recipients`.\nOptional. Additional email addresses to be notified when a grant is pending approval."]
    pub fn set_approver_email_recipients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.approver_email_recipients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approvers`.\n"]
    pub fn set_approvers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.approvers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.approvers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {}

impl BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
            approvals_needed: core::default::Default::default(),
            approver_email_recipients: core::default::Default::default(),
            approvers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approvals_needed` after provisioning.\nHow many users from the above list need to approve.\nIf there are not enough distinct users in the list above then the workflow\nwill indefinitely block. Should always be greater than 0. Currently 1 is the only\nsupported value."]
    pub fn approvals_needed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approvals_needed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `approver_email_recipients` after provisioning.\nOptional. Additional email addresses to be notified when a grant is pending approval."]
    pub fn approver_email_recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approver_email_recipients", self.base))
    }

    #[doc = "Get a reference to the value of field `approvers` after provisioning.\n"]
    pub fn approvers(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approvers", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsDynamic {
    steps: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>>,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_approver_justification: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>>,
    dynamic: PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsDynamic,
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    #[doc =
        "Set the field `require_approver_justification`.\nOptional. Do the approvers need to provide a justification for their actions?"]
    pub fn set_require_approver_justification(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_approver_justification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.steps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.steps = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {}

impl BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
            require_approver_justification: core::default::Default::default(),
            steps: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `require_approver_justification` after provisioning.\nOptional. Do the approvers need to provide a justification for their actions?"]
    pub fn require_approver_justification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_approver_justification", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlement_ApprovalWorkflowDynamic {
    manual_approvals: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>>,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_approvals: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>>,
    dynamic: PrivilegedAccessManagerEntitlement_ApprovalWorkflowDynamic,
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    #[doc = "Set the field `manual_approvals`.\n"]
    pub fn set_manual_approvals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manual_approvals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manual_approvals = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_ApprovalWorkflow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow {}

impl BuildPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflow {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflow {
            manual_approvals: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
        PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manual_approvals` after provisioning.\n"]
    pub fn manual_approvals(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_approvals", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_EligibleUsers {
    principals: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl PrivilegedAccessManagerEntitlement_EligibleUsers { }

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_EligibleUsers {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_EligibleUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_EligibleUsers {
    #[doc =
        "Users who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at \"https://cloud.google.com/iam/docs/principal-identifiers#v1\""]
    pub principals: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPrivilegedAccessManagerEntitlement_EligibleUsers {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_EligibleUsers {
        PrivilegedAccessManagerEntitlement_EligibleUsers { principals: self.principals }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_EligibleUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_EligibleUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_EligibleUsersRef {
        PrivilegedAccessManagerEntitlement_EligibleUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_EligibleUsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `principals` after provisioning.\nUsers who are being allowed for the operation. Each entry should be a valid v1 IAM Principal Identifier. Format for these is documented at \"https://cloud.google.com/iam/docs/principal-identifiers#v1\""]
    pub fn principals(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    role: terrars::PrimField<std::string::String>,
}

impl PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    #[doc =
        "Set the field `condition_expression`.\nThe expression field of the IAM condition to be associated with the role. If specified, a user with an active grant for this entitlement would be able to access the resource only if this condition evaluates to true for their request.\nhttps://cloud.google.com/iam/docs/conditions-overview#attributes."]
    pub fn set_condition_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.condition_expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    #[doc = "IAM role to be granted. https://cloud.google.com/iam/docs/roles-overview."]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
        PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
            condition_expression: core::default::Default::default(),
            role: self.role,
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
        PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `condition_expression` after provisioning.\nThe expression field of the IAM condition to be associated with the role. If specified, a user with an active grant for this entitlement would be able to access the resource only if this condition evaluates to true for their request.\nhttps://cloud.google.com/iam/docs/conditions-overview#attributes."]
    pub fn condition_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nIAM role to be granted. https://cloud.google.com/iam/docs/roles-overview."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessDynamic {
    role_bindings: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings>>,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    resource: terrars::PrimField<std::string::String>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_bindings: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings>>,
    dynamic: PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessDynamic,
}

impl PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    #[doc = "Set the field `role_bindings`.\n"]
    pub fn set_role_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.role_bindings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.role_bindings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    #[doc = "Name of the resource."]
    pub resource: terrars::PrimField<std::string::String>,
    #[doc = "The type of this resource."]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
        PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
            resource: self.resource,
            resource_type: self.resource_type,
            role_bindings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
        PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\nName of the resource."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nThe type of this resource."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `role_bindings` after provisioning.\n"]
    pub fn role_bindings(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.role_bindings", self.base))
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlement_PrivilegedAccessDynamic {
    gcp_iam_access: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>>,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_PrivilegedAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_iam_access: std::option::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>>,
    dynamic: PrivilegedAccessManagerEntitlement_PrivilegedAccessDynamic,
}

impl PrivilegedAccessManagerEntitlement_PrivilegedAccess {
    #[doc = "Set the field `gcp_iam_access`.\n"]
    pub fn set_gcp_iam_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcp_iam_access = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcp_iam_access = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_PrivilegedAccess {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_PrivilegedAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_PrivilegedAccess {}

impl BuildPrivilegedAccessManagerEntitlement_PrivilegedAccess {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_PrivilegedAccess {
        PrivilegedAccessManagerEntitlement_PrivilegedAccess {
            gcp_iam_access: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
        PrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_iam_access` after provisioning.\n"]
    pub fn gcp_iam_access(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_iam_access", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {}

impl PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory { }

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {}

impl BuildPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {
        PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {}
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
        PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {}

impl PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured { }

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {}

impl BuildPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {
        PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {}
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
        PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfigDynamic {
    not_mandatory: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory>>,
    unstructured: std
    ::option
    ::Option<terrars::DynamicBlock<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured>>,
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_mandatory: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured: std
    ::option
    ::Option<std::vec::Vec<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured>>,
    dynamic: PrivilegedAccessManagerEntitlement_RequesterJustificationConfigDynamic,
}

impl PrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    #[doc = "Set the field `not_mandatory`.\n"]
    pub fn set_not_mandatory(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_mandatory = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_mandatory = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unstructured`.\n"]
    pub fn set_unstructured(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unstructured = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unstructured = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {}

impl BuildPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
        PrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
            not_mandatory: core::default::Default::default(),
            unstructured: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
        PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_mandatory` after provisioning.\n"]
    pub fn not_mandatory(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_mandatory", self.base))
    }

    #[doc = "Get a reference to the value of field `unstructured` after provisioning.\n"]
    pub fn unstructured(
        &self,
    ) -> terrars::ListRef<PrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unstructured", self.base))
    }
}

#[derive(Serialize)]
pub struct PrivilegedAccessManagerEntitlement_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PrivilegedAccessManagerEntitlement_Timeouts {
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

impl terrars::ToListMappable for PrivilegedAccessManagerEntitlement_Timeouts {
    type O = terrars::BlockAssignable<PrivilegedAccessManagerEntitlement_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrivilegedAccessManagerEntitlement_Timeouts {}

impl BuildPrivilegedAccessManagerEntitlement_Timeouts {
    pub fn build(self) -> PrivilegedAccessManagerEntitlement_Timeouts {
        PrivilegedAccessManagerEntitlement_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PrivilegedAccessManagerEntitlement_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PrivilegedAccessManagerEntitlement_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PrivilegedAccessManagerEntitlement_TimeoutsRef {
        PrivilegedAccessManagerEntitlement_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PrivilegedAccessManagerEntitlement_TimeoutsRef {
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
