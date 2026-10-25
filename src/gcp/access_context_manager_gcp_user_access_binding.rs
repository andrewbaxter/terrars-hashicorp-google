#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerGcpUserAccessBindingData_ {
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
    access_levels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dry_run_access_levels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    organization_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_Principal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoped_access_settings: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_settings: std::option::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_SessionSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerGcpUserAccessBinding_Timeouts>,
    dynamic: AccessContextManagerGcpUserAccessBindingDynamic,
}

struct AccessContextManagerGcpUserAccessBinding_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerGcpUserAccessBindingData_>,
}

#[derive(Clone)]
pub struct AccessContextManagerGcpUserAccessBinding(std::rc::Rc<AccessContextManagerGcpUserAccessBinding_>);

impl AccessContextManagerGcpUserAccessBinding {
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
        "Set the field `access_levels`.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_access_levels(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dry_run_access_levels`.\nOptional. Dry run access level that will be evaluated but will not be enforced. The\naccess denial based on dry run policy will be logged. Only one access\nlevel is supported, not multiple. This list must have exactly one element.\nExample: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_dry_run_access_levels(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dry_run_access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `group_key`.\nImmutable. Google Group id whose members are subject to this binding's restrictions.\nSee \"id\" in the Google Workspace Directory API's Group Resource (https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource).\nIf a group's email address/alias is changed, this resource will continue to point at the changed group.\nThis field does not accept group email addresses or aliases.\nExample: \"01d520gv4vjcrht\""]
    pub fn set_group_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().group_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scoped_access_settings`.\n"]
    pub fn set_scoped_access_settings(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scoped_access_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scoped_access_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_settings`.\n"]
    pub fn set_session_settings(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_SessionSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().session_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.session_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AccessContextManagerGcpUserAccessBinding_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_levels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dry_run_access_levels` after provisioning.\nOptional. Dry run access level that will be evaluated but will not be enforced. The\naccess denial based on dry run policy will be logged. Only one access\nlevel is supported, not multiple. This list must have exactly one element.\nExample: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn dry_run_access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dry_run_access_levels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_key` after provisioning.\nImmutable. Google Group id whose members are subject to this binding's restrictions.\nSee \"id\" in the Google Workspace Directory API's Group Resource (https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource).\nIf a group's email address/alias is changed, this resource will continue to point at the changed group.\nThis field does not accept group email addresses or aliases.\nExample: \"01d520gv4vjcrht\""]
    pub fn group_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nRequired. ID of the parent organization."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scoped_access_settings` after provisioning.\n"]
    pub fn scoped_access_settings(
        &self,
    ) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoped_access_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_settings` after provisioning.\n"]
    pub fn session_settings(&self) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_SessionSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
        AccessContextManagerGcpUserAccessBinding_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerGcpUserAccessBinding {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerGcpUserAccessBinding { }

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding {
    type O = terrars::ListRef<AccessContextManagerGcpUserAccessBindingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerGcpUserAccessBinding_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_gcp_user_access_binding".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding {
    pub tf_id: std::string::String,
    #[doc = "Required. ID of the parent organization."]
    pub organization_id: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerGcpUserAccessBinding {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerGcpUserAccessBinding {
        let out =
            AccessContextManagerGcpUserAccessBinding(std::rc::Rc::new(AccessContextManagerGcpUserAccessBinding_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(AccessContextManagerGcpUserAccessBindingData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    access_levels: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    dry_run_access_levels: core::default::Default::default(),
                    group_key: core::default::Default::default(),
                    id: core::default::Default::default(),
                    organization_id: self.organization_id,
                    principal: core::default::Default::default(),
                    scoped_access_settings: core::default::Default::default(),
                    session_settings: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerGcpUserAccessBindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBindingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerGcpUserAccessBindingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_levels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dry_run_access_levels` after provisioning.\nOptional. Dry run access level that will be evaluated but will not be enforced. The\naccess denial based on dry run policy will be logged. Only one access\nlevel is supported, not multiple. This list must have exactly one element.\nExample: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn dry_run_access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dry_run_access_levels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `group_key` after provisioning.\nImmutable. Google Group id whose members are subject to this binding's restrictions.\nSee \"id\" in the Google Workspace Directory API's Group Resource (https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource).\nIf a group's email address/alias is changed, this resource will continue to point at the changed group.\nThis field does not accept group email addresses or aliases.\nExample: \"01d520gv4vjcrht\""]
    pub fn group_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nRequired. ID of the parent organization."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scoped_access_settings` after provisioning.\n"]
    pub fn scoped_access_settings(
        &self,
    ) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoped_access_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_settings` after provisioning.\n"]
    pub fn session_settings(&self) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_SessionSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
        AccessContextManagerGcpUserAccessBinding_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBindingDynamic {
    principal: std::option::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_Principal>>,
    scoped_access_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings>>,
    session_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_SessionSettings>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federated_principal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_project_number: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerGcpUserAccessBinding_Principal {
    #[doc =
        "Set the field `federated_principal`.\nImmutable. The IAM principal identifier of the federated workforce or workload to\nassign the policy to. Examples include the following:\n* Single principal:\n'principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}'\n* All workloads in a workload identity pool:\n'principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*'\n* All Workforce Pools in a Google Cloud organization:\n'principalSet://cloudresourcemanager.googleapis.com/organizations/{organization_id}/type/WorkforcePool'\n\nBindings created for all Workforce Pools in a Google Cloud organization\nsupport only 'scoped_access_settings' with the 'restricted_project'\nclient scope and active 'session_settings'. No other configurations are\nallowed."]
    pub fn set_federated_principal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.federated_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nImmutable. Service account email used to assign policies to a single service account.\nIf a service account is subject to multiple policies (e.g., if there is a policy for all\nservice accounts in a project and a policy for the service account), the closest (i.e.\nthe most specific) dry-run policy will be used for the dry-run functionality and the\nclosest policy will be used for the enforcement."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_project_number`.\nImmutable. Cloud project number used to assign policies to all service accounts owned by the project."]
    pub fn set_service_account_project_number(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_project_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_Principal {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_Principal {}

impl BuildAccessContextManagerGcpUserAccessBinding_Principal {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_Principal {
        AccessContextManagerGcpUserAccessBinding_Principal {
            federated_principal: core::default::Default::default(),
            service_account: core::default::Default::default(),
            service_account_project_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_PrincipalRef {
        AccessContextManagerGcpUserAccessBinding_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `federated_principal` after provisioning.\nImmutable. The IAM principal identifier of the federated workforce or workload to\nassign the policy to. Examples include the following:\n* Single principal:\n'principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}'\n* All workloads in a workload identity pool:\n'principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*'\n* All Workforce Pools in a Google Cloud organization:\n'principalSet://cloudresourcemanager.googleapis.com/organizations/{organization_id}/type/WorkforcePool'\n\nBindings created for all Workforce Pools in a Google Cloud organization\nsupport only 'scoped_access_settings' with the 'restricted_project'\nclient scope and active 'session_settings'. No other configurations are\nallowed."]
    pub fn federated_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.federated_principal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nImmutable. Service account email used to assign policies to a single service account.\nIf a service account is subject to multiple policies (e.g., if there is a policy for all\nservice accounts in a project and a policy for the service account), the closest (i.e.\nthe most specific) dry-run policy will be used for the dry-run functionality and the\nclosest policy will be used for the enforcement."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_project_number` after provisioning.\nImmutable. Cloud project number used to assign policies to all service accounts owned by the project."]
    pub fn service_account_project_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_project_number", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_SessionSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_inactivity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_length_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_reauth_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_oidc_max_age: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessContextManagerGcpUserAccessBinding_SessionSettings {
    #[doc =
        "Set the field `max_inactivity`.\nOptional. How long a user is allowed to take between actions before a new access token must be issued. Only set for Google Cloud apps."]
    pub fn set_max_inactivity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_inactivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_length`.\nOptional. The session length. Setting this field to zero is equal to disabling session. Also can set infinite session by flipping the enabled bit to false below. If useOidcMaxAge is true, for OIDC apps, the session length will be the minimum of this field and OIDC max_age param.\nIf this field is set to zero, 'session_length_enabled' must be set to false or left unset."]
    pub fn set_session_length(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_length_enabled`.\nOptional. This field enables or disables Google Cloud session length. When false, all fields set above will be disregarded and the session length is basically infinite.\nIf 'session_length' is set to zero, this field must be false."]
    pub fn set_session_length_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.session_length_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_reauth_method`.\nOptional. The session challenges proposed to users when the Google Cloud session length is up. Possible values: [\"LOGIN\", \"SECURITY_KEY\", \"PASSWORD\"]"]
    pub fn set_session_reauth_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.session_reauth_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_oidc_max_age`.\nOptional. Only useful for OIDC apps. When false, the OIDC max_age param, if passed in the authentication request will be ignored. When true, the re-auth period will be the minimum of the sessionLength field and the max_age OIDC param."]
    pub fn set_use_oidc_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_oidc_max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_SessionSettings {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_SessionSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_SessionSettings {}

impl BuildAccessContextManagerGcpUserAccessBinding_SessionSettings {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_SessionSettings {
        AccessContextManagerGcpUserAccessBinding_SessionSettings {
            max_inactivity: core::default::Default::default(),
            session_length: core::default::Default::default(),
            session_length_enabled: core::default::Default::default(),
            session_reauth_method: core::default::Default::default(),
            use_oidc_max_age: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_SessionSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_SessionSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_SessionSettingsRef {
        AccessContextManagerGcpUserAccessBinding_SessionSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_SessionSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_inactivity` after provisioning.\nOptional. How long a user is allowed to take between actions before a new access token must be issued. Only set for Google Cloud apps."]
    pub fn max_inactivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_inactivity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_length` after provisioning.\nOptional. The session length. Setting this field to zero is equal to disabling session. Also can set infinite session by flipping the enabled bit to false below. If useOidcMaxAge is true, for OIDC apps, the session length will be the minimum of this field and OIDC max_age param.\nIf this field is set to zero, 'session_length_enabled' must be set to false or left unset."]
    pub fn session_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_length_enabled` after provisioning.\nOptional. This field enables or disables Google Cloud session length. When false, all fields set above will be disregarded and the session length is basically infinite.\nIf 'session_length' is set to zero, this field must be false."]
    pub fn session_length_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_length_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_reauth_method` after provisioning.\nOptional. The session challenges proposed to users when the Google Cloud session length is up. Possible values: [\"LOGIN\", \"SECURITY_KEY\", \"PASSWORD\"]"]
    pub fn session_reauth_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_reauth_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_oidc_max_age` after provisioning.\nOptional. Only useful for OIDC apps. When false, the OIDC max_age param, if passed in the authentication request will be ignored. When true, the re-auth period will be the minimum of the sessionLength field and the max_age OIDC param."]
    pub fn use_oidc_max_age(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_oidc_max_age", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsDynamic {
    session_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_SessionSettings>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_settings: std::option::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_SessionSettings>>,
    dynamic: AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsDynamic,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {
    #[doc =
        "Set the field `access_levels`.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_settings`.\n"]
    pub fn set_session_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_SessionSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings {
            access_levels: core::default::Default::default(),
            session_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc = "Get a reference to the value of field `session_settings` after provisioning.\n"]
    pub fn session_settings(&self) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_SessionSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {
    #[doc =
        "Set the field `access_levels`.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings {
            access_levels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettingsRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nOptional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {
    #[doc = "Set the field `client_id`.\nThe OAuth client ID of the application."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the application. Example: \"Cloud Console\""]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {
    pub fn build(
        self,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication {
            client_id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplicationRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe OAuth client ID of the application."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the application. Example: \"Cloud Console\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject {
    name: terrars::PrimField<std::string::String>,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject { }

impl terrars
::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject {
    #[doc =
        "The Google Cloud project resource name.\nFormat: 'projects/{project_number}'. Only the project number is supported.\nExample: 'projects/1234567890'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject {
    pub fn build(
        self,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject {
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProjectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProjectRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe Google Cloud project resource name.\nFormat: 'projects/{project_number}'. Only the project number is supported.\nExample: 'projects/1234567890'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeDynamic {
    restricted_client_application: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication,
        >,
    >,
    restricted_project: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_client_application: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_project: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject>,
    >,
    dynamic: AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeDynamic,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {
    #[doc = "Set the field `restricted_client_application`.\n"]
    pub fn set_restricted_client_application(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplication,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.restricted_client_application = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.restricted_client_application = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restricted_project`.\n"]
    pub fn set_restricted_project(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProject,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.restricted_project = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.restricted_project = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {
    type O =
        terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope {
            restricted_client_application: core::default::Default::default(),
            restricted_project: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `restricted_client_application` after provisioning.\n"]
    pub fn restricted_client_application(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedClientApplicationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_client_application", self.base))
    }

    #[doc = "Get a reference to the value of field `restricted_project` after provisioning.\n"]
    pub fn restricted_project(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope_RestrictedProjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_project", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeDynamic {
    client_scope: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_scope: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope>>,
    dynamic: AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeDynamic,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {
    #[doc = "Set the field `client_scope`.\n"]
    pub fn set_client_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScope,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope {
            client_scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_scope` after provisioning.\n"]
    pub fn client_scope(
        &self,
    ) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope_ClientScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsDynamic {
    active_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings>>,
    dry_run_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings>>,
    scope: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_settings: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dry_run_settings: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<std::vec::Vec<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope>>,
    dynamic: AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsDynamic,
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {
    #[doc = "Set the field `active_settings`.\n"]
    pub fn set_active_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.active_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.active_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dry_run_settings`.\n"]
    pub fn set_dry_run_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dry_run_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dry_run_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_Scope>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {}

impl BuildAccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings {
            active_settings: core::default::Default::default(),
            dry_run_settings: core::default::Default::default(),
            scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef {
        AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_ScopedAccessSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_settings` after provisioning.\n"]
    pub fn active_settings(
        &self,
    ) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ActiveSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dry_run_settings` after provisioning.\n"]
    pub fn dry_run_settings(
        &self,
    ) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_DryRunSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dry_run_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<AccessContextManagerGcpUserAccessBinding_ScopedAccessSettings_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBinding_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerGcpUserAccessBinding_Timeouts {
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

impl terrars::ToListMappable for AccessContextManagerGcpUserAccessBinding_Timeouts {
    type O = terrars::BlockAssignable<AccessContextManagerGcpUserAccessBinding_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding_Timeouts {}

impl BuildAccessContextManagerGcpUserAccessBinding_Timeouts {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBinding_Timeouts {
        AccessContextManagerGcpUserAccessBinding_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
        AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBinding_TimeoutsRef {
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
