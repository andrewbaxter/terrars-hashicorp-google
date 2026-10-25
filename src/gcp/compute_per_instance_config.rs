#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputePerInstanceConfigData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_group_manager: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimal_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_disruptive_allowed_action: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove_instance_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remove_instance_state_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserved_state: std::option::Option<std::vec::Vec<ComputePerInstanceConfig_PreservedState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputePerInstanceConfig_Timeouts>,
    dynamic: ComputePerInstanceConfigDynamic,
}

struct ComputePerInstanceConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputePerInstanceConfigData_>,
}

#[derive(Clone)]
pub struct ComputePerInstanceConfig(std::rc::Rc<ComputePerInstanceConfig_>);

impl ComputePerInstanceConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimal_action`.\nThe minimal action to perform on the instance during an update.\nDefault is 'NONE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn set_minimal_action(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().minimal_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `most_disruptive_allowed_action`.\nThe most disruptive action to perform on the instance during an update.\nDefault is 'REPLACE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn set_most_disruptive_allowed_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().most_disruptive_allowed_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remove_instance_on_destroy`.\nWhen true, deleting this config will immediately remove the underlying instance.\nWhen false, deleting this config will use the behavior as determined by remove_instance_on_destroy."]
    pub fn set_remove_instance_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_instance_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `remove_instance_state_on_destroy`.\nWhen true, deleting this config will immediately remove any specified state from the underlying instance.\nWhen false, deleting this config will *not* immediately remove any state from the underlying instance.\nState will be removed on the next instance recreation or update."]
    pub fn set_remove_instance_state_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_instance_state_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nZone where the containing instance group manager is located"]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserved_state`.\n"]
    pub fn set_preserved_state(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preserved_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preserved_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputePerInstanceConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe instance group manager this instance config is part of."]
    pub fn instance_group_manager(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `minimal_action` after provisioning.\nThe minimal action to perform on the instance during an update.\nDefault is 'NONE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn minimal_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimal_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\nThe most disruptive action to perform on the instance during an update.\nDefault is 'REPLACE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn most_disruptive_allowed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_disruptive_allowed_action", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name for this per-instance config and its corresponding instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_instance_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove the underlying instance.\nWhen false, deleting this config will use the behavior as determined by remove_instance_on_destroy."]
    pub fn remove_instance_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_instance_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_instance_state_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove any specified state from the underlying instance.\nWhen false, deleting this config will *not* immediately remove any state from the underlying instance.\nState will be removed on the next instance recreation or update."]
    pub fn remove_instance_state_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.remove_instance_state_on_destroy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nZone where the containing instance group manager is located"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserved_state` after provisioning.\n"]
    pub fn preserved_state(&self) -> terrars::ListRef<ComputePerInstanceConfig_PreservedStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preserved_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePerInstanceConfig_TimeoutsRef {
        ComputePerInstanceConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputePerInstanceConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputePerInstanceConfig { }

impl terrars::ToListMappable for ComputePerInstanceConfig {
    type O = terrars::ListRef<ComputePerInstanceConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputePerInstanceConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_per_instance_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputePerInstanceConfig {
    pub tf_id: std::string::String,
    #[doc = "The instance group manager this instance config is part of."]
    pub instance_group_manager: terrars::PrimField<std::string::String>,
    #[doc = "The name for this per-instance config and its corresponding instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputePerInstanceConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputePerInstanceConfig {
        let out = ComputePerInstanceConfig(std::rc::Rc::new(ComputePerInstanceConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputePerInstanceConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_group_manager: self.instance_group_manager,
                minimal_action: core::default::Default::default(),
                most_disruptive_allowed_action: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                remove_instance_on_destroy: core::default::Default::default(),
                remove_instance_state_on_destroy: core::default::Default::default(),
                zone: core::default::Default::default(),
                preserved_state: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputePerInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputePerInstanceConfigRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe instance group manager this instance config is part of."]
    pub fn instance_group_manager(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `minimal_action` after provisioning.\nThe minimal action to perform on the instance during an update.\nDefault is 'NONE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn minimal_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimal_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\nThe most disruptive action to perform on the instance during an update.\nDefault is 'REPLACE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn most_disruptive_allowed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_disruptive_allowed_action", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name for this per-instance config and its corresponding instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_instance_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove the underlying instance.\nWhen false, deleting this config will use the behavior as determined by remove_instance_on_destroy."]
    pub fn remove_instance_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remove_instance_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `remove_instance_state_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove any specified state from the underlying instance.\nWhen false, deleting this config will *not* immediately remove any state from the underlying instance.\nState will be removed on the next instance recreation or update."]
    pub fn remove_instance_state_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.remove_instance_state_on_destroy", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nZone where the containing instance group manager is located"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserved_state` after provisioning.\n"]
    pub fn preserved_state(&self) -> terrars::ListRef<ComputePerInstanceConfig_PreservedStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preserved_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePerInstanceConfig_TimeoutsRef {
        ComputePerInstanceConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfigDynamic {
    preserved_state: std::option::Option<terrars::DynamicBlock<ComputePerInstanceConfig_PreservedState>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfig_PreservedState_Disk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    device_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
}

impl ComputePerInstanceConfig_PreservedState_Disk {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted.\nThe available options are 'NEVER' and 'ON_PERMANENT_INSTANCE_DELETION'.\n'NEVER' - detach the disk when the VM is deleted, but do not delete the disk.\n'ON_PERMANENT_INSTANCE_DELETION' will delete the stateful disk when the VM is permanently\ndeleted from the instance group. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nThe mode of the disk. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputePerInstanceConfig_PreservedState_Disk {
    type O = terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_Disk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfig_PreservedState_Disk {
    #[doc =
        "A unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance."]
    pub device_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The URI of an existing persistent disk to attach under the specified device-name in the format\n'projects/project-id/zones/zone/disks/disk-name'."]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildComputePerInstanceConfig_PreservedState_Disk {
    pub fn build(self) -> ComputePerInstanceConfig_PreservedState_Disk {
        ComputePerInstanceConfig_PreservedState_Disk {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
            mode: core::default::Default::default(),
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct ComputePerInstanceConfig_PreservedState_DiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfig_PreservedState_DiskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePerInstanceConfig_PreservedState_DiskRef {
        ComputePerInstanceConfig_PreservedState_DiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfig_PreservedState_DiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted.\nThe available options are 'NEVER' and 'ON_PERMANENT_INSTANCE_DELETION'.\n'NEVER' - detach the disk when the VM is deleted, but do not delete the disk.\n'ON_PERMANENT_INSTANCE_DELETION' will delete the stateful disk when the VM is permanently\ndeleted from the instance group. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_name` after provisioning.\nA unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance."]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode of the disk. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe URI of an existing persistent disk to attach under the specified device-name in the format\n'projects/project-id/zones/zone/disks/disk-name'."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress {
    #[doc = "Set the field `address`.\nThe URL of the reservation for this IP address."]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress {
    type O = terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress {}

impl BuildComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress {
    pub fn build(self) -> ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress {
        ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress { address: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef {
        ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address` after provisioning.\nThe URL of the reservation for this IP address."]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfig_PreservedState_ExternalIpDynamic {
    ip_address: std
    ::option
    ::Option<terrars::DynamicBlock<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfig_PreservedState_ExternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete: std::option::Option<terrars::PrimField<std::string::String>>,
    interface_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<std::vec::Vec<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>>,
    dynamic: ComputePerInstanceConfig_PreservedState_ExternalIpDynamic,
}

impl ComputePerInstanceConfig_PreservedState_ExternalIp {
    #[doc =
        "Set the field `auto_delete`.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn set_auto_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_address = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputePerInstanceConfig_PreservedState_ExternalIp {
    type O = terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_ExternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfig_PreservedState_ExternalIp {
    #[doc = ""]
    pub interface_name: terrars::PrimField<std::string::String>,
}

impl BuildComputePerInstanceConfig_PreservedState_ExternalIp {
    pub fn build(self) -> ComputePerInstanceConfig_PreservedState_ExternalIp {
        ComputePerInstanceConfig_PreservedState_ExternalIp {
            auto_delete: core::default::Default::default(),
            interface_name: self.interface_name,
            ip_address: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePerInstanceConfig_PreservedState_ExternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfig_PreservedState_ExternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePerInstanceConfig_PreservedState_ExternalIpRef {
        ComputePerInstanceConfig_PreservedState_ExternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfig_PreservedState_ExternalIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_delete` after provisioning.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn auto_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfig_PreservedState_InternalIpDynamic {
    ip_address: std
    ::option
    ::Option<terrars::DynamicBlock<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfig_PreservedState_InternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete: std::option::Option<terrars::PrimField<std::string::String>>,
    interface_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<std::vec::Vec<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>>,
    dynamic: ComputePerInstanceConfig_PreservedState_InternalIpDynamic,
}

impl ComputePerInstanceConfig_PreservedState_InternalIp {
    #[doc =
        "Set the field `auto_delete`.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn set_auto_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddress>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_address = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputePerInstanceConfig_PreservedState_InternalIp {
    type O = terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_InternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfig_PreservedState_InternalIp {
    #[doc = ""]
    pub interface_name: terrars::PrimField<std::string::String>,
}

impl BuildComputePerInstanceConfig_PreservedState_InternalIp {
    pub fn build(self) -> ComputePerInstanceConfig_PreservedState_InternalIp {
        ComputePerInstanceConfig_PreservedState_InternalIp {
            auto_delete: core::default::Default::default(),
            interface_name: self.interface_name,
            ip_address: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePerInstanceConfig_PreservedState_InternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfig_PreservedState_InternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputePerInstanceConfig_PreservedState_InternalIpRef {
        ComputePerInstanceConfig_PreservedState_InternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfig_PreservedState_InternalIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_delete` after provisioning.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn auto_delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::ListRef<ComputePerInstanceConfig_PreservedState_ExternalIp_IpAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfig_PreservedStateDynamic {
    disk: std::option::Option<terrars::DynamicBlock<ComputePerInstanceConfig_PreservedState_Disk>>,
    external_ip: std::option::Option<terrars::DynamicBlock<ComputePerInstanceConfig_PreservedState_ExternalIp>>,
    internal_ip: std::option::Option<terrars::DynamicBlock<ComputePerInstanceConfig_PreservedState_InternalIp>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfig_PreservedState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<std::vec::Vec<ComputePerInstanceConfig_PreservedState_Disk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ip: std::option::Option<std::vec::Vec<ComputePerInstanceConfig_PreservedState_ExternalIp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<std::vec::Vec<ComputePerInstanceConfig_PreservedState_InternalIp>>,
    dynamic: ComputePerInstanceConfig_PreservedStateDynamic,
}

impl ComputePerInstanceConfig_PreservedState {
    #[doc =
        "Set the field `metadata`.\nPreserved metadata defined for this instance. This is a list of key->value pairs."]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_Disk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_ip`.\n"]
    pub fn set_external_ip(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_ExternalIp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_ip = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState_InternalIp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internal_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_ip = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputePerInstanceConfig_PreservedState {
    type O = terrars::BlockAssignable<ComputePerInstanceConfig_PreservedState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfig_PreservedState {}

impl BuildComputePerInstanceConfig_PreservedState {
    pub fn build(self) -> ComputePerInstanceConfig_PreservedState {
        ComputePerInstanceConfig_PreservedState {
            metadata: core::default::Default::default(),
            disk: core::default::Default::default(),
            external_ip: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePerInstanceConfig_PreservedStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfig_PreservedStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePerInstanceConfig_PreservedStateRef {
        ComputePerInstanceConfig_PreservedStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfig_PreservedStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nPreserved metadata defined for this instance. This is a list of key->value pairs."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputePerInstanceConfig_Timeouts {
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

impl terrars::ToListMappable for ComputePerInstanceConfig_Timeouts {
    type O = terrars::BlockAssignable<ComputePerInstanceConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfig_Timeouts {}

impl BuildComputePerInstanceConfig_Timeouts {
    pub fn build(self) -> ComputePerInstanceConfig_Timeouts {
        ComputePerInstanceConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputePerInstanceConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputePerInstanceConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputePerInstanceConfig_TimeoutsRef {
        ComputePerInstanceConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfig_TimeoutsRef {
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
