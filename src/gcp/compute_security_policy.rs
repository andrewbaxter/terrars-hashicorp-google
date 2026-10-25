#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeSecurityPolicyData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adaptive_protection_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_AdaptiveProtectionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_options_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_AdvancedOptionsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recaptcha_options_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_RecaptchaOptionsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeSecurityPolicy_Timeouts>,
    dynamic: ComputeSecurityPolicyDynamic,
}

struct ComputeSecurityPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeSecurityPolicyData_>,
}

#[derive(Clone)]
pub struct ComputeSecurityPolicy(std::rc::Rc<ComputeSecurityPolicy_>);

impl ComputeSecurityPolicy {
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

    #[doc = "Set the field `description`.\nAn optional description of this security policy. Max size is 2048."]
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
        "Set the field `labels`.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `adaptive_protection_config`.\n"]
    pub fn set_adaptive_protection_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_AdaptiveProtectionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().adaptive_protection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.adaptive_protection_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_options_config`.\n"]
    pub fn set_advanced_options_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_AdvancedOptionsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_options_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_options_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recaptcha_options_config`.\n"]
    pub fn set_recaptcha_options_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_RecaptchaOptionsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recaptcha_options_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recaptcha_options_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeSecurityPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `adaptive_protection_config` after provisioning.\n"]
    pub fn adaptive_protection_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_AdaptiveProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recaptcha_options_config` after provisioning.\n"]
    pub fn recaptcha_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_RecaptchaOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicy_TimeoutsRef {
        ComputeSecurityPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeSecurityPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeSecurityPolicy { }

impl terrars::ToListMappable for ComputeSecurityPolicy {
    type O = terrars::ListRef<ComputeSecurityPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeSecurityPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_security_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeSecurityPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the security policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeSecurityPolicy {
        let out = ComputeSecurityPolicy(std::rc::Rc::new(ComputeSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeSecurityPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                adaptive_protection_config: core::default::Default::default(),
                advanced_options_config: core::default::Default::default(),
                recaptcha_options_config: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeSecurityPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `adaptive_protection_config` after provisioning.\n"]
    pub fn adaptive_protection_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_AdaptiveProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recaptcha_options_config` after provisioning.\n"]
    pub fn recaptcha_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_RecaptchaOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicy_TimeoutsRef {
        ComputeSecurityPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyDynamic {
    adaptive_protection_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_AdaptiveProtectionConfig>>,
    advanced_options_config: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_AdvancedOptionsConfig>>,
    recaptcha_options_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_RecaptchaOptionsConfig>>,
    rule: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_each_unique_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    #[doc =
        "Set the field `enable_each_unique_value`.\nIf enabled, traffic matching each unique value for the specified type constitutes a separate traffic unit. It can only be set to true if value is empty."]
    pub fn set_enable_each_unique_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_each_unique_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nRequests that match this value constitute a granular traffic unit."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    type O =
        terrars
        ::BlockAssignable<
            ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    #[doc = "Type of this configuration."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    pub fn build(
        self,
    ) -> ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
            enable_each_unique_value: core::default::Default::default(),
            type_: self.type_,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_each_unique_value` after provisioning.\nIf enabled, traffic matching each unique value for the specified type constitutes a separate traffic unit. It can only be set to true if value is empty."]
    pub fn enable_each_unique_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_each_unique_value", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of this configuration."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nRequests that match this value constitute a granular traffic unit."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsDynamic {
    traffic_granularity_configs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_deploy_confidence_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_deploy_expiration_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_deploy_impacted_baseline_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_deploy_load_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detection_absolute_qps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detection_load_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detection_relative_to_baseline_qps: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_granularity_configs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
        >,
    >,
    dynamic: ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsDynamic,
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    #[doc = "Set the field `auto_deploy_confidence_threshold`.\n"]
    pub fn set_auto_deploy_confidence_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auto_deploy_confidence_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_deploy_expiration_sec`.\n"]
    pub fn set_auto_deploy_expiration_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auto_deploy_expiration_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_deploy_impacted_baseline_threshold`.\n"]
    pub fn set_auto_deploy_impacted_baseline_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.auto_deploy_impacted_baseline_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_deploy_load_threshold`.\n"]
    pub fn set_auto_deploy_load_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auto_deploy_load_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detection_absolute_qps`.\n"]
    pub fn set_detection_absolute_qps(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.detection_absolute_qps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detection_load_threshold`.\n"]
    pub fn set_detection_load_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.detection_load_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detection_relative_to_baseline_qps`.\n"]
    pub fn set_detection_relative_to_baseline_qps(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.detection_relative_to_baseline_qps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_granularity_configs`.\n"]
    pub fn set_traffic_granularity_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.traffic_granularity_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.traffic_granularity_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    type O =
        terrars
        ::BlockAssignable<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    #[doc =
        "The name must be 1-63 characters long, and comply with RFC1035. The name must be unique within the security policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    pub fn build(self) -> ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
            auto_deploy_confidence_threshold: core::default::Default::default(),
            auto_deploy_expiration_sec: core::default::Default::default(),
            auto_deploy_impacted_baseline_threshold: core::default::Default::default(),
            auto_deploy_load_threshold: core::default::Default::default(),
            detection_absolute_qps: core::default::Default::default(),
            detection_load_threshold: core::default::Default::default(),
            detection_relative_to_baseline_qps: core::default::Default::default(),
            name: self.name,
            traffic_granularity_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_deploy_confidence_threshold` after provisioning.\n"]
    pub fn auto_deploy_confidence_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_deploy_confidence_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_deploy_expiration_sec` after provisioning.\n"]
    pub fn auto_deploy_expiration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_deploy_expiration_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_deploy_impacted_baseline_threshold` after provisioning.\n"]
    pub fn auto_deploy_impacted_baseline_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_deploy_impacted_baseline_threshold", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `auto_deploy_load_threshold` after provisioning.\n"]
    pub fn auto_deploy_load_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_deploy_load_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `detection_absolute_qps` after provisioning.\n"]
    pub fn detection_absolute_qps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detection_absolute_qps", self.base))
    }

    #[doc = "Get a reference to the value of field `detection_load_threshold` after provisioning.\n"]
    pub fn detection_load_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detection_load_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `detection_relative_to_baseline_qps` after provisioning.\n"]
    pub fn detection_relative_to_baseline_qps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detection_relative_to_baseline_qps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name must be 1-63 characters long, and comply with RFC1035. The name must be unique within the security policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_granularity_configs` after provisioning.\n"]
    pub fn traffic_granularity_configs(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_granularity_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigDynamic {
    threshold_configs: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs>,
    >,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_visibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_configs: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs>>,
    dynamic: ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigDynamic,
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    #[doc = "Set the field `enable`.\nIf set to true, enables CAAP for L7 DDoS detection."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_visibility`.\nRule visibility. Supported values include: \"STANDARD\", \"PREMIUM\"."]
    pub fn set_rule_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_configs`.\n"]
    pub fn set_threshold_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threshold_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threshold_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {}

impl BuildComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    pub fn build(self) -> ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
            enable: core::default::Default::default(),
            rule_visibility: core::default::Default::default(),
            threshold_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
        ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nIf set to true, enables CAAP for L7 DDoS detection."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_visibility` after provisioning.\nRule visibility. Supported values include: \"STANDARD\", \"PREMIUM\"."]
    pub fn rule_visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_visibility", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold_configs` after provisioning.\n"]
    pub fn threshold_configs(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_AdaptiveProtectionConfigDynamic {
    layer_7_ddos_defense_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layer_7_ddos_defense_config: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>>,
    dynamic: ComputeSecurityPolicy_AdaptiveProtectionConfigDynamic,
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfig {
    #[doc = "Set the field `layer_7_ddos_defense_config`.\n"]
    pub fn set_layer_7_ddos_defense_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layer_7_ddos_defense_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layer_7_ddos_defense_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_AdaptiveProtectionConfig {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_AdaptiveProtectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_AdaptiveProtectionConfig {}

impl BuildComputeSecurityPolicy_AdaptiveProtectionConfig {
    pub fn build(self) -> ComputeSecurityPolicy_AdaptiveProtectionConfig {
        ComputeSecurityPolicy_AdaptiveProtectionConfig {
            layer_7_ddos_defense_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_AdaptiveProtectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_AdaptiveProtectionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_AdaptiveProtectionConfigRef {
        ComputeSecurityPolicy_AdaptiveProtectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_AdaptiveProtectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `layer_7_ddos_defense_config` after provisioning.\n"]
    pub fn layer_7_ddos_defense_config(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer_7_ddos_defense_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    content_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig { }

impl terrars::ToListMappable for ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    #[doc = "A list of custom Content-Type header values to apply the JSON parsing."]
    pub content_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    pub fn build(self) -> ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
        ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig { content_types: self.content_types }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
        ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_types` after provisioning.\nA list of custom Content-Type header values to apply the JSON parsing."]
    pub fn content_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.content_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_AdvancedOptionsConfigDynamic {
    json_custom_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_AdvancedOptionsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_parsing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body_inspection_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ip_request_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_custom_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
    dynamic: ComputeSecurityPolicy_AdvancedOptionsConfigDynamic,
}

impl ComputeSecurityPolicy_AdvancedOptionsConfig {
    #[doc = "Set the field `json_parsing`.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\"."]
    pub fn set_json_parsing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\"."]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_body_inspection_size`.\nThe maximum request size chosen by the customer with Waf enabled. Values supported are \"8KB\", \"16KB, \"32KB\", \"48KB\" and \"64KB\". Values are case insensitive."]
    pub fn set_request_body_inspection_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.request_body_inspection_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_ip_request_headers`.\nAn optional list of case-insensitive request header names to use for resolving the callers client IP address."]
    pub fn set_user_ip_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_ip_request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_custom_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_custom_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_AdvancedOptionsConfig {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_AdvancedOptionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_AdvancedOptionsConfig {}

impl BuildComputeSecurityPolicy_AdvancedOptionsConfig {
    pub fn build(self) -> ComputeSecurityPolicy_AdvancedOptionsConfig {
        ComputeSecurityPolicy_AdvancedOptionsConfig {
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            request_body_inspection_size: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
            json_custom_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_AdvancedOptionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_AdvancedOptionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_AdvancedOptionsConfigRef {
        ComputeSecurityPolicy_AdvancedOptionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_AdvancedOptionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `json_parsing` after provisioning.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\"."]
    pub fn json_parsing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_level` after provisioning.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\"."]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_body_inspection_size` after provisioning.\nThe maximum request size chosen by the customer with Waf enabled. Values supported are \"8KB\", \"16KB, \"32KB\", \"48KB\" and \"64KB\". Values are case insensitive."]
    pub fn request_body_inspection_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_body_inspection_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_ip_request_headers` after provisioning.\nAn optional list of case-insensitive request header names to use for resolving the callers client IP address."]
    pub fn user_ip_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_ip_request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_custom_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_RecaptchaOptionsConfig {
    redirect_site_key: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicy_RecaptchaOptionsConfig { }

impl terrars::ToListMappable for ComputeSecurityPolicy_RecaptchaOptionsConfig {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_RecaptchaOptionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_RecaptchaOptionsConfig {
    #[doc =
        "A field to supply a reCAPTCHA site key to be used for all the rules using the redirect action with the type of GOOGLE_RECAPTCHA under the security policy. The specified site key needs to be created from the reCAPTCHA API. The user is responsible for the validity of the specified site key. If not specified, a Google-managed site key is used."]
    pub redirect_site_key: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_RecaptchaOptionsConfig {
    pub fn build(self) -> ComputeSecurityPolicy_RecaptchaOptionsConfig {
        ComputeSecurityPolicy_RecaptchaOptionsConfig { redirect_site_key: self.redirect_site_key }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_RecaptchaOptionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_RecaptchaOptionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_RecaptchaOptionsConfigRef {
        ComputeSecurityPolicy_RecaptchaOptionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_RecaptchaOptionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `redirect_site_key` after provisioning.\nA field to supply a reCAPTCHA site key to be used for all the rules using the redirect action with the type of GOOGLE_RECAPTCHA under the security policy. The specified site key needs to be created from the reCAPTCHA API. The user is responsible for the validity of the specified site key. If not specified, a Google-managed site key is used."]
    pub fn redirect_site_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_site_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    header_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    #[doc = "Set the field `header_value`.\nThe value to set the named header to."]
    pub fn set_header_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    #[doc = "The name of the header to set."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
        ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
            header_name: self.header_name,
            header_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
        ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to set."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_value` after provisioning.\nThe value to set the named header to."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_Rule_HeaderActionDynamic {
    request_headers_to_adds: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_adds: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>>,
    dynamic: ComputeSecurityPolicy_Rule_HeaderActionDynamic,
}

impl ComputeSecurityPolicy_Rule_HeaderAction {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_adds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_adds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_HeaderAction {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_HeaderAction {}

impl BuildComputeSecurityPolicy_Rule_HeaderAction {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_HeaderAction {
        ComputeSecurityPolicy_Rule_HeaderAction {
            request_headers_to_adds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_HeaderActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_Rule_HeaderActionRef {
        ComputeSecurityPolicy_Rule_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_HeaderActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_adds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_Match_Config {
    src_ip_ranges: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_Match_Config { }

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_Match_Config {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_Match_Config {
    #[doc =
        "Set of IP addresses or ranges (IPV4 or IPV6) in CIDR notation to match against inbound traffic. There is a limit of 10 IP ranges per rule. A value of '*' matches all IPs (can be used to override the default behavior)."]
    pub src_ip_ranges: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeSecurityPolicy_Rule_Match_Config {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_Match_Config {
        ComputeSecurityPolicy_Rule_Match_Config { src_ip_ranges: self.src_ip_ranges }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_Match_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_Match_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_Rule_Match_ConfigRef {
        ComputeSecurityPolicy_Rule_Match_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_Match_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSet of IP addresses or ranges (IPV4 or IPV6) in CIDR notation to match against inbound traffic. There is a limit of 10 IP ranges per rule. A value of '*' matches all IPs (can be used to override the default behavior)."]
    pub fn src_ip_ranges(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_Match_Expr {
    expression: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicy_Rule_Match_Expr { }

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_Match_Expr {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_Expr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_Match_Expr {
    #[doc =
        "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_Match_Expr {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_Match_Expr {
        ComputeSecurityPolicy_Rule_Match_Expr { expression: self.expression }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_Match_ExprRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_Match_ExprRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_Rule_Match_ExprRef {
        ComputeSecurityPolicy_Rule_Match_ExprRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_Match_ExprRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_token_site_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_token_site_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    #[doc =
        "Set the field `action_token_site_keys`.\nA list of site keys to be used during the validation of reCAPTCHA action-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created"]
    pub fn set_action_token_site_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.action_token_site_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_token_site_keys`.\nA list of site keys to be used during the validation of reCAPTCHA session-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created."]
    pub fn set_session_token_site_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_token_site_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {}

impl BuildComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
        ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
            action_token_site_keys: core::default::Default::default(),
            session_token_site_keys: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
        ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_token_site_keys` after provisioning.\nA list of site keys to be used during the validation of reCAPTCHA action-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created"]
    pub fn action_token_site_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_token_site_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_token_site_keys` after provisioning.\nA list of site keys to be used during the validation of reCAPTCHA session-tokens. The provided site keys need to be created from reCAPTCHA API under the same project where the security policy is created."]
    pub fn session_token_site_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_token_site_keys", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_Rule_Match_ExprOptionsDynamic {
    recaptcha_options: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_Match_ExprOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recaptcha_options: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>>,
    dynamic: ComputeSecurityPolicy_Rule_Match_ExprOptionsDynamic,
}

impl ComputeSecurityPolicy_Rule_Match_ExprOptions {
    #[doc = "Set the field `recaptcha_options`.\n"]
    pub fn set_recaptcha_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recaptcha_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recaptcha_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_Match_ExprOptions {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_ExprOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_Match_ExprOptions {}

impl BuildComputeSecurityPolicy_Rule_Match_ExprOptions {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_Match_ExprOptions {
        ComputeSecurityPolicy_Rule_Match_ExprOptions {
            recaptcha_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
        ComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recaptcha_options` after provisioning.\n"]
    pub fn recaptcha_options(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_Rule_MatchDynamic {
    config: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_Match_Config>>,
    expr: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_Match_Expr>>,
    expr_options: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_Match_ExprOptions>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned_expr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_Match_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_Match_Expr>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr_options: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_Match_ExprOptions>>,
    dynamic: ComputeSecurityPolicy_Rule_MatchDynamic,
}

impl ComputeSecurityPolicy_Rule_Match {
    #[doc =
        "Set the field `versioned_expr`.\nPredefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config."]
    pub fn set_versioned_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.versioned_expr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_Expr>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expr = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expr_options`.\n"]
    pub fn set_expr_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match_ExprOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expr_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expr_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_Match {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_Match {}

impl BuildComputeSecurityPolicy_Rule_Match {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_Match {
        ComputeSecurityPolicy_Rule_Match {
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            expr_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_Rule_MatchRef {
        ComputeSecurityPolicy_Rule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `versioned_expr` after provisioning.\nPredefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config."]
    pub fn versioned_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_Match_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_Match_ExprRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }

    #[doc = "Get a reference to the value of field `expr_options` after provisioning.\n"]
    pub fn expr_options(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_Match_ExprOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value. Available options: EQUALS: The operator matches if the field value equals the specified value. STARTS_WITH: The operator matches if the field value starts with the specified value. ENDS_WITH: The operator matches if the field value ends with the specified value. CONTAINS: The operator matches if the field value contains the specified value. EQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation. The field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionDynamic {
    request_body: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>>,
    request_cookie: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>>,
    request_header: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>>,
    request_query_param: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>>,
    request_uri: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    target_rule_set: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_cookie: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_query_param: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_uri: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>>,
    dynamic: ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionDynamic,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    #[doc =
        "Set the field `target_rule_ids`.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion. If omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn set_target_rule_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_rule_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_body`.\n"]
    pub fn set_request_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_cookie = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_header`.\n"]
    pub fn set_request_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_query_param`.\n"]
    pub fn set_request_query_param(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_query_param = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_query_param = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_uri`.\n"]
    pub fn set_request_uri(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_uri = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_uri = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
            target_rule_ids: core::default::Default::default(),
            target_rule_set: self.target_rule_set,
            request_body: core::default::Default::default(),
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_rule_ids` after provisioning.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion. If omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn target_rule_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_rule_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_rule_set` after provisioning.\nTarget WAF rule set to apply the preconfigured WAF exclusion."]
    pub fn target_rule_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `request_body` after provisioning.\n"]
    pub fn request_body(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }

    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_query_param", self.base))
    }

    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfigDynamic {
    exclusion: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>>,
    dynamic: ComputeSecurityPolicy_Rule_PreconfiguredWafConfigDynamic,
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig {}

impl BuildComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
            exclusion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
        ComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    count: terrars::PrimField<f64>,
    interval_sec: terrars::PrimField<f64>,
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold { }

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    #[doc = "Number of HTTP(S) requests for calculating the threshold."]
    pub count: terrars::PrimField<f64>,
    #[doc = "Interval over which the threshold is computed."]
    pub interval_sec: terrars::PrimField<f64>,
}

impl BuildComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
        ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
            count: self.count,
            interval_sec: self.interval_sec,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
        ComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    #[doc =
        "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key_type`.\nDetermines the key to enforce the rate_limit_threshold on"]
    pub fn set_enforce_on_key_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {}

impl BuildComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
        ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
        ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_type` after provisioning.\nDetermines the key to enforce the rate_limit_threshold on"]
    pub fn enforce_on_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    #[doc =
        "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    #[doc = "Type of the redirect action."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
        ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
            target: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
        ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    count: terrars::PrimField<f64>,
    interval_sec: terrars::PrimField<f64>,
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold { }

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    #[doc = "Number of HTTP(S) requests for calculating the threshold."]
    pub count: terrars::PrimField<f64>,
    #[doc = "Interval over which the threshold is computed."]
    pub interval_sec: terrars::PrimField<f64>,
}

impl BuildComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
        ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
            count: self.count,
            interval_sec: self.interval_sec,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
        ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_Rule_RateLimitOptionsDynamic {
    ban_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>>,
    enforce_on_key_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>>,
    exceed_redirect_options: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>>,
    rate_limit_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_duration_sec: std::option::Option<terrars::PrimField<f64>>,
    conform_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    exceed_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_threshold: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_configs: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exceed_redirect_options: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_threshold: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>>,
    dynamic: ComputeSecurityPolicy_Rule_RateLimitOptionsDynamic,
}

impl ComputeSecurityPolicy_Rule_RateLimitOptions {
    #[doc =
        "Set the field `ban_duration_sec`.\nCan only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn set_ban_duration_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ban_duration_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key`.\nDetermines the key to enforce the rateLimitThreshold on"]
    pub fn set_enforce_on_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enforce_on_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ban_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ban_threshold = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enforce_on_key_configs`.\n"]
    pub fn set_enforce_on_key_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enforce_on_key_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enforce_on_key_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exceed_redirect_options`.\n"]
    pub fn set_exceed_redirect_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exceed_redirect_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exceed_redirect_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rate_limit_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_threshold = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_RateLimitOptions {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_RateLimitOptions {
    #[doc =
        "Action to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only."]
    pub conform_action: terrars::PrimField<std::string::String>,
    #[doc =
        "Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are \"deny()\" where valid values for status are 403, 404, 429, and 502, and \"redirect\" where the redirect parameters come from exceedRedirectOptions below."]
    pub exceed_action: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_RateLimitOptions {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_RateLimitOptions {
        ComputeSecurityPolicy_Rule_RateLimitOptions {
            ban_duration_sec: core::default::Default::default(),
            conform_action: self.conform_action,
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: self.exceed_action,
            ban_threshold: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            exceed_redirect_options: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_RateLimitOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_RateLimitOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicy_Rule_RateLimitOptionsRef {
        ComputeSecurityPolicy_Rule_RateLimitOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_RateLimitOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ban_duration_sec` after provisioning.\nCan only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn ban_duration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ban_duration_sec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `conform_action` after provisioning.\nAction to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only."]
    pub fn conform_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conform_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on"]
    pub fn enforce_on_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are \"deny()\" where valid values for status are 403, 404, 429, and 502, and \"redirect\" where the redirect parameters come from exceedRedirectOptions below."]
    pub fn exceed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exceed_action", self.base))
    }

    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ban_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforce_on_key_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `exceed_redirect_options` after provisioning.\n"]
    pub fn exceed_redirect_options(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exceed_redirect_options", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule_RedirectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicy_Rule_RedirectOptions {
    #[doc =
        "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule_RedirectOptions {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RedirectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule_RedirectOptions {
    #[doc =
        "Type of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy_Rule_RedirectOptions {
    pub fn build(self) -> ComputeSecurityPolicy_Rule_RedirectOptions {
        ComputeSecurityPolicy_Rule_RedirectOptions {
            target: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_Rule_RedirectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_Rule_RedirectOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_Rule_RedirectOptionsRef {
        ComputeSecurityPolicy_Rule_RedirectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_Rule_RedirectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicy_RuleDynamic {
    header_action: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_HeaderAction>>,
    match_: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_Match>>,
    preconfigured_waf_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig>>,
    rate_limit_options: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_RateLimitOptions>>,
    redirect_options: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicy_Rule_RedirectOptions>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Rule {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_HeaderAction>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preconfigured_waf_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_options: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_RateLimitOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_options: std::option::Option<std::vec::Vec<ComputeSecurityPolicy_Rule_RedirectOptions>>,
    dynamic: ComputeSecurityPolicy_RuleDynamic,
}

impl ComputeSecurityPolicy_Rule {
    #[doc = "Set the field `description`.\nAn optional description of this rule. Max size is 64."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preview`.\nWhen set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such."]
    pub fn set_preview(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_HeaderAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_PreconfiguredWafConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.preconfigured_waf_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.preconfigured_waf_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RateLimitOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rate_limit_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicy_Rule_RedirectOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicy_Rule {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Rule {
    #[doc = "Action to take when match matches the request."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "An unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeSecurityPolicy_Rule {
    pub fn build(self) -> ComputeSecurityPolicy_Rule {
        ComputeSecurityPolicy_Rule {
            action: self.action,
            description: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: self.priority,
            header_action: core::default::Default::default(),
            match_: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
            redirect_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_RuleRef {
        ComputeSecurityPolicy_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take when match matches the request."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this rule. Max size is 64."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nWhen set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preconfigured_waf_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_RateLimitOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_options", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> terrars::ListRef<ComputeSecurityPolicy_Rule_RedirectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicy_Timeouts {
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

impl terrars::ToListMappable for ComputeSecurityPolicy_Timeouts {
    type O = terrars::BlockAssignable<ComputeSecurityPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicy_Timeouts {}

impl BuildComputeSecurityPolicy_Timeouts {
    pub fn build(self) -> ComputeSecurityPolicy_Timeouts {
        ComputeSecurityPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeSecurityPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicy_TimeoutsRef {
        ComputeSecurityPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicy_TimeoutsRef {
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
