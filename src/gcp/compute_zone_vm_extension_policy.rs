#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeZoneVmExtensionPolicyData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extension_policies: std::option::Option<std::vec::Vec<ComputeZoneVmExtensionPolicy_ExtensionPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selectors: std::option::Option<std::vec::Vec<ComputeZoneVmExtensionPolicy_InstanceSelectors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeZoneVmExtensionPolicy_Timeouts>,
    dynamic: ComputeZoneVmExtensionPolicyDynamic,
}

struct ComputeZoneVmExtensionPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeZoneVmExtensionPolicyData_>,
}

#[derive(Clone)]
pub struct ComputeZoneVmExtensionPolicy(std::rc::Rc<ComputeZoneVmExtensionPolicy_>);

impl ComputeZoneVmExtensionPolicy {
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\nPriority of this policy."]
    pub fn set_priority(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extension_policies`.\n"]
    pub fn set_extension_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_ExtensionPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().extension_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.extension_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_selectors`.\n"]
    pub fn set_instance_selectors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_InstanceSelectors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_selectors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeZoneVmExtensionPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_by_global` after provisioning.\nIndicates if this policy is managed by a global policy."]
    pub fn managed_by_global(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_by_global", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of this policy."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the policy."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone for this request."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_selectors` after provisioning.\n"]
    pub fn instance_selectors(&self) -> terrars::ListRef<ComputeZoneVmExtensionPolicy_InstanceSelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeZoneVmExtensionPolicy_TimeoutsRef {
        ComputeZoneVmExtensionPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeZoneVmExtensionPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeZoneVmExtensionPolicy { }

impl terrars::ToListMappable for ComputeZoneVmExtensionPolicy {
    type O = terrars::ListRef<ComputeZoneVmExtensionPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeZoneVmExtensionPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_zone_vm_extension_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeZoneVmExtensionPolicy {
    pub tf_id: std::string::String,
    #[doc = "Name of the resource. Provided by the client when the resource is created."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Name of the zone for this request."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildComputeZoneVmExtensionPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeZoneVmExtensionPolicy {
        let out = ComputeZoneVmExtensionPolicy(std::rc::Rc::new(ComputeZoneVmExtensionPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeZoneVmExtensionPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                zone: self.zone,
                extension_policies: core::default::Default::default(),
                instance_selectors: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeZoneVmExtensionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeZoneVmExtensionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeZoneVmExtensionPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_by_global` after provisioning.\nIndicates if this policy is managed by a global policy."]
    pub fn managed_by_global(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_by_global", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of this policy."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the policy."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nName of the zone for this request."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_selectors` after provisioning.\n"]
    pub fn instance_selectors(&self) -> terrars::ListRef<ComputeZoneVmExtensionPolicy_InstanceSelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeZoneVmExtensionPolicy_TimeoutsRef {
        ComputeZoneVmExtensionPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeZoneVmExtensionPolicyDynamic {
    extension_policies: std::option::Option<terrars::DynamicBlock<ComputeZoneVmExtensionPolicy_ExtensionPolicies>>,
    instance_selectors: std::option::Option<terrars::DynamicBlock<ComputeZoneVmExtensionPolicy_InstanceSelectors>>,
}

#[derive(Serialize)]
pub struct ComputeZoneVmExtensionPolicy_ExtensionPolicies {
    extension_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinned_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeZoneVmExtensionPolicy_ExtensionPolicies {
    #[doc = "Set the field `pinned_version`.\nThe specific version of the extension to install."]
    pub fn set_pinned_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pinned_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_config`.\nString-based configuration data for the extension."]
    pub fn set_string_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeZoneVmExtensionPolicy_ExtensionPolicies {
    type O = terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_ExtensionPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeZoneVmExtensionPolicy_ExtensionPolicies {
    #[doc = ""]
    pub extension_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeZoneVmExtensionPolicy_ExtensionPolicies {
    pub fn build(self) -> ComputeZoneVmExtensionPolicy_ExtensionPolicies {
        ComputeZoneVmExtensionPolicy_ExtensionPolicies {
            extension_name: self.extension_name,
            pinned_version: core::default::Default::default(),
            string_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeZoneVmExtensionPolicy_ExtensionPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeZoneVmExtensionPolicy_ExtensionPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeZoneVmExtensionPolicy_ExtensionPoliciesRef {
        ComputeZoneVmExtensionPolicy_ExtensionPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeZoneVmExtensionPolicy_ExtensionPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extension_name` after provisioning.\n"]
    pub fn extension_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extension_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pinned_version` after provisioning.\nThe specific version of the extension to install."]
    pub fn pinned_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pinned_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `string_config` after provisioning.\nString-based configuration data for the extension."]
    pub fn string_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {
    #[doc = "Set the field `inclusion_labels`.\nA map of key-value pairs representing VM labels."]
    pub fn set_inclusion_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inclusion_labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {
    type O = terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {}

impl BuildComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {
    pub fn build(self) -> ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {
        ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector {
            inclusion_labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
        ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inclusion_labels` after provisioning.\nA map of key-value pairs representing VM labels."]
    pub fn inclusion_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.inclusion_labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeZoneVmExtensionPolicy_InstanceSelectorsDynamic {
    label_selector: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector>>,
}

#[derive(Serialize)]
pub struct ComputeZoneVmExtensionPolicy_InstanceSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_selector: std::option::Option<std::vec::Vec<ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector>>,
    dynamic: ComputeZoneVmExtensionPolicy_InstanceSelectorsDynamic,
}

impl ComputeZoneVmExtensionPolicy_InstanceSelectors {
    #[doc = "Set the field `label_selector`.\n"]
    pub fn set_label_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeZoneVmExtensionPolicy_InstanceSelectors {
    type O = terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_InstanceSelectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeZoneVmExtensionPolicy_InstanceSelectors {}

impl BuildComputeZoneVmExtensionPolicy_InstanceSelectors {
    pub fn build(self) -> ComputeZoneVmExtensionPolicy_InstanceSelectors {
        ComputeZoneVmExtensionPolicy_InstanceSelectors {
            label_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeZoneVmExtensionPolicy_InstanceSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeZoneVmExtensionPolicy_InstanceSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeZoneVmExtensionPolicy_InstanceSelectorsRef {
        ComputeZoneVmExtensionPolicy_InstanceSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeZoneVmExtensionPolicy_InstanceSelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_selector` after provisioning.\n"]
    pub fn label_selector(&self) -> terrars::ListRef<ComputeZoneVmExtensionPolicy_InstanceSelectors_LabelSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeZoneVmExtensionPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeZoneVmExtensionPolicy_Timeouts {
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

impl terrars::ToListMappable for ComputeZoneVmExtensionPolicy_Timeouts {
    type O = terrars::BlockAssignable<ComputeZoneVmExtensionPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeZoneVmExtensionPolicy_Timeouts {}

impl BuildComputeZoneVmExtensionPolicy_Timeouts {
    pub fn build(self) -> ComputeZoneVmExtensionPolicy_Timeouts {
        ComputeZoneVmExtensionPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeZoneVmExtensionPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeZoneVmExtensionPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeZoneVmExtensionPolicy_TimeoutsRef {
        ComputeZoneVmExtensionPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeZoneVmExtensionPolicy_TimeoutsRef {
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
