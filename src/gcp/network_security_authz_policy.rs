#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkSecurityAuthzPolicyData_ {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_provider: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_CustomProvider>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_rules: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_rules: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_Target>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkSecurityAuthzPolicy_Timeouts>,
    dynamic: NetworkSecurityAuthzPolicyDynamic,
}

struct NetworkSecurityAuthzPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkSecurityAuthzPolicyData_>,
}

#[derive(Clone)]
pub struct NetworkSecurityAuthzPolicy(std::rc::Rc<NetworkSecurityAuthzPolicy_>);

impl NetworkSecurityAuthzPolicy {
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

    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
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
        "Set the field `labels`.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policy_profile`.\nDefines the type of authorization being performed. 'REQUEST_AUTHZ' applies to request authorization. CUSTOM\nauthorization policies with Authz extensions will be allowed with ext_authz or ext_proc protocols. Extensions are\ninvoked only once when the request headers arrive. 'CONTENT_AUTHZ' applies to content security, sanitization, etc.\nOnly CUSTOM action is allowed in this policy profile. AuthzExtensions in the custom provider must support ext_proc\nprotocol and be capable of receiving all ext_proc events (REQUEST_HEADERS, REQUEST_BODY, REQUEST_TRAILERS,\nRESPONSE_HEADERS, RESPONSE_BODY, RESPONSE_TRAILERS) with FULL_DUPLEX_STREAMED body send mode. Possible values: [\"REQUEST_AUTHZ\", \"CONTENT_AUTHZ\"]"]
    pub fn set_policy_profile(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_provider`.\n"]
    pub fn set_custom_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_CustomProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_rules`.\n"]
    pub fn set_http_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_rules`.\n"]
    pub fn set_network_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_Target>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkSecurityAuthzPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nWhen the action is CUSTOM, customProvider must be specified.\nWhen the action is ALLOW, only requests matching the policy will be allowed.\nWhen the action is DENY, only requests matching the policy will be denied.\n\nWhen a request arrives, the policies are evaluated in the following order:\n1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.\n2. If there are any DENY policies that match the request, the request is denied.\n3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.\n4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request. Possible values: [\"ALLOW\", \"DENY\", \"CUSTOM\"]"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the AuthzPolicy resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_profile` after provisioning.\nDefines the type of authorization being performed. 'REQUEST_AUTHZ' applies to request authorization. CUSTOM\nauthorization policies with Authz extensions will be allowed with ext_authz or ext_proc protocols. Extensions are\ninvoked only once when the request headers arrive. 'CONTENT_AUTHZ' applies to content security, sanitization, etc.\nOnly CUSTOM action is allowed in this policy profile. AuthzExtensions in the custom provider must support ext_proc\nprotocol and be capable of receiving all ext_proc events (REQUEST_HEADERS, REQUEST_BODY, REQUEST_TRAILERS,\nRESPONSE_HEADERS, RESPONSE_BODY, RESPONSE_TRAILERS) with FULL_DUPLEX_STREAMED body send mode. Possible values: [\"REQUEST_AUTHZ\", \"CONTENT_AUTHZ\"]"]
    pub fn policy_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_provider` after provisioning.\n"]
    pub fn custom_provider(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_CustomProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_rules` after provisioning.\n"]
    pub fn http_rules(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_rules` after provisioning.\n"]
    pub fn network_rules(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityAuthzPolicy_TimeoutsRef {
        NetworkSecurityAuthzPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkSecurityAuthzPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkSecurityAuthzPolicy { }

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy {
    type O = terrars::ListRef<NetworkSecurityAuthzPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkSecurityAuthzPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_security_authz_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkSecurityAuthzPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "When the action is CUSTOM, customProvider must be specified.\nWhen the action is ALLOW, only requests matching the policy will be allowed.\nWhen the action is DENY, only requests matching the policy will be denied.\n\nWhen a request arrives, the policies are evaluated in the following order:\n1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.\n2. If there are any DENY policies that match the request, the request is denied.\n3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.\n4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request. Possible values: [\"ALLOW\", \"DENY\", \"CUSTOM\"]"]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Identifier. Name of the AuthzPolicy resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecurityAuthzPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkSecurityAuthzPolicy {
        let out = NetworkSecurityAuthzPolicy(std::rc::Rc::new(NetworkSecurityAuthzPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkSecurityAuthzPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                policy_profile: core::default::Default::default(),
                project: core::default::Default::default(),
                custom_provider: core::default::Default::default(),
                http_rules: core::default::Default::default(),
                network_rules: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkSecurityAuthzPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkSecurityAuthzPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nWhen the action is CUSTOM, customProvider must be specified.\nWhen the action is ALLOW, only requests matching the policy will be allowed.\nWhen the action is DENY, only requests matching the policy will be denied.\n\nWhen a request arrives, the policies are evaluated in the following order:\n1. If there is a CUSTOM policy that matches the request, the CUSTOM policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request.\n2. If there are any DENY policies that match the request, the request is denied.\n3. If there are no ALLOW policies for the resource or if any of the ALLOW policies match the request, the request is allowed.\n4. Else the request is denied by default if none of the configured AuthzPolicies with ALLOW action match the request. Possible values: [\"ALLOW\", \"DENY\", \"CUSTOM\"]"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the AuthzExtension resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the AuthzPolicy resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_profile` after provisioning.\nDefines the type of authorization being performed. 'REQUEST_AUTHZ' applies to request authorization. CUSTOM\nauthorization policies with Authz extensions will be allowed with ext_authz or ext_proc protocols. Extensions are\ninvoked only once when the request headers arrive. 'CONTENT_AUTHZ' applies to content security, sanitization, etc.\nOnly CUSTOM action is allowed in this policy profile. AuthzExtensions in the custom provider must support ext_proc\nprotocol and be capable of receiving all ext_proc events (REQUEST_HEADERS, REQUEST_BODY, REQUEST_TRAILERS,\nRESPONSE_HEADERS, RESPONSE_BODY, RESPONSE_TRAILERS) with FULL_DUPLEX_STREAMED body send mode. Possible values: [\"REQUEST_AUTHZ\", \"CONTENT_AUTHZ\"]"]
    pub fn policy_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_provider` after provisioning.\n"]
    pub fn custom_provider(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_CustomProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_rules` after provisioning.\n"]
    pub fn http_rules(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_rules` after provisioning.\n"]
    pub fn network_rules(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecurityAuthzPolicy_TimeoutsRef {
        NetworkSecurityAuthzPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicyDynamic {
    custom_provider: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_CustomProvider>>,
    http_rules: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules>>,
    network_rules: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules>>,
    target: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_Target>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension {
    resources: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension { }

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension {
    #[doc =
        "A list of references to authorization extensions that will be invoked for requests matching this policy. Limited to 1 custom provider."]
    pub resources: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension {
        NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension { resources: self.resources }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtensionRef {
        NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of references to authorization extensions that will be invoked for requests matching this policy. Limited to 1 custom provider."]
    pub fn resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_CustomProvider_CloudIap {
    enabled: terrars::PrimField<bool>,
}

impl NetworkSecurityAuthzPolicy_CustomProvider_CloudIap { }

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_CustomProvider_CloudIap {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_CustomProvider_CloudIap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_CustomProvider_CloudIap {
    #[doc = "Enable Cloud IAP at the AuthzPolicy level."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildNetworkSecurityAuthzPolicy_CustomProvider_CloudIap {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_CustomProvider_CloudIap {
        NetworkSecurityAuthzPolicy_CustomProvider_CloudIap { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_CustomProvider_CloudIapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_CustomProvider_CloudIapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_CustomProvider_CloudIapRef {
        NetworkSecurityAuthzPolicy_CustomProvider_CloudIapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_CustomProvider_CloudIapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnable Cloud IAP at the AuthzPolicy level."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_CustomProviderDynamic {
    authz_extension: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension>>,
    cloud_iap: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_CustomProvider_CloudIap>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_CustomProvider {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authz_extension: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_iap: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_CustomProvider_CloudIap>>,
    dynamic: NetworkSecurityAuthzPolicy_CustomProviderDynamic,
}

impl NetworkSecurityAuthzPolicy_CustomProvider {
    #[doc = "Set the field `authz_extension`.\n"]
    pub fn set_authz_extension(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authz_extension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authz_extension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_iap`.\n"]
    pub fn set_cloud_iap(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_CustomProvider_CloudIap>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_iap = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_iap = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_CustomProvider {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_CustomProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_CustomProvider {}

impl BuildNetworkSecurityAuthzPolicy_CustomProvider {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_CustomProvider {
        NetworkSecurityAuthzPolicy_CustomProvider {
            authz_extension: core::default::Default::default(),
            cloud_iap: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_CustomProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_CustomProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_CustomProviderRef {
        NetworkSecurityAuthzPolicy_CustomProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_CustomProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authz_extension` after provisioning.\n"]
    pub fn authz_extension(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_CustomProvider_AuthzExtensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authz_extension", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_iap` after provisioning.\n"]
    pub fn cloud_iap(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_CustomProvider_CloudIapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_iap", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks {
    length: terrars::PrimField<f64>,
    prefix: terrars::PrimField<std::string::String>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks { }

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks {
    #[doc = "The length of the address range."]
    pub length: terrars::PrimField<f64>,
    #[doc = "The address prefix."]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks {
            length: self.length,
            prefix: self.prefix,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nThe length of the address range."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nThe address prefix."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_PrincipalRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsDynamic {
    principal: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_selector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `principal_selector`.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn set_principal_selector(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.principal_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_Principal>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            principal_selector: core::default::Default::default(),
            suffix: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `principal_selector` after provisioning.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn principal_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_selector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccountRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {
    #[doc =
        "Set the field `ids`.\nA list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.\nLimited to 5 matches."]
    pub fn set_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet {
            ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSetRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ids` after provisioning.\nA list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.\nLimited to 5 matches."]
    pub fn ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesDynamic {
    iam_service_account: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount>>,
    tag_value_id_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_service_account: std
    ::option
    ::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_value_id_set: std
    ::option
    ::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {
    #[doc = "Set the field `iam_service_account`.\n"]
    pub fn set_iam_service_account(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iam_service_account = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_service_account = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_value_id_set`.\n"]
    pub fn set_tag_value_id_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSet>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_value_id_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_value_id_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources {
            iam_service_account: core::default::Default::default(),
            tag_value_id_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_service_account` after provisioning.\n"]
    pub fn iam_service_account(
        &self,
    ) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_IamServiceAccountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_value_id_set` after provisioning.\n"]
    pub fn tag_value_id_set(
        &self,
    ) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources_TagValueIdSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_value_id_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesDynamic {
    ip_blocks: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    principals: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_NotSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_blocks: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_NotSources {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_NotSources {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_NotSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_NotSources {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_NotSources {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_NotSources {
        NetworkSecurityAuthzPolicy_HttpRules_From_NotSources {
            ip_blocks: core::default::Default::default(),
            principals: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_From_SourcesDynamic {
    ip_blocks: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    principals: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_Sources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_blocks: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_From_SourcesDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_Sources {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Principals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From_Sources {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From_Sources {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From_Sources {
        NetworkSecurityAuthzPolicy_HttpRules_From_Sources {
            ip_blocks: core::default::Default::default(),
            principals: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From_SourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_From_SourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_From_SourcesRef {
        NetworkSecurityAuthzPolicy_HttpRules_From_SourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_From_SourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_PrincipalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_FromDynamic {
    not_sources: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_NotSources>>,
    sources: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_From {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_sources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_NotSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_FromDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_From {
    #[doc = "Set the field `not_sources`.\n"]
    pub fn set_not_sources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_NotSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_From {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_From {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_From {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_From {
        NetworkSecurityAuthzPolicy_HttpRules_From {
            not_sources: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_FromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_FromRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_HttpRules_FromRef {
        NetworkSecurityAuthzPolicy_HttpRules_FromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_FromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_sources` after provisioning.\n"]
    pub fn not_sources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_NotSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_ValueRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersDynamic {
    value: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {
    #[doc = "Set the field `name`.\nSpecifies the name of the header in the request."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_Value>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the header in the request."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(
        &self,
    ) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetDynamic {
    headers: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_Headers>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet {
            headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsDynamic {
    header_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>>,
    hosts: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>>,
    paths: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_set: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {
    #[doc =
        "Set the field `methods`.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_set`.\n"]
    pub fn set_header_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.paths = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {
        NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations {
            methods: core::default::Default::default(),
            header_set: core::default::Default::default(),
            hosts: core::default::Default::default(),
            paths: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc = "Get a reference to the value of field `header_set` after provisioning.\n"]
    pub fn header_set(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_set", self.base))
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {
    #[doc = "Set the field `contains`.\nA substring match on the MCP method parameter name."]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\nAn exact match on the MCP method parameter name."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_case`.\nSpecifies that the string match should be case insensitive."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nA prefix match on the MCP method parameter name."]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\nA suffix match on the MCP method parameter name."]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_ParamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_ParamsRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nA substring match on the MCP method parameter name."]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nAn exact match on the MCP method parameter name."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that the string match should be case insensitive."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nA prefix match on the MCP method parameter name."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nA suffix match on the MCP method parameter name."]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsDynamic {
    params: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_Params>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
    #[doc =
        "The MCP method to match against. Allowed values are as follows:\n1) “tools”, “prompts”, “resources” - these will match against all sub methods under the respective methods.\n2) “prompts/list”, “tools/list”, “resources/list”, “resources/templates/list”\n3) “prompts/get”, “tools/call”, “resources/subscribe”, “resources/unsubscribe”, “resources/read”\nParams cannot be specified for categories 1) and 2)."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods {
            name: self.name,
            params: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe MCP method to match against. Allowed values are as follows:\n1) “tools”, “prompts”, “resources” - these will match against all sub methods under the respective methods.\n2) “prompts/list”, “tools/list”, “resources/list”, “resources/templates/list”\n3) “prompts/get”, “tools/call”, “resources/subscribe”, “resources/unsubscribe”, “resources/read”\nParams cannot be specified for categories 1) and 2)."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpDynamic {
    methods: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_protocol_methods_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {
    #[doc =
        "Set the field `base_protocol_methods_option`.\nIf specified, matches on the MCP protocol’s non-access specific methods namely: * initialize/ * completion/ * logging/ * notifications/ * ping Default value: \"SKIP_BASE_PROTOCOL_METHODS\" Possible values: [\"SKIP_BASE_PROTOCOL_METHODS\", \"MATCH_BASE_PROTOCOL_METHODS\"]"]
    pub fn set_base_protocol_methods_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.base_protocol_methods_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `methods`.\n"]
    pub fn set_methods(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_Methods>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.methods = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.methods = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp {
            base_protocol_methods_option: core::default::Default::default(),
            methods: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_protocol_methods_option` after provisioning.\nIf specified, matches on the MCP protocol’s non-access specific methods namely: * initialize/ * completion/ * logging/ * notifications/ * ping Default value: \"SKIP_BASE_PROTOCOL_METHODS\" Possible values: [\"SKIP_BASE_PROTOCOL_METHODS\", \"MATCH_BASE_PROTOCOL_METHODS\"]"]
    pub fn base_protocol_methods_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_protocol_methods_option", self.base))
    }

    #[doc = "Get a reference to the value of field `methods` after provisioning.\n"]
    pub fn methods(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp_MethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_To_OperationsDynamic {
    header_set: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>>,
    hosts: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>>,
    mcp: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp>>,
    paths: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_set: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosts: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paths: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_To_OperationsDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_Operations {
    #[doc =
        "Set the field `methods`.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_set`.\n"]
    pub fn set_header_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hosts`.\n"]
    pub fn set_hosts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Hosts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mcp`.\n"]
    pub fn set_mcp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Mcp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mcp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mcp = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `paths`.\n"]
    pub fn set_paths(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_Paths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.paths = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To_Operations {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To_Operations {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To_Operations {
        NetworkSecurityAuthzPolicy_HttpRules_To_Operations {
            methods: core::default::Default::default(),
            header_set: core::default::Default::default(),
            hosts: core::default::Default::default(),
            mcp: core::default::Default::default(),
            paths: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_To_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_HttpRules_To_OperationsRef {
        NetworkSecurityAuthzPolicy_HttpRules_To_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_To_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nA list of HTTP methods to match against. Each entry must be a valid HTTP method name (GET, PUT, POST, HEAD, PATCH, DELETE, OPTIONS). It only allows exact match and is always case sensitive."]
    pub fn methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc = "Get a reference to the value of field `header_set` after provisioning.\n"]
    pub fn header_set(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HeaderSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_set", self.base))
    }

    #[doc = "Get a reference to the value of field `hosts` after provisioning.\n"]
    pub fn hosts(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_HostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc = "Get a reference to the value of field `mcp` after provisioning.\n"]
    pub fn mcp(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_McpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp", self.base))
    }

    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_Operations_PathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRules_ToDynamic {
    not_operations: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations>>,
    operations: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To_Operations>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_To {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_operations: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To_Operations>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRules_ToDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules_To {
    #[doc = "Set the field `not_operations`.\n"]
    pub fn set_not_operations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_NotOperations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_operations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To_Operations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules_To {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules_To {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules_To {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules_To {
        NetworkSecurityAuthzPolicy_HttpRules_To {
            not_operations: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRules_ToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRules_ToRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_HttpRules_ToRef {
        NetworkSecurityAuthzPolicy_HttpRules_ToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRules_ToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_operations` after provisioning.\n"]
    pub fn not_operations(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_NotOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_operations", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_To_OperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_HttpRulesDynamic {
    from: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From>>,
    to: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_To>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_HttpRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    when: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_To>>,
    dynamic: NetworkSecurityAuthzPolicy_HttpRulesDynamic,
}

impl NetworkSecurityAuthzPolicy_HttpRules {
    #[doc =
        "Set the field `when`.\nCEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes."]
    pub fn set_when(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.when = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from`.\n"]
    pub fn set_from(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_To>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_HttpRules {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_HttpRules {}

impl BuildNetworkSecurityAuthzPolicy_HttpRules {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_HttpRules {
        NetworkSecurityAuthzPolicy_HttpRules {
            when: core::default::Default::default(),
            from: core::default::Default::default(),
            to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_HttpRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_HttpRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_HttpRulesRef {
        NetworkSecurityAuthzPolicy_HttpRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_HttpRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `when` after provisioning.\nCEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes."]
    pub fn when(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.when", self.base))
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_FromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_ToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {
    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {
        NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal {
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_PrincipalRef {
        NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsDynamic {
    principal: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_selector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std
    ::option
    ::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {
    #[doc =
        "Set the field `principal_selector`.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn set_principal_selector(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.principal_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_Principal>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {
        NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals {
            principal_selector: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef {
        NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `principal_selector` after provisioning.\nAn enum to decide what principal value the principal rule will match against. If not specified, the PrincipalSelector is CLIENT_CERT_URI_SAN. Default value: \"CLIENT_CERT_URI_SAN\" Possible values: [\"PRINCIPAL_SELECTOR_UNSPECIFIED\", \"CLIENT_CERT_URI_SAN\", \"CLIENT_CERT_DNS_NAME_SAN\", \"CLIENT_CERT_COMMON_NAME\"]"]
    pub fn principal_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesDynamic {
    ip_blocks: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    principals: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_blocks: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {
        NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources {
            ip_blocks: core::default::Default::default(),
            principals: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesRef {
        NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesDynamic {
    ip_blocks: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    principals: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_Sources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_blocks: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principals: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_Sources {
    #[doc = "Set the field `ip_blocks`.\n"]
    pub fn set_ip_blocks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_blocks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_blocks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_Principals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_From_Sources {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_From_Sources {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_From_Sources {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_From_Sources {
        NetworkSecurityAuthzPolicy_NetworkRules_From_Sources {
            ip_blocks: core::default::Default::default(),
            principals: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesRef {
        NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_blocks` after provisioning.\n"]
    pub fn ip_blocks(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_IpBlocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources_PrincipalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_HttpRules_From_Sources_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_FromDynamic {
    not_sources: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources>>,
    sources: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_From {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_sources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_FromDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_From {
    #[doc = "Set the field `not_sources`.\n"]
    pub fn set_not_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_NotSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From_Sources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_From {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_From {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_From {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_From {
        NetworkSecurityAuthzPolicy_NetworkRules_From {
            not_sources: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_FromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_FromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_FromRef {
        NetworkSecurityAuthzPolicy_NetworkRules_FromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_FromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_sources` after provisioning.\n"]
    pub fn not_sources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_From_NotSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_From_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {
    #[doc =
        "Set the field `contains`.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact`.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix`.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix`.\nThe input string must have the suffix specified here. Note: empty suffix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {
        NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef {
        NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contains` after provisioning.\nThe input string must have the substring specified here. Note: empty contains match is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc.def"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exact` after provisioning.\nThe input string must match exactly the string specified here.\nExamples:\n* abc only matches the value abc."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nIf true, indicates the exact/prefix/suffix/contains matching should be case insensitive. For example, the matcher data will match both input string Data and data if set to true."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nThe input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead.\nExamples:\n* abc matches the value abc.xyz"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix` after provisioning.\nThe input string must have the suffix specified here. Note: empty suffix is not allowed, please use regex instead.\nExamples:\n* abc matches the value xyz.abc"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsDynamic {
    snis: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snis: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {
    #[doc = "Set the field `snis`.\n"]
    pub fn set_snis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {
        NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations {
            snis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsRef {
        NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `snis` after provisioning.\n"]
    pub fn snis(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snis", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsDynamic {
    snis: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snis: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To_Operations {
    #[doc = "Set the field `snis`.\n"]
    pub fn set_snis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_Snis>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_To_Operations {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_To_Operations {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_To_Operations {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_To_Operations {
        NetworkSecurityAuthzPolicy_NetworkRules_To_Operations {
            snis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsRef {
        NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `snis` after provisioning.\n"]
    pub fn snis(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations_SnisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snis", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRules_ToDynamic {
    not_operations: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations>>,
    operations: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_To {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_operations: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRules_ToDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules_To {
    #[doc = "Set the field `not_operations`.\n"]
    pub fn set_not_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_operations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To_Operations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules_To {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules_To {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules_To {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules_To {
        NetworkSecurityAuthzPolicy_NetworkRules_To {
            not_operations: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules_ToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRules_ToRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_NetworkRules_ToRef {
        NetworkSecurityAuthzPolicy_NetworkRules_ToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRules_ToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_operations` after provisioning.\n"]
    pub fn not_operations(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_To_NotOperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_operations", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_To_OperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecurityAuthzPolicy_NetworkRulesDynamic {
    from: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_From>>,
    to: std::option::Option<terrars::DynamicBlock<NetworkSecurityAuthzPolicy_NetworkRules_To>>,
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_NetworkRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    when: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_From>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<std::vec::Vec<NetworkSecurityAuthzPolicy_NetworkRules_To>>,
    dynamic: NetworkSecurityAuthzPolicy_NetworkRulesDynamic,
}

impl NetworkSecurityAuthzPolicy_NetworkRules {
    #[doc =
        "Set the field `when`.\nCEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes."]
    pub fn set_when(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.when = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from`.\n"]
    pub fn set_from(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_From>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules_To>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_NetworkRules {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_NetworkRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_NetworkRules {}

impl BuildNetworkSecurityAuthzPolicy_NetworkRules {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_NetworkRules {
        NetworkSecurityAuthzPolicy_NetworkRules {
            when: core::default::Default::default(),
            from: core::default::Default::default(),
            to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_NetworkRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_NetworkRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_NetworkRulesRef {
        NetworkSecurityAuthzPolicy_NetworkRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_NetworkRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `when` after provisioning.\nCEL expression that describes the conditions to be satisfied for the action. The result of the CEL expression is ANDed with the from and to. Refer to the CEL language reference for a list of available attributes."]
    pub fn when(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.when", self.base))
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_FromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::ListRef<NetworkSecurityAuthzPolicy_NetworkRules_ToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkSecurityAuthzPolicy_Target {
    #[doc =
        "Set the field `load_balancing_scheme`.\nRequired when targeting forwarding rules and secure web proxy. Must not be specified when targeting Agent\nGateway. All resources referenced by this policy and extensions must share the same load balancing scheme.\nFor more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn set_load_balancing_scheme(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.load_balancing_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of references to the Forwarding Rules or Secure Web Proxy Gateways or Agent Gateways on which this\npolicy will be applied."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_Target {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_Target {}

impl BuildNetworkSecurityAuthzPolicy_Target {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_Target {
        NetworkSecurityAuthzPolicy_Target {
            load_balancing_scheme: core::default::Default::default(),
            resources: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_TargetRef {
        NetworkSecurityAuthzPolicy_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nRequired when targeting forwarding rules and secure web proxy. Must not be specified when targeting Agent\nGateway. All resources referenced by this policy and extensions must share the same load balancing scheme.\nFor more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). Possible values: [\"INTERNAL_MANAGED\", \"EXTERNAL_MANAGED\", \"INTERNAL_SELF_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of references to the Forwarding Rules or Secure Web Proxy Gateways or Agent Gateways on which this\npolicy will be applied."]
    pub fn resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecurityAuthzPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecurityAuthzPolicy_Timeouts {
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

impl terrars::ToListMappable for NetworkSecurityAuthzPolicy_Timeouts {
    type O = terrars::BlockAssignable<NetworkSecurityAuthzPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecurityAuthzPolicy_Timeouts {}

impl BuildNetworkSecurityAuthzPolicy_Timeouts {
    pub fn build(self) -> NetworkSecurityAuthzPolicy_Timeouts {
        NetworkSecurityAuthzPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecurityAuthzPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecurityAuthzPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecurityAuthzPolicy_TimeoutsRef {
        NetworkSecurityAuthzPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecurityAuthzPolicy_TimeoutsRef {
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
