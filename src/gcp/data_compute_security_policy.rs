#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeSecurityPolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeSecurityPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeSecurityPolicyData_>,
}

#[derive(Clone)]
pub struct DataComputeSecurityPolicy(std::rc::Rc<DataComputeSecurityPolicy_>);

impl DataComputeSecurityPolicy {
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

    #[doc = "Set the field `name`.\nThe name of the security policy."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\nThe URI of the created resource."]
    pub fn set_self_link(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `adaptive_protection_config` after provisioning.\nAdaptive Protection Config of this security policy."]
    pub fn adaptive_protection_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_AdaptiveProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `recaptcha_options_config` after provisioning.\nreCAPTCHA configuration options to be applied for the security policy."]
    pub fn recaptcha_options_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_RecaptchaOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rule(&self) -> terrars::SetRef<DataComputeSecurityPolicy_RuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
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
}

impl terrars::Referable for DataComputeSecurityPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeSecurityPolicy { }

impl terrars::ToListMappable for DataComputeSecurityPolicy {
    type O = terrars::ListRef<DataComputeSecurityPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeSecurityPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_security_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeSecurityPolicy {
    pub tf_id: std::string::String,
}

impl BuildDataComputeSecurityPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeSecurityPolicy {
        let out = DataComputeSecurityPolicy(std::rc::Rc::new(DataComputeSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeSecurityPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                self_link: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeSecurityPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `adaptive_protection_config` after provisioning.\nAdaptive Protection Config of this security policy."]
    pub fn adaptive_protection_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_AdaptiveProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `recaptcha_options_config` after provisioning.\nreCAPTCHA configuration options to be applied for the security policy."]
    pub fn recaptcha_options_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_RecaptchaOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rule(&self) -> terrars::SetRef<DataComputeSecurityPolicy_RuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_each_unique_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    #[doc = "Set the field `enable_each_unique_value`.\n"]
    pub fn set_enable_each_unique_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_each_unique_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    type O =
        terrars
        ::BlockAssignable<
            DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {}

impl BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs {
            enable_each_unique_value: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_each_unique_value` after provisioning.\n"]
    pub fn enable_each_unique_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_each_unique_value", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_granularity_configs: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
        >,
    >,
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigs,
                        >,
                    >,
    ) -> Self {
        self.traffic_granularity_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    type O =
        terrars
        ::BlockAssignable<
            DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {}

impl BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
    pub fn build(
        self,
    ) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs {
            auto_deploy_confidence_threshold: core::default::Default::default(),
            auto_deploy_expiration_sec: core::default::Default::default(),
            auto_deploy_impacted_baseline_threshold: core::default::Default::default(),
            auto_deploy_load_threshold: core::default::Default::default(),
            detection_absolute_qps: core::default::Default::default(),
            detection_load_threshold: core::default::Default::default(),
            detection_relative_to_baseline_qps: core::default::Default::default(),
            name: core::default::Default::default(),
            traffic_granularity_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_granularity_configs` after provisioning.\n"]
    pub fn traffic_granularity_configs(
        &self,
    ) -> terrars
    ::ListRef<
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs_TrafficGranularityConfigsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_granularity_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_visibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_configs: std
    ::option
    ::Option<
        terrars
        ::ListField<DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs>,
    >,
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_visibility`.\n"]
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
                        ::ListField<
                            DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigs,
                        >,
                    >,
    ) -> Self {
        self.threshold_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {}

impl BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
    pub fn build(self) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig {
            enable: core::default::Default::default(),
            rule_visibility: core::default::Default::default(),
            threshold_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_visibility` after provisioning.\n"]
    pub fn rule_visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_visibility", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold_configs` after provisioning.\n"]
    pub fn threshold_configs(
        &self,
    ) -> terrars
    ::ListRef<DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig_ThresholdConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layer_7_ddos_defense_config: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>>,
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfig {
    #[doc = "Set the field `layer_7_ddos_defense_config`.\n"]
    pub fn set_layer_7_ddos_defense_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfig>,
                    >,
    ) -> Self {
        self.layer_7_ddos_defense_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_AdaptiveProtectionConfig {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_AdaptiveProtectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig {}

impl BuildDataComputeSecurityPolicy_AdaptiveProtectionConfig {
    pub fn build(self) -> DataComputeSecurityPolicy_AdaptiveProtectionConfig {
        DataComputeSecurityPolicy_AdaptiveProtectionConfig {
            layer_7_ddos_defense_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_AdaptiveProtectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_AdaptiveProtectionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_AdaptiveProtectionConfigRef {
        DataComputeSecurityPolicy_AdaptiveProtectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_AdaptiveProtectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `layer_7_ddos_defense_config` after provisioning.\n"]
    pub fn layer_7_ddos_defense_config(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_AdaptiveProtectionConfig_Layer7DdosDefenseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer_7_ddos_defense_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    #[doc = "Set the field `content_types`.\n"]
    pub fn set_content_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.content_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {}

impl BuildDataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    pub fn build(self) -> DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
        DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
            content_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
        DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_types` after provisioning.\n"]
    pub fn content_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.content_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_AdvancedOptionsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_custom_config: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_parsing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body_inspection_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ip_request_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeSecurityPolicy_AdvancedOptionsConfig {
    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
    ) -> Self {
        self.json_custom_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_parsing`.\n"]
    pub fn set_json_parsing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_body_inspection_size`.\n"]
    pub fn set_request_body_inspection_size(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.request_body_inspection_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_ip_request_headers`.\n"]
    pub fn set_user_ip_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_ip_request_headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_AdvancedOptionsConfig {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_AdvancedOptionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_AdvancedOptionsConfig {}

impl BuildDataComputeSecurityPolicy_AdvancedOptionsConfig {
    pub fn build(self) -> DataComputeSecurityPolicy_AdvancedOptionsConfig {
        DataComputeSecurityPolicy_AdvancedOptionsConfig {
            json_custom_config: core::default::Default::default(),
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            request_body_inspection_size: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_AdvancedOptionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_AdvancedOptionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_AdvancedOptionsConfigRef {
        DataComputeSecurityPolicy_AdvancedOptionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_AdvancedOptionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_custom_config", self.base))
    }

    #[doc = "Get a reference to the value of field `json_parsing` after provisioning.\n"]
    pub fn json_parsing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `request_body_inspection_size` after provisioning.\n"]
    pub fn request_body_inspection_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_body_inspection_size", self.base))
    }

    #[doc = "Get a reference to the value of field `user_ip_request_headers` after provisioning.\n"]
    pub fn user_ip_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_ip_request_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_RecaptchaOptionsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_site_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_RecaptchaOptionsConfig {
    #[doc = "Set the field `redirect_site_key`.\n"]
    pub fn set_redirect_site_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redirect_site_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_RecaptchaOptionsConfig {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_RecaptchaOptionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_RecaptchaOptionsConfig {}

impl BuildDataComputeSecurityPolicy_RecaptchaOptionsConfig {
    pub fn build(self) -> DataComputeSecurityPolicy_RecaptchaOptionsConfig {
        DataComputeSecurityPolicy_RecaptchaOptionsConfig { redirect_site_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_RecaptchaOptionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_RecaptchaOptionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_RecaptchaOptionsConfigRef {
        DataComputeSecurityPolicy_RecaptchaOptionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_RecaptchaOptionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `redirect_site_key` after provisioning.\n"]
    pub fn redirect_site_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_site_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_value`.\n"]
    pub fn set_header_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {}

impl BuildDataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
        DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
        DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\n"]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_adds: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>>,
}

impl DataComputeSecurityPolicy_Rule_HeaderAction {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAdds>>,
    ) -> Self {
        self.request_headers_to_adds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_HeaderAction {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_HeaderAction {}

impl BuildDataComputeSecurityPolicy_Rule_HeaderAction {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_HeaderAction {
        DataComputeSecurityPolicy_Rule_HeaderAction { request_headers_to_adds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_HeaderActionRef {
        DataComputeSecurityPolicy_Rule_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_HeaderActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_HeaderAction_RequestHeadersToAddsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_adds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_Match_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeSecurityPolicy_Rule_Match_Config {
    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_Match_Config {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_Match_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_Match_Config {}

impl BuildDataComputeSecurityPolicy_Rule_Match_Config {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_Match_Config {
        DataComputeSecurityPolicy_Rule_Match_Config { src_ip_ranges: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_Match_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_Match_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_Match_ConfigRef {
        DataComputeSecurityPolicy_Rule_Match_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_Match_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_Match_Expr {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_Match_Expr {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_Match_Expr {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_Match_Expr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_Match_Expr {}

impl BuildDataComputeSecurityPolicy_Rule_Match_Expr {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_Match_Expr {
        DataComputeSecurityPolicy_Rule_Match_Expr { expression: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_Match_ExprRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_Match_ExprRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeSecurityPolicy_Rule_Match_ExprRef {
        DataComputeSecurityPolicy_Rule_Match_ExprRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_Match_ExprRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_token_site_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_token_site_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    #[doc = "Set the field `action_token_site_keys`.\n"]
    pub fn set_action_token_site_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.action_token_site_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_token_site_keys`.\n"]
    pub fn set_session_token_site_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_token_site_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {}

impl BuildDataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
        DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions {
            action_token_site_keys: core::default::Default::default(),
            session_token_site_keys: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
        DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_token_site_keys` after provisioning.\n"]
    pub fn action_token_site_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_token_site_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `session_token_site_keys` after provisioning.\n"]
    pub fn session_token_site_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_token_site_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_Match_ExprOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recaptcha_options: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>>,
}

impl DataComputeSecurityPolicy_Rule_Match_ExprOptions {
    #[doc = "Set the field `recaptcha_options`.\n"]
    pub fn set_recaptcha_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptions>>,
    ) -> Self {
        self.recaptcha_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_Match_ExprOptions {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_Match_ExprOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_Match_ExprOptions {}

impl BuildDataComputeSecurityPolicy_Rule_Match_ExprOptions {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_Match_ExprOptions {
        DataComputeSecurityPolicy_Rule_Match_ExprOptions { recaptcha_options: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
        DataComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_Match_ExprOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recaptcha_options` after provisioning.\n"]
    pub fn recaptcha_options(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_Match_ExprOptions_RecaptchaOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_Expr>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr_options: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_ExprOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned_expr: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_Match {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_Config>>,
    ) -> Self {
        self.config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_Expr>>,
    ) -> Self {
        self.expr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expr_options`.\n"]
    pub fn set_expr_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_Match_ExprOptions>>,
    ) -> Self {
        self.expr_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versioned_expr`.\n"]
    pub fn set_versioned_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.versioned_expr = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_Match {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_Match {}

impl BuildDataComputeSecurityPolicy_Rule_Match {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_Match {
        DataComputeSecurityPolicy_Rule_Match {
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            expr_options: core::default::Default::default(),
            versioned_expr: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeSecurityPolicy_Rule_MatchRef {
        DataComputeSecurityPolicy_Rule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_Match_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_Match_ExprRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }

    #[doc = "Get a reference to the value of field `expr_options` after provisioning.\n"]
    pub fn expr_options(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_Match_ExprOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr_options", self.base))
    }

    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\n"]
    pub fn versioned_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    type O =
        terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_cookie: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_query_param: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_uri: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_set: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    #[doc = "Set the field `request_body`.\n"]
    pub fn set_request_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBody>,
                    >,
    ) -> Self {
        self.request_body = std::option::Option::Some(v.into());
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
                        ::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookie>,
                    >,
    ) -> Self {
        self.request_cookie = std::option::Option::Some(v.into());
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
                        ::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeader>,
                    >,
    ) -> Self {
        self.request_header = std::option::Option::Some(v.into());
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
                        ::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>,
                    >,
    ) -> Self {
        self.request_query_param = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUri>,
                    >,
    ) -> Self {
        self.request_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_rule_ids`.\n"]
    pub fn set_target_rule_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_rule_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_rule_set`.\n"]
    pub fn set_target_rule_set(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_rule_set = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion {
            request_body: core::default::Default::default(),
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            target_rule_ids: core::default::Default::default(),
            target_rule_set: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_body` after provisioning.\n"]
    pub fn request_body(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestBodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }

    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_query_param", self.base))
    }

    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion_RequestUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `target_rule_ids` after provisioning.\n"]
    pub fn target_rule_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_rule_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `target_rule_set` after provisioning.\n"]
    pub fn target_rule_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>>,
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_Exclusion>>,
    ) -> Self {
        self.exclusion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig {}

impl BuildDataComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig { exclusion: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
        DataComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig_ExclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_sec`.\n"]
    pub fn set_interval_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {}

impl BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\n"]
    pub fn interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    #[doc = "Set the field `enforce_on_key_name`.\n"]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key_type`.\n"]
    pub fn set_enforce_on_key_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {}

impl BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\n"]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_on_key_type` after provisioning.\n"]
    pub fn enforce_on_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {}

impl BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions {
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_sec`.\n"]
    pub fn set_interval_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {}

impl BuildDataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
        DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_sec` after provisioning.\n"]
    pub fn interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_duration_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_threshold: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conform_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_configs: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exceed_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exceed_redirect_options: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_threshold: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>>,
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptions {
    #[doc = "Set the field `ban_duration_sec`.\n"]
    pub fn set_ban_duration_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ban_duration_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThreshold>>,
    ) -> Self {
        self.ban_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conform_action`.\n"]
    pub fn set_conform_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.conform_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key`.\n"]
    pub fn set_enforce_on_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enforce_on_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key_configs`.\n"]
    pub fn set_enforce_on_key_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigs>>,
    ) -> Self {
        self.enforce_on_key_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key_name`.\n"]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exceed_action`.\n"]
    pub fn set_exceed_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exceed_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exceed_redirect_options`.\n"]
    pub fn set_exceed_redirect_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptions>>,
    ) -> Self {
        self.exceed_redirect_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThreshold>>,
    ) -> Self {
        self.rate_limit_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_RateLimitOptions {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_RateLimitOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_RateLimitOptions {}

impl BuildDataComputeSecurityPolicy_Rule_RateLimitOptions {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_RateLimitOptions {
        DataComputeSecurityPolicy_Rule_RateLimitOptions {
            ban_duration_sec: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            exceed_redirect_options: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_RateLimitOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_RateLimitOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_RateLimitOptionsRef {
        DataComputeSecurityPolicy_Rule_RateLimitOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_RateLimitOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ban_duration_sec` after provisioning.\n"]
    pub fn ban_duration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ban_duration_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_RateLimitOptions_BanThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ban_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `conform_action` after provisioning.\n"]
    pub fn conform_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conform_action", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_on_key` after provisioning.\n"]
    pub fn enforce_on_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_RateLimitOptions_EnforceOnKeyConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforce_on_key_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_on_key_name` after provisioning.\n"]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `exceed_action` after provisioning.\n"]
    pub fn exceed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exceed_action", self.base))
    }

    #[doc = "Get a reference to the value of field `exceed_redirect_options` after provisioning.\n"]
    pub fn exceed_redirect_options(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_RateLimitOptions_ExceedRedirectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exceed_redirect_options", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_RateLimitOptions_RateLimitThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule_RedirectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeSecurityPolicy_Rule_RedirectOptions {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule_RedirectOptions {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule_RedirectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule_RedirectOptions {}

impl BuildDataComputeSecurityPolicy_Rule_RedirectOptions {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule_RedirectOptions {
        DataComputeSecurityPolicy_Rule_RedirectOptions {
            target: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_Rule_RedirectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_Rule_RedirectOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeSecurityPolicy_Rule_RedirectOptionsRef {
        DataComputeSecurityPolicy_Rule_RedirectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_Rule_RedirectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeSecurityPolicy_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_HeaderAction>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preconfigured_waf_config: std
    ::option
    ::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_options: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_options: std::option::Option<terrars::ListField<DataComputeSecurityPolicy_Rule_RedirectOptions>>,
}

impl DataComputeSecurityPolicy_Rule {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_HeaderAction>>,
    ) -> Self {
        self.header_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfig>>,
    ) -> Self {
        self.preconfigured_waf_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preview`.\n"]
    pub fn set_preview(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_RateLimitOptions>>,
    ) -> Self {
        self.rate_limit_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeSecurityPolicy_Rule_RedirectOptions>>,
    ) -> Self {
        self.redirect_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeSecurityPolicy_Rule {
    type O = terrars::BlockAssignable<DataComputeSecurityPolicy_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeSecurityPolicy_Rule {}

impl BuildDataComputeSecurityPolicy_Rule {
    pub fn build(self) -> DataComputeSecurityPolicy_Rule {
        DataComputeSecurityPolicy_Rule {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            header_action: core::default::Default::default(),
            match_: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
            redirect_options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeSecurityPolicy_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeSecurityPolicy_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeSecurityPolicy_RuleRef {
        DataComputeSecurityPolicy_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeSecurityPolicy_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_PreconfiguredWafConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preconfigured_waf_config", self.base))
    }

    #[doc = "Get a reference to the value of field `preview` after provisioning.\n"]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_RateLimitOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_options", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> terrars::ListRef<DataComputeSecurityPolicy_Rule_RedirectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_options", self.base))
    }
}
