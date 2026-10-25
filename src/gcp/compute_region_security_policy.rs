#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionSecurityPolicyData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_options_config: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_AdvancedOptionsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ddos_protection_config: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_DdosProtectionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionSecurityPolicy_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_fields: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_UserDefinedFields>>,
    dynamic: ComputeRegionSecurityPolicyDynamic,
}

struct ComputeRegionSecurityPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionSecurityPolicyData_>,
}

#[derive(Clone)]
pub struct ComputeRegionSecurityPolicy(std::rc::Rc<ComputeRegionSecurityPolicy_>);

impl ComputeRegionSecurityPolicy {
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

    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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
        "Set the field `type_`.\nThe type indicates the intended use of the security policy.\n- CLOUD_ARMOR: Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers.\n- CLOUD_ARMOR_EDGE: Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache.\n- CLOUD_ARMOR_NETWORK: Cloud Armor network policies can be configured to filter packets targeting network load balancing resources such as backend services, target pools, target instances, and instances with external IPs. They filter requests before the request is served from the application.\nThis field can be set only at resource creation time. Possible values: [\"CLOUD_ARMOR\", \"CLOUD_ARMOR_EDGE\", \"CLOUD_ARMOR_NETWORK\"]"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_options_config`.\n"]
    pub fn set_advanced_options_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_AdvancedOptionsConfig>>,
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

    #[doc = "Set the field `ddos_protection_config`.\n"]
    pub fn set_ddos_protection_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_DdosProtectionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ddos_protection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ddos_protection_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionSecurityPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_UserDefinedFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_defined_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_defined_fields = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ddos_protection_config` after provisioning.\n"]
    pub fn ddos_protection_config(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_DdosProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ddos_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionSecurityPolicy_TimeoutsRef {
        ComputeRegionSecurityPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_UserDefinedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRegionSecurityPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRegionSecurityPolicy { }

impl terrars::ToListMappable for ComputeRegionSecurityPolicy {
    type O = terrars::ListRef<ComputeRegionSecurityPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRegionSecurityPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_region_security_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionSecurityPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRegionSecurityPolicy {
        let out = ComputeRegionSecurityPolicy(std::rc::Rc::new(ComputeRegionSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRegionSecurityPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
                advanced_options_config: core::default::Default::default(),
                ddos_protection_config: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_defined_fields: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionSecurityPolicyRef {
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

    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ddos_protection_config` after provisioning.\n"]
    pub fn ddos_protection_config(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_DdosProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ddos_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionSecurityPolicy_TimeoutsRef {
        ComputeRegionSecurityPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_UserDefinedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicyDynamic {
    advanced_options_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_AdvancedOptionsConfig>>,
    ddos_protection_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_DdosProtectionConfig>>,
    rules: std::option::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules>>,
    user_defined_fields: std::option::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_UserDefinedFields>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    content_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig { }

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    #[doc = "A list of custom Content-Type header values to apply the JSON parsing."]
    pub content_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
    pub fn build(self) -> ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig {
        ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig { content_types: self.content_types }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
        ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef {
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
struct ComputeRegionSecurityPolicy_AdvancedOptionsConfigDynamic {
    json_custom_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_parsing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ip_request_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_custom_config: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>>,
    dynamic: ComputeRegionSecurityPolicy_AdvancedOptionsConfigDynamic,
}

impl ComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    #[doc =
        "Set the field `json_parsing`.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\". Possible values: [\"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\"]"]
    pub fn set_json_parsing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `log_level`.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\". Possible values: [\"NORMAL\", \"VERBOSE\"]"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
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
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfig>,
                    >,
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

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_AdvancedOptionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_AdvancedOptionsConfig {}

impl BuildComputeRegionSecurityPolicy_AdvancedOptionsConfig {
    pub fn build(self) -> ComputeRegionSecurityPolicy_AdvancedOptionsConfig {
        ComputeRegionSecurityPolicy_AdvancedOptionsConfig {
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            user_ip_request_headers: core::default::Default::default(),
            json_custom_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
        ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_AdvancedOptionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `json_parsing` after provisioning.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\". Possible values: [\"DISABLED\", \"STANDARD\", \"STANDARD_WITH_GRAPHQL\"]"]
    pub fn json_parsing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_level` after provisioning.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\". Possible values: [\"NORMAL\", \"VERBOSE\"]"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_ip_request_headers` after provisioning.\nAn optional list of case-insensitive request header names to use for resolving the callers client IP address."]
    pub fn user_ip_request_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_ip_request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_AdvancedOptionsConfig_JsonCustomConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_custom_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_DdosProtectionConfig {
    ddos_protection: terrars::PrimField<std::string::String>,
}

impl ComputeRegionSecurityPolicy_DdosProtectionConfig { }

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_DdosProtectionConfig {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_DdosProtectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_DdosProtectionConfig {
    #[doc =
        "Google Cloud Armor offers the following options to help protect systems against DDoS attacks:\n- STANDARD: basic always-on protection for network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED: additional protections for Managed Protection Plus subscribers who use network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED_PREVIEW: flag to enable the security policy in preview mode. Possible values: [\"ADVANCED\", \"ADVANCED_PREVIEW\", \"STANDARD\"]"]
    pub ddos_protection: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_DdosProtectionConfig {
    pub fn build(self) -> ComputeRegionSecurityPolicy_DdosProtectionConfig {
        ComputeRegionSecurityPolicy_DdosProtectionConfig { ddos_protection: self.ddos_protection }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_DdosProtectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_DdosProtectionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_DdosProtectionConfigRef {
        ComputeRegionSecurityPolicy_DdosProtectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_DdosProtectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ddos_protection` after provisioning.\nGoogle Cloud Armor offers the following options to help protect systems against DDoS attacks:\n- STANDARD: basic always-on protection for network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED: additional protections for Managed Protection Plus subscribers who use network load balancers, protocol forwarding, or VMs with public IP addresses.\n- ADVANCED_PREVIEW: flag to enable the security policy in preview mode. Possible values: [\"ADVANCED\", \"ADVANCED_PREVIEW\", \"STANDARD\"]"]
    pub fn ddos_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ddos_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_Match_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeRegionSecurityPolicy_Rules_Match_Config {
    #[doc = "Set the field `src_ip_ranges`.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_Match_Config {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_Match_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_Match_Config {}

impl BuildComputeRegionSecurityPolicy_Rules_Match_Config {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_Match_Config {
        ComputeRegionSecurityPolicy_Rules_Match_Config { src_ip_ranges: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
        ComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_Match_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of srcIpRanges allowed is 10."]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_Match_Expr {
    expression: terrars::PrimField<std::string::String>,
}

impl ComputeRegionSecurityPolicy_Rules_Match_Expr { }

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_Match_Expr {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_Match_Expr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_Match_Expr {
    #[doc =
        "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_Match_Expr {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_Match_Expr {
        ComputeRegionSecurityPolicy_Rules_Match_Expr { expression: self.expression }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_Match_ExprRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_Match_ExprRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_Match_ExprRef {
        ComputeRegionSecurityPolicy_Rules_Match_ExprRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_Match_ExprRef {
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

#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicy_Rules_MatchDynamic {
    config: std::option::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_Match_Config>>,
    expr: std::option::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_Match_Expr>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned_expr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_Match_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_Match_Expr>>,
    dynamic: ComputeRegionSecurityPolicy_Rules_MatchDynamic,
}

impl ComputeRegionSecurityPolicy_Rules_Match {
    #[doc =
        "Set the field `versioned_expr`.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn set_versioned_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.versioned_expr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_Match_Config>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_Match_Expr>>,
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
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_Match {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_Match {}

impl BuildComputeRegionSecurityPolicy_Rules_Match {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_Match {
        ComputeRegionSecurityPolicy_Rules_Match {
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionSecurityPolicy_Rules_MatchRef {
        ComputeRegionSecurityPolicy_Rules_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `versioned_expr` after provisioning.\nPreconfigured versioned expression. If this field is specified, config must also be specified.\nAvailable preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding srcIpRange field in config. Possible values: [\"SRC_IPS_V1\"]"]
    pub fn versioned_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_Match_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_Match_ExprRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    #[doc = "Set the field `name`.\nName of the user-defined field, as given in the definition."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `values`.\nMatching values of the field. Each element can be a 32-bit unsigned decimal or hexadecimal (starting with \"0x\") number (e.g. \"64\") or range (e.g. \"0x400-0x7ff\")."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {}

impl BuildComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
        ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
        ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the user-defined field, as given in the definition."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nMatching values of the field. Each element can be a 32-bit unsigned decimal or hexadecimal (starting with \"0x\") number (e.g. \"64\") or range (e.g. \"0x400-0x7ff\")."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicy_Rules_NetworkMatchDynamic {
    user_defined_fields: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_NetworkMatch {
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
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>>,
    dynamic: ComputeRegionSecurityPolicy_Rules_NetworkMatchDynamic,
}

impl ComputeRegionSecurityPolicy_Rules_NetworkMatch {
    #[doc =
        "Set the field `dest_ip_ranges`.\nDestination IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn set_dest_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_ports`.\nDestination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn set_dest_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_protocols`.\nIPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. \"6\"), range (e.g. \"253-254\"), or one of the following protocol names: \"tcp\", \"udp\", \"icmp\", \"esp\", \"ah\", \"ipip\", or \"sctp\"."]
    pub fn set_ip_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_asns`.\nBGP Autonomous System Number associated with the source IP address."]
    pub fn set_src_asns(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.src_asns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_ip_ranges`.\nSource IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_ports`.\nSource port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn set_src_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_region_codes`.\nTwo-letter ISO 3166-1 alpha-2 country code associated with the source IP address."]
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
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFields>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_defined_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_NetworkMatch {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_NetworkMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_NetworkMatch {}

impl BuildComputeRegionSecurityPolicy_Rules_NetworkMatch {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_NetworkMatch {
        ComputeRegionSecurityPolicy_Rules_NetworkMatch {
            dest_ip_ranges: core::default::Default::default(),
            dest_ports: core::default::Default::default(),
            ip_protocols: core::default::Default::default(),
            src_asns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_ports: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
        ComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_NetworkMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nDestination IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn dest_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_ports` after provisioning.\nDestination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn dest_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_protocols` after provisioning.\nIPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. \"6\"), range (e.g. \"253-254\"), or one of the following protocol names: \"tcp\", \"udp\", \"icmp\", \"esp\", \"ah\", \"ipip\", or \"sctp\"."]
    pub fn ip_protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_protocols", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_asns` after provisioning.\nBGP Autonomous System Number associated with the source IP address."]
    pub fn src_asns(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_asns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IPv4/IPv6 addresses or CIDR prefixes, in standard text format."]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_ports` after provisioning.\nSource port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. \"80\") or range (e.g. \"0-1023\")."]
    pub fn src_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_region_codes` after provisioning.\nTwo-letter ISO 3166-1 alpha-2 country code associated with the source IP address."]
    pub fn src_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_region_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_NetworkMatch_UserDefinedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    type O =
        terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    type O =
        terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    type O =
        terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value. Possible values: [\"CONTAINS\", \"ENDS_WITH\", \"EQUALS\", \"EQUALS_ANY\", \"STARTS_WITH\"]"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionDynamic {
    request_body: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody>>,
    request_cookie: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie>>,
    request_header: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader>>,
    request_query_param: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam>,
    >,
    request_uri: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    target_rule_set: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_cookie: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_query_param: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_uri: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri>>,
    dynamic: ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionDynamic,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    #[doc =
        "Set the field `target_rule_ids`.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn set_target_rule_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
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
                        ::BlockAssignable<
                            ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBody,
                        >,
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
                        ::BlockAssignable<
                            ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookie,
                        >,
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
                        ::BlockAssignable<
                            ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeader,
                        >,
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
                            ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParam,
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
                        ::BlockAssignable<
                            ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUri,
                        >,
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

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion {
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
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_rule_ids` after provisioning.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn target_rule_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_rule_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_rule_set` after provisioning.\nTarget WAF rule set to apply the preconfigured WAF exclusion."]
    pub fn target_rule_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `request_body` after provisioning.\n"]
    pub fn request_body(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestBodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }

    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_query_param", self.base))
    }

    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion_RequestUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigDynamic {
    exclusion: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>>,
    dynamic: ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigDynamic,
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_Exclusion>,
                    >,
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

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {}

impl BuildComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig {
            exclusion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
        ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig_ExclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    #[doc = "Set the field `count`.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_sec`.\nInterval over which the threshold is computed."]
    pub fn set_interval_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {}

impl BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef {
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
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    #[doc =
        "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_on_key_type`.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKeyConfigs\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn set_enforce_on_key_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {}

impl BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs {
            enforce_on_key_name: core::default::Default::default(),
            enforce_on_key_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_type` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKeyConfigs\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn enforce_on_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    #[doc = "Set the field `count`.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_sec`.\nInterval over which the threshold is computed."]
    pub fn set_interval_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {}

impl BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold {
            count: core::default::Default::default(),
            interval_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef {
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
struct ComputeRegionSecurityPolicy_Rules_RateLimitOptionsDynamic {
    ban_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>>,
    enforce_on_key_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>>,
    rate_limit_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_duration_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conform_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exceed_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_threshold: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_configs: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_threshold: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>>,
    dynamic: ComputeRegionSecurityPolicy_Rules_RateLimitOptionsDynamic,
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    #[doc =
        "Set the field `ban_duration_sec`.\nCan only be specified if the action for the rule is \"rate_based_ban\".\nIf specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn set_ban_duration_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ban_duration_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `conform_action`.\nAction to take for requests that are under the configured rate limit threshold.\nValid option is \"allow\" only."]
    pub fn set_conform_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.conform_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_on_key`.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn set_enforce_on_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enforce_on_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exceed_action`.\nAction to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn set_exceed_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exceed_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThreshold>>,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigs>,
                    >,
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

    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThreshold>,
                    >,
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

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions {}

impl BuildComputeRegionSecurityPolicy_Rules_RateLimitOptions {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptions {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptions {
            ban_duration_sec: core::default::Default::default(),
            conform_action: core::default::Default::default(),
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: core::default::Default::default(),
            ban_threshold: core::default::Default::default(),
            enforce_on_key_configs: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
        ComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ban_duration_sec` after provisioning.\nCan only be specified if the action for the rule is \"rate_based_ban\".\nIf specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn ban_duration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ban_duration_sec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `conform_action` after provisioning.\nAction to take for requests that are under the configured rate limit threshold.\nValid option is \"allow\" only."]
    pub fn conform_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conform_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on. Possible values are:\n* ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if \"enforceOnKey\" is not configured.\n* IP: The source IP address of the request is the key. Each IP has this limit enforced separately.\n* HTTP_HEADER: The value of the HTTP header whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL.\n* XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key defaults to the source IP address of the request i.e. key type IP.\n* HTTP_COOKIE: The value of the HTTP cookie whose name is configured under \"enforceOnKeyName\". The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL.\n* HTTP_PATH: The URL path of the HTTP request. The key value is truncated to the first 128 bytes.\n* SNI: Server name indication in the TLS session of the HTTPS request. The key value is truncated to the first 128 bytes. The key type defaults to ALL on a HTTP session.\n* REGION_CODE: The country/region from which the request originates.\n* TLS_JA3_FINGERPRINT: JA3 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* TLS_JA4_FINGERPRINT: JA4 TLS/SSL fingerprint if the client connects using HTTPS, HTTP/2 or HTTP/3. If not available, the key type defaults to ALL.\n* USER_IP: The IP address of the originating client, which is resolved based on \"userIpRequestHeaders\" configured with the security policy. If there is no \"userIpRequestHeaders\" configuration or an IP address cannot be resolved from it, the key type defaults to IP. Possible values: [\"ALL\", \"IP\", \"HTTP_HEADER\", \"XFF_IP\", \"HTTP_COOKIE\", \"HTTP_PATH\", \"SNI\", \"REGION_CODE\", \"TLS_JA3_FINGERPRINT\", \"TLS_JA4_FINGERPRINT\", \"USER_IP\"]"]
    pub fn enforce_on_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types:\nHTTP_HEADER -- Name of the HTTP header whose value is taken as the key value.\nHTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to deny with a specified HTTP response code.\nValid options are deny(STATUS), where valid values for STATUS are 403, 404, 429, and 502."]
    pub fn exceed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exceed_action", self.base))
    }

    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_BanThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ban_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_on_key_configs` after provisioning.\n"]
    pub fn enforce_on_key_configs(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_EnforceOnKeyConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforce_on_key_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_RateLimitOptions_RateLimitThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionSecurityPolicy_RulesDynamic {
    match_: std::option::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_Match>>,
    network_match: std::option::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_NetworkMatch>>,
    preconfigured_waf_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>>,
    rate_limit_options: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionSecurityPolicy_Rules_RateLimitOptions>>,
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Rules {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    priority: terrars::PrimField<f64>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_match: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_NetworkMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preconfigured_waf_config: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_options: std::option::Option<std::vec::Vec<ComputeRegionSecurityPolicy_Rules_RateLimitOptions>>,
    dynamic: ComputeRegionSecurityPolicy_RulesDynamic,
}

impl ComputeRegionSecurityPolicy_Rules {
    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preview`.\nIf set to true, the specified action is not enforced."]
    pub fn set_preview(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_Match>>,
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

    #[doc = "Set the field `network_match`.\n"]
    pub fn set_network_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_NetworkMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules_RateLimitOptions>>,
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
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Rules {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Rules {
    #[doc =
        "The Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "An integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeRegionSecurityPolicy_Rules {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Rules {
        ComputeRegionSecurityPolicy_Rules {
            action: self.action,
            description: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: self.priority,
            match_: core::default::Default::default(),
            network_match: core::default::Default::default(),
            preconfigured_waf_config: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionSecurityPolicy_RulesRef {
        ComputeRegionSecurityPolicy_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the rule is matched. The following are the valid actions:\n\n* allow: allow access to target.\n\n* deny(STATUS): deny access to target, returns the HTTP response code specified. Valid values for STATUS are 403, 404, and 502.\n\n* rate_based_ban: limit client traffic to the configured threshold and ban the client if the traffic exceeds the threshold. Configure parameters for this action in RateLimitOptions. Requires rateLimitOptions to be set.\n\n* redirect: redirect to a different target. This can either be an internal reCAPTCHA redirect, or an external URL-based redirect via a 302 response. Parameters for this action can be configured via redirectOptions. This action is only supported in Global Security Policies of type CLOUD_ARMOR.\n\n* throttle: limit client traffic to the configured threshold. Configure parameters for this action in rateLimitOptions. Requires rateLimitOptions to be set for this."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list.\nThe priority must be a positive value between 0 and 2147483647.\nRules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest priority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `network_match` after provisioning.\n"]
    pub fn network_match(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_NetworkMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_match", self.base))
    }

    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_PreconfiguredWafConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preconfigured_waf_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> terrars::ListRef<ComputeRegionSecurityPolicy_Rules_RateLimitOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionSecurityPolicy_Timeouts {
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

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_Timeouts {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_Timeouts {}

impl BuildComputeRegionSecurityPolicy_Timeouts {
    pub fn build(self) -> ComputeRegionSecurityPolicy_Timeouts {
        ComputeRegionSecurityPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionSecurityPolicy_TimeoutsRef {
        ComputeRegionSecurityPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ComputeRegionSecurityPolicy_UserDefinedFields {
    base: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mask: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionSecurityPolicy_UserDefinedFields {
    #[doc =
        "Set the field `mask`.\nIf specified, apply this mask (bitwise AND) to the field to ignore bits before matching.\nEncoded as a hexadecimal number (starting with \"0x\").\nThe last byte of the field (in network byte order) corresponds to the least significant byte of the mask."]
    pub fn set_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mask = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of this field. Must be unique within the policy."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `offset`.\nOffset of the first byte of the field (in network byte order) relative to 'base'."]
    pub fn set_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\nSize of the field in bytes. Valid values: 1-4."]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionSecurityPolicy_UserDefinedFields {
    type O = terrars::BlockAssignable<ComputeRegionSecurityPolicy_UserDefinedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionSecurityPolicy_UserDefinedFields {
    #[doc =
        "The base relative to which 'offset' is measured. Possible values are:\n- IPV4: Points to the beginning of the IPv4 header.\n- IPV6: Points to the beginning of the IPv6 header.\n- TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.\n- UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. Possible values: [\"IPV4\", \"IPV6\", \"TCP\", \"UDP\"]"]
    pub base: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionSecurityPolicy_UserDefinedFields {
    pub fn build(self) -> ComputeRegionSecurityPolicy_UserDefinedFields {
        ComputeRegionSecurityPolicy_UserDefinedFields {
            base: self.base,
            mask: core::default::Default::default(),
            name: core::default::Default::default(),
            offset: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionSecurityPolicy_UserDefinedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionSecurityPolicy_UserDefinedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionSecurityPolicy_UserDefinedFieldsRef {
        ComputeRegionSecurityPolicy_UserDefinedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionSecurityPolicy_UserDefinedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base` after provisioning.\nThe base relative to which 'offset' is measured. Possible values are:\n- IPV4: Points to the beginning of the IPv4 header.\n- IPV6: Points to the beginning of the IPv6 header.\n- TCP: Points to the beginning of the TCP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments.\n- UDP: Points to the beginning of the UDP header, skipping over any IPv4 options or IPv6 extension headers. Not present for non-first fragments. Possible values: [\"IPV4\", \"IPV6\", \"TCP\", \"UDP\"]"]
    pub fn base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mask` after provisioning.\nIf specified, apply this mask (bitwise AND) to the field to ignore bits before matching.\nEncoded as a hexadecimal number (starting with \"0x\").\nThe last byte of the field (in network byte order) corresponds to the least significant byte of the mask."]
    pub fn mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mask", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this field. Must be unique within the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `offset` after provisioning.\nOffset of the first byte of the field (in network byte order) relative to 'base'."]
    pub fn offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size` after provisioning.\nSize of the field in bytes. Valid values: 1-4."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}
