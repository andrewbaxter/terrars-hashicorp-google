#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeRegionSecurityPolicyData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeRegionSecurityPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeRegionSecurityPolicyData_>,
}

#[derive(Clone)]
pub struct DataComputeRegionSecurityPolicy(std::rc::Rc<DataComputeRegionSecurityPolicy_>);

impl DataComputeRegionSecurityPolicy {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe Region in which the created Region Security Policy should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ddos_protection_config` after provisioning.\nConfiguration for Google Cloud Armor DDOS Proctection Config."]
    pub fn ddos_protection_config(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_DdosProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ddos_protection_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created Region Security Policy should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rules(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_policy_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy.\n- CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.\n- CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.\n- CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.\nThis field can be set only at resource creation time. Possible values: [\"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_defined_fields` after provisioning.\nDefinitions of user-defined fields for CLOUD_ARMOR_NETWORK policies.\nA user-defined field consists of up to 4 bytes extracted from a fixed offset in the packet, relative to the IPv4, IPv6, TCP, or UDP header, with an optional mask to select certain bits.\nRules may then specify matching values for these fields."]
    pub fn user_defined_fields(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_UserDefinedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeRegionSecurityPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeRegionSecurityPolicy { }

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy {
    type O = terrars::ListRef<DataComputeRegionSecurityPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeRegionSecurityPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_region_security_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeRegionSecurityPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeRegionSecurityPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeRegionSecurityPolicy {
        let out = DataComputeRegionSecurityPolicy(std::rc::Rc::new(DataComputeRegionSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeRegionSecurityPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeRegionSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeRegionSecurityPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `advanced_options_config` after provisioning.\nAdvanced Options Config of this security policy."]
    pub fn advanced_options_config(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ddos_protection_config` after provisioning.\nConfiguration for Google Cloud Armor DDOS Proctection Config."]
    pub fn ddos_protection_config(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_DdosProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ddos_protection_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created Region Security Policy should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nThe set of rules that belong to this policy. There must always be a default rule (rule with priority 2147483647 and match \"*\"). If no rules are provided when creating a security policy, a default rule with action \"allow\" will be added."]
    pub fn rules(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_policy_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy.\n- CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.\n- CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.\n- CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.\nThis field can be set only at resource creation time. Possible values: [\"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_defined_fields` after provisioning.\nDefinitions of user-defined fields for CLOUD_ARMOR_NETWORK policies.\nA user-defined field consists of up to 4 bytes extracted from a fixed offset in the packet, relative to the IPv4, IPv6, TCP, or UDP header, with an optional mask to select certain bits.\nRules may then specify matching values for these fields."]
    pub fn user_defined_fields(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_UserDefinedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    #[doc = "Set the field `content_types`.\n"]
    pub fn set_content_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.content_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {}

impl BuildDataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
        DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
            content_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
        DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
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
pub struct DataComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_custom_config: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_parsing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ip_request_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>,
                    >,
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

    #[doc = "Set the field `user_ip_request_headers`.\n"]
    pub fn set_user_ip_request_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_ip_request_headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_AdvancedOptionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_AdvancedOptionsConfig {}

impl BuildDataComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_AdvancedOptionsConfig {
        DataComputeRegionSecurityPolicy_AdvancedOptionsConfig {
            json_custom_config: core::default::Default::default(),
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
        DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef> {
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

    #[doc = "Get a reference to the value of field `user_ip_request_headers` after provisioning.\n"]
    pub fn user_ip_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_ip_request_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_DdosProtectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ddos_protection: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_DdosProtectionConfig {
    #[doc = "Set the field `ddos_protection`.\n"]
    pub fn set_ddos_protection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ddos_protection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_DdosProtectionConfig {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_DdosProtectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_DdosProtectionConfig {}

impl BuildDataComputeRegionSecurityPolicy_DdosProtectionConfig {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_DdosProtectionConfig {
        DataComputeRegionSecurityPolicy_DdosProtectionConfig { ddos_protection: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_DdosProtectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_DdosProtectionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_DdosProtectionConfigRef {
        DataComputeRegionSecurityPolicy_DdosProtectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_DdosProtectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ddos_protection` after provisioning.\n"]
    pub fn ddos_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ddos_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_Match_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRegionSecurityPolicy_Rules_Match_Config {
    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_Match_Config {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_Match_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_Match_Config {}

impl BuildDataComputeRegionSecurityPolicy_Rules_Match_Config {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_Match_Config {
        DataComputeRegionSecurityPolicy_Rules_Match_Config { src_ip_ranges: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
        DataComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_Match_Expr {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_Match_Expr {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_Match_Expr {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_Match_Expr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_Match_Expr {}

impl BuildDataComputeRegionSecurityPolicy_Rules_Match_Expr {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_Match_Expr {
        DataComputeRegionSecurityPolicy_Rules_Match_Expr { expression: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_Match_ExprRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_Match_ExprRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_Match_ExprRef {
        DataComputeRegionSecurityPolicy_Rules_Match_ExprRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_Match_ExprRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_Match_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_Match_Expr>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned_expr: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_Match {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_Match_Config>>,
    ) -> Self {
        self.config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_Match_Expr>>,
    ) -> Self {
        self.expr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versioned_expr`.\n"]
    pub fn set_versioned_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.versioned_expr = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_Match {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_Match {}

impl BuildDataComputeRegionSecurityPolicy_Rules_Match {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_Match {
        DataComputeRegionSecurityPolicy_Rules_Match {
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            versioned_expr: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_MatchRef {
        DataComputeRegionSecurityPolicy_Rules_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_Match_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_Match_ExprRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }

    #[doc = "Get a reference to the value of field `versioned_expr` after provisioning.\n"]
    pub fn versioned_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {}

impl BuildDataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
        DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
        DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_NetworkMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_asns: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_region_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_fields: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>>,
}

impl DataComputeRegionSecurityPolicy_Rules_NetworkMatch {
    #[doc = "Set the field `dest_ip_ranges`.\n"]
    pub fn set_dest_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_ports`.\n"]
    pub fn set_dest_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_protocols`.\n"]
    pub fn set_ip_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_asns`.\n"]
    pub fn set_src_asns(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.src_asns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_ports`.\n"]
    pub fn set_src_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_region_codes`.\n"]
    pub fn set_src_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>>,
    ) -> Self {
        self.user_defined_fields = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_NetworkMatch {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_NetworkMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_NetworkMatch {}

impl BuildDataComputeRegionSecurityPolicy_Rules_NetworkMatch {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_NetworkMatch {
        DataComputeRegionSecurityPolicy_Rules_NetworkMatch {
            dest_ip_ranges: core::default::Default::default(),
            dest_ports: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
            src_asns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_ports: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
        DataComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\n"]
    pub fn dest_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_ports` after provisioning.\n"]
    pub fn dest_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_protocols` after provisioning.\n"]
    pub fn ip_protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `src_asns` after provisioning.\n"]
    pub fn src_asns(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_asns", self.base))
    }

    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `src_ports` after provisioning.\n"]
    pub fn src_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\n"]
    pub fn src_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_region_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    type O =
        terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    type O =
        terrars
        ::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    type O =
        terrars
        ::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
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

impl terrars
::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    type O =
        terrars
        ::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    type O =
        terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_cookie: std
    ::option
    ::Option<
        terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header: std
    ::option
    ::Option<
        terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_query_param: std
    ::option
    ::Option<
        terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_uri: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_set: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    #[doc = "Set the field `request_body`.\n"]
    pub fn set_request_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody,
                        >,
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
                        ::ListField<
                            DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie,
                        >,
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
                        ::ListField<
                            DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader,
                        >,
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
                        ::ListField<
                            DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam,
                        >,
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
                        terrars
                        ::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri>,
                    >,
    ) -> Self {
        self.request_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_rule_ids`.\n"]
    pub fn set_target_rule_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
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
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_body` after provisioning.\n"]
    pub fn request_body(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }

    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> terrars
    ::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_query_param", self.base))
    }

    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `target_rule_ids` after provisioning.\n"]
    pub fn target_rule_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_rule_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `target_rule_set` after provisioning.\n"]
    pub fn target_rule_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>>,
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>,
                    >,
    ) -> Self {
        self.exclusion = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {}

impl BuildDataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig { exclusion: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
        DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {}

impl BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {}

impl BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
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

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {}

impl BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
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
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_duration_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_threshold: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conform_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_configs: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exceed_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_threshold: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>>,
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    #[doc = "Set the field `ban_duration_sec`.\n"]
    pub fn set_ban_duration_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ban_duration_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>>,
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
                    ::Into<
                        terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>,
                    >,
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

    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>,
                    >,
    ) -> Self {
        self.rate_limit_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions {}

impl BuildDataComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptions {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptions {
            ban_duration_sec: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
        DataComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ban_duration_sec` after provisioning.\n"]
    pub fn ban_duration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ban_duration_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef> {
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
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef> {
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

    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_match: std::option::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_NetworkMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preconfigured_waf_config: std
    ::option
    ::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_options: std::option::Option<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions>>,
}

impl DataComputeRegionSecurityPolicy_Rules {
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

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_match`.\n"]
    pub fn set_network_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_NetworkMatch>>,
    ) -> Self {
        self.network_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>>,
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
        v: impl std::convert::Into<terrars::ListField<DataComputeRegionSecurityPolicy_Rules_RateLimitOptions>>,
    ) -> Self {
        self.rate_limit_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_Rules {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_Rules {}

impl BuildDataComputeRegionSecurityPolicy_Rules {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_Rules {
        DataComputeRegionSecurityPolicy_Rules {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            match_: core::default::Default::default(),
            network_match: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeRegionSecurityPolicy_RulesRef {
        DataComputeRegionSecurityPolicy_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_RulesRef {
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

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `network_match` after provisioning.\n"]
    pub fn network_match(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_NetworkMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_match", self.base))
    }

    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef> {
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
    pub fn rate_limit_options(&self) -> terrars::ListRef<DataComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionSecurityPolicy_UserDefinedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mask: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeRegionSecurityPolicy_UserDefinedFields {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mask`.\n"]
    pub fn set_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mask = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offset`.\n"]
    pub fn set_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionSecurityPolicy_UserDefinedFields {
    type O = terrars::BlockAssignable<DataComputeRegionSecurityPolicy_UserDefinedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionSecurityPolicy_UserDefinedFields {}

impl BuildDataComputeRegionSecurityPolicy_UserDefinedFields {
    pub fn build(self) -> DataComputeRegionSecurityPolicy_UserDefinedFields {
        DataComputeRegionSecurityPolicy_UserDefinedFields {
            base: core::default::Default::default(),
            mask: core::default::Default::default(),
            name: core::default::Default::default(),
            offset: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeRegionSecurityPolicy_UserDefinedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionSecurityPolicy_UserDefinedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionSecurityPolicy_UserDefinedFieldsRef {
        DataComputeRegionSecurityPolicy_UserDefinedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionSecurityPolicy_UserDefinedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `mask` after provisioning.\n"]
    pub fn mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mask", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `offset` after provisioning.\n"]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
