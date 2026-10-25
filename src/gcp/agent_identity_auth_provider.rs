#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AgentIdentityAuthProviderData_ {
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
    allowed_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    auth_provider_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocked_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_provider_type_params: std::option::Option<std::vec::Vec<AgentIdentityAuthProvider_AuthProviderTypeParams>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AgentIdentityAuthProvider_Timeouts>,
    dynamic: AgentIdentityAuthProviderDynamic,
}

struct AgentIdentityAuthProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgentIdentityAuthProviderData_>,
}

#[derive(Clone)]
pub struct AgentIdentityAuthProvider(std::rc::Rc<AgentIdentityAuthProvider_>);

impl AgentIdentityAuthProvider {
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
        "Set the field `allowed_scopes`.\nList of scopes that are allowed to be requested for this auth_provider.\nIf this list is non-empty, only scopes within this list may be requested.\nIf this list is empty, all scopes may be requested.\nScopes appearing in 'blocked_scopes' are disallowed even if they appear in\n'allowed_scopes'.\nThe number of allowed scopes is limited to 200."]
    pub fn set_allowed_scopes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `blocked_scopes`.\nList of scopes that are blocked from being requested for this\nauth_provider. If a scope appears in this list, it will not be requested,\neven if it also appears in 'allowed_scopes'. 'blocked_scopes' takes\nprecedence over 'allowed_scopes'. The number of blocked scopes is limited\nto 200."]
    pub fn set_blocked_scopes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().blocked_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the resource.\nMust be less than 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `workload_ids`.\nInput only. Represents the workload identity in IAM 'principal://' format of the\nagent(s) that will use this AuthProvider. Example:\n'principal://agents.global.org-${ORG_ID}.system.id.goog/resources/aiplatform/projects/{PROJECT_ID}/locations/{LOCATIONS}/reasoningEngines/{ID}'"]
    pub fn set_workload_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().workload_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_provider_type_params`.\n"]
    pub fn set_auth_provider_type_params(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_provider_type_params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_provider_type_params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AgentIdentityAuthProvider_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allowed_scopes` after provisioning.\nList of scopes that are allowed to be requested for this auth_provider.\nIf this list is non-empty, only scopes within this list may be requested.\nIf this list is empty, all scopes may be requested.\nScopes appearing in 'blocked_scopes' are disallowed even if they appear in\n'allowed_scopes'.\nThe number of allowed scopes is limited to 200."]
    pub fn allowed_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_provider_id` after provisioning.\nThe ID to use for the AuthProvider, which will become the final segment\nof the AuthProvider's resource name.\nThis value should be 1-63 characters, and valid characters\nare /a-z-/. The first character must be a lowercase letter, and the\nlast character must be a lowercase letter or a number."]
    pub fn auth_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_provider_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocked_scopes` after provisioning.\nList of scopes that are blocked from being requested for this\nauth_provider. If a scope appears in this list, it will not be requested,\neven if it also appears in 'allowed_scopes'. 'blocked_scopes' takes\nprecedence over 'allowed_scopes'. The number of blocked scopes is limited\nto 200."]
    pub fn blocked_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocked_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted` after provisioning.\nThis is set to true if the auth_provider is deleted."]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the resource.\nMust be less than 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe time when the auth_provider will expire."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The full resource name of the auth_provider. Format:\nprojects/{project}/locations/{location}/authProviders/{auth_provider}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the auth_provider.\nPossible values:\nENABLED\nDISABLED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_ids` after provisioning.\nInput only. Represents the workload identity in IAM 'principal://' format of the\nagent(s) that will use this AuthProvider. Example:\n'principal://agents.global.org-${ORG_ID}.system.id.goog/resources/aiplatform/projects/{PROJECT_ID}/locations/{LOCATIONS}/reasoningEngines/{ID}'"]
    pub fn workload_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_provider_type_params` after provisioning.\n"]
    pub fn auth_provider_type_params(&self) -> terrars::ListRef<AgentIdentityAuthProvider_AuthProviderTypeParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_provider_type_params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgentIdentityAuthProvider_TimeoutsRef {
        AgentIdentityAuthProvider_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AgentIdentityAuthProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgentIdentityAuthProvider { }

impl terrars::ToListMappable for AgentIdentityAuthProvider {
    type O = terrars::ListRef<AgentIdentityAuthProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgentIdentityAuthProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_agent_identity_auth_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgentIdentityAuthProvider {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the AuthProvider, which will become the final segment\nof the AuthProvider's resource name.\nThis value should be 1-63 characters, and valid characters\nare /a-z-/. The first character must be a lowercase letter, and the\nlast character must be a lowercase letter or a number."]
    pub auth_provider_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildAgentIdentityAuthProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> AgentIdentityAuthProvider {
        let out = AgentIdentityAuthProvider(std::rc::Rc::new(AgentIdentityAuthProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgentIdentityAuthProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allowed_scopes: core::default::Default::default(),
                auth_provider_id: self.auth_provider_id,
                blocked_scopes: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                workload_ids: core::default::Default::default(),
                auth_provider_type_params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgentIdentityAuthProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgentIdentityAuthProviderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_scopes` after provisioning.\nList of scopes that are allowed to be requested for this auth_provider.\nIf this list is non-empty, only scopes within this list may be requested.\nIf this list is empty, all scopes may be requested.\nScopes appearing in 'blocked_scopes' are disallowed even if they appear in\n'allowed_scopes'.\nThe number of allowed scopes is limited to 200."]
    pub fn allowed_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_provider_id` after provisioning.\nThe ID to use for the AuthProvider, which will become the final segment\nof the AuthProvider's resource name.\nThis value should be 1-63 characters, and valid characters\nare /a-z-/. The first character must be a lowercase letter, and the\nlast character must be a lowercase letter or a number."]
    pub fn auth_provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_provider_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocked_scopes` after provisioning.\nList of scopes that are blocked from being requested for this\nauth_provider. If a scope appears in this list, it will not be requested,\neven if it also appears in 'allowed_scopes'. 'blocked_scopes' takes\nprecedence over 'allowed_scopes'. The number of blocked scopes is limited\nto 200."]
    pub fn blocked_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocked_scopes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted` after provisioning.\nThis is set to true if the auth_provider is deleted."]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the resource.\nMust be less than 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe time when the auth_provider will expire."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The full resource name of the auth_provider. Format:\nprojects/{project}/locations/{location}/authProviders/{auth_provider}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the auth_provider.\nPossible values:\nENABLED\nDISABLED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_ids` after provisioning.\nInput only. Represents the workload identity in IAM 'principal://' format of the\nagent(s) that will use this AuthProvider. Example:\n'principal://agents.global.org-${ORG_ID}.system.id.goog/resources/aiplatform/projects/{PROJECT_ID}/locations/{LOCATIONS}/reasoningEngines/{ID}'"]
    pub fn workload_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_provider_type_params` after provisioning.\n"]
    pub fn auth_provider_type_params(&self) -> terrars::ListRef<AgentIdentityAuthProvider_AuthProviderTypeParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_provider_type_params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgentIdentityAuthProvider_TimeoutsRef {
        AgentIdentityAuthProvider_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AgentIdentityAuthProviderDynamic {
    auth_provider_type_params: std
    ::option
    ::Option<terrars::DynamicBlock<AgentIdentityAuthProvider_AuthProviderTypeParams>>,
}

#[derive(Serialize)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider {}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider { }

impl terrars::ToListMappable for AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider {
    type O = terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider {}

impl BuildAgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider {
    pub fn build(self) -> AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider {
        AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProvider {}
    }
}

#[allow(dead_code)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProviderRef {
        AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey {
    #[doc = "Set the field `api_key`.\nInput only. The API key for this auth_provider."]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey {
    type O = terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey {}

impl BuildAgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey {
    pub fn build(self) -> AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey {
        AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKeyRef {
        AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nInput only. The API key for this auth_provider."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_continue_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_pkce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {
    #[doc =
        "Set the field `authorization_url`.\nThe authorization endpoint to send users to for consenting to delegate\nto the agent.\neg. \"https://auth.atlassian.com/authorize\""]
    pub fn set_authorization_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.authorization_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nThe client ID of the OAuth client."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nInput only. The client secret of the OAuth client."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\nInput only. The client secret of the OAuth client."]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_secret_wo_version`.\nTriggers update of 'client_secret_wo' write-only. Increment this value when an update to 'client_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_client_secret_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_continue_uri`.\nThe default continue URI for 3LO flow and it will be used when no continue\nURI is provided in the RetrieveCredentials request."]
    pub fn set_default_continue_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_continue_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_pkce`.\nEnables Proof Key for Code Exchange (PKCE) for the OAuth flow to prevent\nauthorization code interception attacks."]
    pub fn set_enable_pkce(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_pkce = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token_url`.\nThe token endpoint for requesting tokens on behalf of an end user.\neg. \"https://auth.atlassian.com/oauth/token\""]
    pub fn set_token_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {
    type O = terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {}

impl BuildAgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {
    pub fn build(self) -> AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {
        AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth {
            authorization_url: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_wo_version: core::default::Default::default(),
            default_continue_uri: core::default::Default::default(),
            enable_pkce: core::default::Default::default(),
            token_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauthRef {
        AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorization_url` after provisioning.\nThe authorization endpoint to send users to for consenting to delegate\nto the agent.\neg. \"https://auth.atlassian.com/authorize\""]
    pub fn authorization_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_url", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID of the OAuth client."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The client secret of the OAuth client."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_wo` after provisioning.\nInput only. The client secret of the OAuth client."]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_wo_version` after provisioning.\nTriggers update of 'client_secret_wo' write-only. Increment this value when an update to 'client_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn client_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_continue_uri` after provisioning.\nThe default continue URI for 3LO flow and it will be used when no continue\nURI is provided in the RetrieveCredentials request."]
    pub fn default_continue_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_continue_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_pkce` after provisioning.\nEnables Proof Key for Code Exchange (PKCE) for the OAuth flow to prevent\nauthorization code interception attacks."]
    pub fn enable_pkce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_pkce", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_url` after provisioning.\nThe redirect URL this auth_provider uses for the OAuth exchange.\nThis is deterministic based on the name of the auth_provider."]
    pub fn redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_url` after provisioning.\nThe token endpoint for requesting tokens on behalf of an end user.\neg. \"https://auth.atlassian.com/oauth/token\""]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {
    #[doc = "Set the field `client_id`.\nThe client ID of the OAuth client."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nInput only. The client secret of the OAuth client."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_wo`.\nInput only. The client secret of the OAuth client."]
    pub fn set_client_secret_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_secret_wo_version`.\nTriggers update of 'client_secret_wo' write-only. Increment this value when an update to 'client_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_client_secret_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_url`.\nThe token endpoint of the OAuth client."]
    pub fn set_token_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {
    type O = terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {}

impl BuildAgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {
    pub fn build(self) -> AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {
        AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            client_secret_wo: core::default::Default::default(),
            client_secret_wo_version: core::default::Default::default(),
            token_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauthRef {
        AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID of the OAuth client."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The client secret of the OAuth client."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_wo` after provisioning.\nInput only. The client secret of the OAuth client."]
    pub fn client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret_wo_version` after provisioning.\nTriggers update of 'client_secret_wo' write-only. Increment this value when an update to 'client_secret_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn client_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_wo_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_url` after provisioning.\nThe token endpoint of the OAuth client."]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgentIdentityAuthProvider_AuthProviderTypeParamsDynamic {
    api_key: std::option::Option<terrars::DynamicBlock<AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey>>,
    three_legged_oauth: std
    ::option
    ::Option<terrars::DynamicBlock<AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth>>,
    two_legged_oauth: std
    ::option
    ::Option<terrars::DynamicBlock<AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth>>,
}

#[derive(Serialize)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<std::vec::Vec<AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    three_legged_oauth: std
    ::option
    ::Option<std::vec::Vec<AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    two_legged_oauth: std::option::Option<std::vec::Vec<AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth>>,
    dynamic: AgentIdentityAuthProvider_AuthProviderTypeParamsDynamic,
}

impl AgentIdentityAuthProvider_AuthProviderTypeParams {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `three_legged_oauth`.\n"]
    pub fn set_three_legged_oauth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.three_legged_oauth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.three_legged_oauth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `two_legged_oauth`.\n"]
    pub fn set_two_legged_oauth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.two_legged_oauth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.two_legged_oauth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgentIdentityAuthProvider_AuthProviderTypeParams {
    type O = terrars::BlockAssignable<AgentIdentityAuthProvider_AuthProviderTypeParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentIdentityAuthProvider_AuthProviderTypeParams {}

impl BuildAgentIdentityAuthProvider_AuthProviderTypeParams {
    pub fn build(self) -> AgentIdentityAuthProvider_AuthProviderTypeParams {
        AgentIdentityAuthProvider_AuthProviderTypeParams {
            api_key: core::default::Default::default(),
            three_legged_oauth: core::default::Default::default(),
            two_legged_oauth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentIdentityAuthProvider_AuthProviderTypeParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProvider_AuthProviderTypeParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentIdentityAuthProvider_AuthProviderTypeParamsRef {
        AgentIdentityAuthProvider_AuthProviderTypeParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentIdentityAuthProvider_AuthProviderTypeParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ge_auth_provider` after provisioning.\nMessage describing GeminiEnterpriseAuthProviderParams object.\nSince GeminiEnterpriseAuthProviderParams currently takes no subfields, defining this empty block selects the ge_auth_provider type."]
    pub fn ge_auth_provider(
        &self,
    ) -> terrars::ListRef<AgentIdentityAuthProvider_AuthProviderTypeParams_GeAuthProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ge_auth_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::ListRef<AgentIdentityAuthProvider_AuthProviderTypeParams_ApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `three_legged_oauth` after provisioning.\n"]
    pub fn three_legged_oauth(
        &self,
    ) -> terrars::ListRef<AgentIdentityAuthProvider_AuthProviderTypeParams_ThreeLeggedOauthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.three_legged_oauth", self.base))
    }

    #[doc = "Get a reference to the value of field `two_legged_oauth` after provisioning.\n"]
    pub fn two_legged_oauth(
        &self,
    ) -> terrars::ListRef<AgentIdentityAuthProvider_AuthProviderTypeParams_TwoLeggedOauthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.two_legged_oauth", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentIdentityAuthProvider_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentIdentityAuthProvider_Timeouts {
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

impl terrars::ToListMappable for AgentIdentityAuthProvider_Timeouts {
    type O = terrars::BlockAssignable<AgentIdentityAuthProvider_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentIdentityAuthProvider_Timeouts {}

impl BuildAgentIdentityAuthProvider_Timeouts {
    pub fn build(self) -> AgentIdentityAuthProvider_Timeouts {
        AgentIdentityAuthProvider_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentIdentityAuthProvider_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentIdentityAuthProvider_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentIdentityAuthProvider_TimeoutsRef {
        AgentIdentityAuthProvider_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentIdentityAuthProvider_TimeoutsRef {
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
