#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataPrivilegedAccessManagerEntitlementData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entitlement_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPrivilegedAccessManagerEntitlement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPrivilegedAccessManagerEntitlementData_>,
}

#[derive(Clone)]
pub struct DataPrivilegedAccessManagerEntitlement(std::rc::Rc<DataPrivilegedAccessManagerEntitlement_>);

impl DataPrivilegedAccessManagerEntitlement {
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

    #[doc =
        "Set the field `entitlement_id`.\nThe ID to use for this Entitlement. This will become the last part of the resource name.\nThis value should be 4-63 characters, and valid characters are \"[a-z]\", \"[0-9]\", and \"-\". The first character should be from [a-z].\nThis value should be unique among all other Entitlements under the specified 'parent'."]
    pub fn set_entitlement_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().entitlement_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe region of the Entitlement resource."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nFormat: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}"]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `additional_notification_targets` after provisioning.\nAdditionalNotificationTargets includes email addresses to be notified."]
    pub fn additional_notification_targets(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `approval_workflow` after provisioning.\nThe approvals needed before access will be granted to a requester.\nNo approvals will be needed if this field is null. Different types of approval workflows that can be used to gate privileged access granting."]
    pub fn approval_workflow(&self) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_workflow", self.extract_ref()))
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
        "Get a reference to the value of field `eligible_users` after provisioning.\nWho can create Grants using Entitlement. This list should contain at most one entry"]
    pub fn eligible_users(&self) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_EligibleUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eligible_users", self.extract_ref()))
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
        "Get a reference to the value of field `privileged_access` after provisioning.\nPrivileged access that this service can be used to gate."]
    pub fn privileged_access(&self) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.privileged_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requester_justification_config` after provisioning.\nDefines the ways in which a requester should provide the justification while requesting for access."]
    pub fn requester_justification_config(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
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
}

impl terrars::Referable for DataPrivilegedAccessManagerEntitlement {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPrivilegedAccessManagerEntitlement { }

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement {
    type O = terrars::ListRef<DataPrivilegedAccessManagerEntitlementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPrivilegedAccessManagerEntitlement_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_privileged_access_manager_entitlement".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement {
    pub tf_id: std::string::String,
}

impl BuildDataPrivilegedAccessManagerEntitlement {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPrivilegedAccessManagerEntitlement {
        let out = DataPrivilegedAccessManagerEntitlement(std::rc::Rc::new(DataPrivilegedAccessManagerEntitlement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPrivilegedAccessManagerEntitlementData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                entitlement_id: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                parent: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPrivilegedAccessManagerEntitlementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPrivilegedAccessManagerEntitlementRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `additional_notification_targets` after provisioning.\nAdditionalNotificationTargets includes email addresses to be notified."]
    pub fn additional_notification_targets(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.additional_notification_targets", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `approval_workflow` after provisioning.\nThe approvals needed before access will be granted to a requester.\nNo approvals will be needed if this field is null. Different types of approval workflows that can be used to gate privileged access granting."]
    pub fn approval_workflow(&self) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_workflow", self.extract_ref()))
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
        "Get a reference to the value of field `eligible_users` after provisioning.\nWho can create Grants using Entitlement. This list should contain at most one entry"]
    pub fn eligible_users(&self) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_EligibleUsersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eligible_users", self.extract_ref()))
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
        "Get a reference to the value of field `privileged_access` after provisioning.\nPrivileged access that this service can be used to gate."]
    pub fn privileged_access(&self) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.privileged_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requester_justification_config` after provisioning.\nDefines the ways in which a requester should provide the justification while requesting for access."]
    pub fn requester_justification_config(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.requester_justification_config", self.extract_ref()),
        )
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
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_email_recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_email_recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    #[doc = "Set the field `admin_email_recipients`.\n"]
    pub fn set_admin_email_recipients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.admin_email_recipients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requester_email_recipients`.\n"]
    pub fn set_requester_email_recipients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.requester_email_recipients = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {}

impl BuildDataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
        DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargets {
            admin_email_recipients: core::default::Default::default(),
            requester_email_recipients: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
        DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_AdditionalNotificationTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_email_recipients` after provisioning.\n"]
    pub fn admin_email_recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.admin_email_recipients", self.base))
    }

    #[doc = "Get a reference to the value of field `requester_email_recipients` after provisioning.\n"]
    pub fn requester_email_recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.requester_email_recipients", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.principals = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    type O =
        terrars
        ::BlockAssignable<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {}

impl BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers {
            principals: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approvals_needed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approver_email_recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approvers: std
    ::option
    ::Option<
        terrars::ListField<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers>,
    >,
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    #[doc = "Set the field `approvals_needed`.\n"]
    pub fn set_approvals_needed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.approvals_needed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approver_email_recipients`.\n"]
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
                        ::ListField<
                            DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_Approvers,
                        >,
                    >,
    ) -> Self {
        self.approvers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {}

impl BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps {
            approvals_needed: core::default::Default::default(),
            approver_email_recipients: core::default::Default::default(),
            approvers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approvals_needed` after provisioning.\n"]
    pub fn approvals_needed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approvals_needed", self.base))
    }

    #[doc = "Get a reference to the value of field `approver_email_recipients` after provisioning.\n"]
    pub fn approver_email_recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approver_email_recipients", self.base))
    }

    #[doc = "Get a reference to the value of field `approvers` after provisioning.\n"]
    pub fn approvers(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps_ApproversRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approvers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_approver_justification: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std
    ::option
    ::Option<terrars::ListField<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>>,
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    #[doc = "Set the field `require_approver_justification`.\n"]
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
                        ::ListField<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_Steps>,
                    >,
    ) -> Self {
        self.steps = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {}

impl BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals {
            require_approver_justification: core::default::Default::default(),
            steps: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `require_approver_justification` after provisioning.\n"]
    pub fn require_approver_justification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_approver_justification", self.base))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_approvals: std
    ::option
    ::Option<terrars::ListField<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>>,
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    #[doc = "Set the field `manual_approvals`.\n"]
    pub fn set_manual_approvals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovals>,
                    >,
    ) -> Self {
        self.manual_approvals = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {}

impl BuildDataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow {
            manual_approvals: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
        DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_ApprovalWorkflowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manual_approvals` after provisioning.\n"]
    pub fn manual_approvals(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_ApprovalWorkflow_ManualApprovalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_approvals", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_EligibleUsers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataPrivilegedAccessManagerEntitlement_EligibleUsers {
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.principals = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_EligibleUsers {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_EligibleUsers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_EligibleUsers {}

impl BuildDataPrivilegedAccessManagerEntitlement_EligibleUsers {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_EligibleUsers {
        DataPrivilegedAccessManagerEntitlement_EligibleUsers { principals: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_EligibleUsersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_EligibleUsersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_EligibleUsersRef {
        DataPrivilegedAccessManagerEntitlement_EligibleUsersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_EligibleUsersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    #[doc = "Set the field `condition_expression`.\n"]
    pub fn set_condition_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.condition_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    type O =
        terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {}

impl BuildDataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
        DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings {
            condition_expression: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
        DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_expression` after provisioning.\n"]
    pub fn condition_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_bindings: std
    ::option
    ::Option<terrars::ListField<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings>>,
}

impl DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_bindings`.\n"]
    pub fn set_role_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindings>,
                    >,
    ) -> Self {
        self.role_bindings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {}

impl BuildDataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
        DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess {
            resource: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            role_bindings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
        DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `role_bindings` after provisioning.\n"]
    pub fn role_bindings(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess_RoleBindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.role_bindings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_PrivilegedAccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_iam_access: std
    ::option
    ::Option<terrars::ListField<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>>,
}

impl DataPrivilegedAccessManagerEntitlement_PrivilegedAccess {
    #[doc = "Set the field `gcp_iam_access`.\n"]
    pub fn set_gcp_iam_access(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccess>>,
    ) -> Self {
        self.gcp_iam_access = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_PrivilegedAccess {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_PrivilegedAccess {}

impl BuildDataPrivilegedAccessManagerEntitlement_PrivilegedAccess {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_PrivilegedAccess {
        DataPrivilegedAccessManagerEntitlement_PrivilegedAccess { gcp_iam_access: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
        DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_PrivilegedAccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_iam_access` after provisioning.\n"]
    pub fn gcp_iam_access(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_PrivilegedAccess_GcpIamAccessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_iam_access", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {}

impl DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory { }

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {
    type O =
        terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {}

impl BuildDataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {
        DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory {}
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
        DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {}

impl DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured { }

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {
    type O =
        terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {}

impl BuildDataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {
        DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured {}
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
        DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_mandatory: std
    ::option
    ::Option<terrars::ListField<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured: std
    ::option
    ::Option<terrars::ListField<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured>>,
}

impl DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    #[doc = "Set the field `not_mandatory`.\n"]
    pub fn set_not_mandatory(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatory>,
                    >,
    ) -> Self {
        self.not_mandatory = std::option::Option::Some(v.into());
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
                        ::ListField<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_Unstructured>,
                    >,
    ) -> Self {
        self.unstructured = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    type O = terrars::BlockAssignable<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {}

impl BuildDataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
    pub fn build(self) -> DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
        DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig {
            not_mandatory: core::default::Default::default(),
            unstructured: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
        DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_mandatory` after provisioning.\n"]
    pub fn not_mandatory(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_NotMandatoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_mandatory", self.base))
    }

    #[doc = "Get a reference to the value of field `unstructured` after provisioning.\n"]
    pub fn unstructured(
        &self,
    ) -> terrars::ListRef<DataPrivilegedAccessManagerEntitlement_RequesterJustificationConfig_UnstructuredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unstructured", self.base))
    }
}
