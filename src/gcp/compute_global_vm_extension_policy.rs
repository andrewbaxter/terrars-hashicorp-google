#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeGlobalVmExtensionPolicyData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extension_policies: std::option::Option<std::vec::Vec<ComputeGlobalVmExtensionPolicy_ExtensionPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selectors: std::option::Option<std::vec::Vec<ComputeGlobalVmExtensionPolicy_InstanceSelectors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_operation: std::option::Option<std::vec::Vec<ComputeGlobalVmExtensionPolicy_RolloutOperation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeGlobalVmExtensionPolicy_Timeouts>,
    dynamic: ComputeGlobalVmExtensionPolicyDynamic,
}

struct ComputeGlobalVmExtensionPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeGlobalVmExtensionPolicyData_>,
}

#[derive(Clone)]
pub struct ComputeGlobalVmExtensionPolicy(std::rc::Rc<ComputeGlobalVmExtensionPolicy_>);

impl ComputeGlobalVmExtensionPolicy {
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

    #[doc = "Set the field `priority`.\nUsed to resolve conflicts when multiple policies are active. Defaults to 0."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_ExtensionPolicies>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_InstanceSelectors>>,
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

    #[doc = "Set the field `rollout_operation`.\n"]
    pub fn set_rollout_operation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout_operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout_operation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeGlobalVmExtensionPolicy_Timeouts>) -> Self {
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
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long and match the regular expression '^[a-z]([-a-z0-9]{0,61}[a-z0-9])?$' to comply with RFC1035."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nUsed to resolve conflicts when multiple policies are active. Defaults to 0."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scoped_resource_status` after provisioning.\nThe scoped resource status."]
    pub fn scoped_resource_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scoped_resource_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined fully-qualified URL for this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_timestamp` after provisioning.\nUpdate timestamp in RFC3339 text format."]
    pub fn update_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_selectors` after provisioning.\n"]
    pub fn instance_selectors(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout_operation` after provisioning.\n"]
    pub fn rollout_operation(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_RolloutOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeGlobalVmExtensionPolicy_TimeoutsRef {
        ComputeGlobalVmExtensionPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeGlobalVmExtensionPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeGlobalVmExtensionPolicy { }

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy {
    type O = terrars::ListRef<ComputeGlobalVmExtensionPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeGlobalVmExtensionPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_global_vm_extension_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long and match the regular expression '^[a-z]([-a-z0-9]{0,61}[a-z0-9])?$' to comply with RFC1035."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeGlobalVmExtensionPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeGlobalVmExtensionPolicy {
        let out = ComputeGlobalVmExtensionPolicy(std::rc::Rc::new(ComputeGlobalVmExtensionPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeGlobalVmExtensionPolicyData_ {
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
                extension_policies: core::default::Default::default(),
                instance_selectors: core::default::Default::default(),
                rollout_operation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeGlobalVmExtensionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeGlobalVmExtensionPolicyRef {
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
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long and match the regular expression '^[a-z]([-a-z0-9]{0,61}[a-z0-9])?$' to comply with RFC1035."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nUsed to resolve conflicts when multiple policies are active. Defaults to 0."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scoped_resource_status` after provisioning.\nThe scoped resource status."]
    pub fn scoped_resource_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scoped_resource_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined fully-qualified URL for this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_timestamp` after provisioning.\nUpdate timestamp in RFC3339 text format."]
    pub fn update_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_selectors` after provisioning.\n"]
    pub fn instance_selectors(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_selectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout_operation` after provisioning.\n"]
    pub fn rollout_operation(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_RolloutOperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeGlobalVmExtensionPolicy_TimeoutsRef {
        ComputeGlobalVmExtensionPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeGlobalVmExtensionPolicyDynamic {
    extension_policies: std::option::Option<terrars::DynamicBlock<ComputeGlobalVmExtensionPolicy_ExtensionPolicies>>,
    instance_selectors: std::option::Option<terrars::DynamicBlock<ComputeGlobalVmExtensionPolicy_InstanceSelectors>>,
    rollout_operation: std::option::Option<terrars::DynamicBlock<ComputeGlobalVmExtensionPolicy_RolloutOperation>>,
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_ExtensionPolicies {
    extension_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinned_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeGlobalVmExtensionPolicy_ExtensionPolicies {
    #[doc = "Set the field `pinned_version`.\nThe version pinning for the extension."]
    pub fn set_pinned_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pinned_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_config`.\nString configuration payload."]
    pub fn set_string_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_ExtensionPolicies {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_ExtensionPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_ExtensionPolicies {
    #[doc = ""]
    pub extension_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeGlobalVmExtensionPolicy_ExtensionPolicies {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_ExtensionPolicies {
        ComputeGlobalVmExtensionPolicy_ExtensionPolicies {
            extension_name: self.extension_name,
            pinned_version: core::default::Default::default(),
            string_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_ExtensionPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_ExtensionPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_ExtensionPoliciesRef {
        ComputeGlobalVmExtensionPolicy_ExtensionPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_ExtensionPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extension_name` after provisioning.\n"]
    pub fn extension_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extension_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pinned_version` after provisioning.\nThe version pinning for the extension."]
    pub fn pinned_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pinned_version", self.base))
    }

    #[doc = "Get a reference to the value of field `string_config` after provisioning.\nString configuration payload."]
    pub fn string_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {
    #[doc = "Set the field `inclusion_labels`.\nLabels as key value pairs."]
    pub fn set_inclusion_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inclusion_labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {}

impl BuildComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {
        ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector {
            inclusion_labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
        ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inclusion_labels` after provisioning.\nLabels as key value pairs."]
    pub fn inclusion_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.inclusion_labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeGlobalVmExtensionPolicy_InstanceSelectorsDynamic {
    label_selector: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector>>,
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_InstanceSelectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_selector: std::option::Option<std::vec::Vec<ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector>>,
    dynamic: ComputeGlobalVmExtensionPolicy_InstanceSelectorsDynamic,
}

impl ComputeGlobalVmExtensionPolicy_InstanceSelectors {
    #[doc = "Set the field `label_selector`.\n"]
    pub fn set_label_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelector>>,
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

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_InstanceSelectors {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_InstanceSelectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_InstanceSelectors {}

impl BuildComputeGlobalVmExtensionPolicy_InstanceSelectors {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_InstanceSelectors {
        ComputeGlobalVmExtensionPolicy_InstanceSelectors {
            label_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef {
        ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_InstanceSelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_selector` after provisioning.\n"]
    pub fn label_selector(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_InstanceSelectors_LabelSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {
    #[doc = "Set the field `location_name`.\n"]
    pub fn set_location_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {
    type O =
        terrars
        ::BlockAssignable<
            ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {}

impl BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {
    pub fn build(
        self,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus {
            location_name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location_name` after provisioning.\n"]
    pub fn location_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_rollout_status: std
    ::option
    ::Option<
        terrars
        ::SetField<
            ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {
    #[doc = "Set the field `location_rollout_status`.\n"]
    pub fn set_location_rollout_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus,
                        >,
                    >,
    ) -> Self {
        self.location_rollout_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout_plan`.\n"]
    pub fn set_rollout_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {}

impl BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts {
            location_rollout_status: core::default::Default::default(),
            rollout: core::default::Default::default(),
            rollout_plan: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRolloutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRolloutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRolloutsRef {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRolloutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRolloutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location_rollout_status` after provisioning.\n"]
    pub fn location_rollout_status(
        &self,
    ) -> terrars
    ::SetRef<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.location_rollout_status", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout_plan` after provisioning.\n"]
    pub fn rollout_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_rollout_status: std
    ::option
    ::Option<
        terrars
        ::SetField<
            ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {
    #[doc = "Set the field `location_rollout_status`.\n"]
    pub fn set_location_rollout_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatus,
                        >,
                    >,
    ) -> Self {
        self.location_rollout_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout_plan`.\n"]
    pub fn set_rollout_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {}

impl BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout {
            location_rollout_status: core::default::Default::default(),
            rollout: core::default::Default::default(),
            rollout_plan: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRolloutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRolloutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRolloutRef {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRolloutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRolloutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location_rollout_status` after provisioning.\n"]
    pub fn location_rollout_status(
        &self,
    ) -> terrars
    ::SetRef<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts_LocationRolloutStatusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.location_rollout_status", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout_plan` after provisioning.\n"]
    pub fn rollout_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_rollouts: std
    ::option
    ::Option<terrars::ListField<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    previous_rollout: std
    ::option
    ::Option<terrars::ListField<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout>>,
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {
    #[doc = "Set the field `current_rollouts`.\n"]
    pub fn set_current_rollouts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRollouts>,
                    >,
    ) -> Self {
        self.current_rollouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `previous_rollout`.\n"]
    pub fn set_previous_rollout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRollout>,
                    >,
    ) -> Self {
        self.previous_rollout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {}

impl BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus {
            current_rollouts: core::default::Default::default(),
            previous_rollout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatusRef {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_rollouts` after provisioning.\n"]
    pub fn current_rollouts(
        &self,
    ) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_CurrentRolloutsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_rollouts", self.base))
    }

    #[doc = "Get a reference to the value of field `previous_rollout` after provisioning.\n"]
    pub fn previous_rollout(
        &self,
    ) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatus_PreviousRolloutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.previous_rollout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conflict_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_rollout_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {
    #[doc = "Set the field `conflict_behavior`.\nSpecifies the behavior of the rollout if a conflict is detected."]
    pub fn set_conflict_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.conflict_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the rollout plan."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `predefined_rollout_plan`.\nSpecifies the predefined rollout plan for the policy."]
    pub fn set_predefined_rollout_plan(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.predefined_rollout_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_uuid`.\nThe UUID that identifies a policy rollout retry attempt. It should only be set when retrying an existing rollout. Updating this field along with other policy fields (description, extension_policies, instance_selectors, priority) in the same plan will return an error."]
    pub fn set_retry_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retry_uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {}

impl BuildComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput {
            conflict_behavior: core::default::Default::default(),
            name: core::default::Default::default(),
            predefined_rollout_plan: core::default::Default::default(),
            retry_uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInputRef {
        ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `conflict_behavior` after provisioning.\nSpecifies the behavior of the rollout if a conflict is detected."]
    pub fn conflict_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conflict_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the rollout plan."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `predefined_rollout_plan` after provisioning.\nSpecifies the predefined rollout plan for the policy."]
    pub fn predefined_rollout_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_rollout_plan", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_uuid` after provisioning.\nThe UUID that identifies a policy rollout retry attempt. It should only be set when retrying an existing rollout. Updating this field along with other policy fields (description, extension_policies, instance_selectors, priority) in the same plan will return an error."]
    pub fn retry_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_uuid", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeGlobalVmExtensionPolicy_RolloutOperationDynamic {
    rollout_input: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput>>,
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_input: std::option::Option<std::vec::Vec<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput>>,
    dynamic: ComputeGlobalVmExtensionPolicy_RolloutOperationDynamic,
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperation {
    #[doc = "Set the field `rollout_input`.\n"]
    pub fn set_rollout_input(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollout_input = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout_input = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_RolloutOperation {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_RolloutOperation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_RolloutOperation {}

impl BuildComputeGlobalVmExtensionPolicy_RolloutOperation {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_RolloutOperation {
        ComputeGlobalVmExtensionPolicy_RolloutOperation {
            rollout_input: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_RolloutOperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_RolloutOperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeGlobalVmExtensionPolicy_RolloutOperationRef {
        ComputeGlobalVmExtensionPolicy_RolloutOperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_RolloutOperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rollout_status` after provisioning.\nRollout status."]
    pub fn rollout_status(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_status", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout_input` after provisioning.\n"]
    pub fn rollout_input(&self) -> terrars::ListRef<ComputeGlobalVmExtensionPolicy_RolloutOperation_RolloutInputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_input", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeGlobalVmExtensionPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeGlobalVmExtensionPolicy_Timeouts {
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

impl terrars::ToListMappable for ComputeGlobalVmExtensionPolicy_Timeouts {
    type O = terrars::BlockAssignable<ComputeGlobalVmExtensionPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeGlobalVmExtensionPolicy_Timeouts {}

impl BuildComputeGlobalVmExtensionPolicy_Timeouts {
    pub fn build(self) -> ComputeGlobalVmExtensionPolicy_Timeouts {
        ComputeGlobalVmExtensionPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeGlobalVmExtensionPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeGlobalVmExtensionPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeGlobalVmExtensionPolicy_TimeoutsRef {
        ComputeGlobalVmExtensionPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeGlobalVmExtensionPolicy_TimeoutsRef {
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
