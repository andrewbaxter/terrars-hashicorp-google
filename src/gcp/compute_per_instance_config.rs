use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputePerInstanceConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_group_manager: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimal_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_disruptive_allowed_action: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_instance_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_instance_state_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserved_state: Option<Vec<ComputePerInstanceConfigPreservedStateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputePerInstanceConfigTimeoutsEl>,
    dynamic: ComputePerInstanceConfigDynamic,
}

struct ComputePerInstanceConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputePerInstanceConfigData>,
}

#[derive(Clone)]
pub struct ComputePerInstanceConfig(Rc<ComputePerInstanceConfig_>);

impl ComputePerInstanceConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `minimal_action`.\nThe minimal action to perform on the instance during an update.\nDefault is 'NONE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn set_minimal_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().minimal_action = Some(v.into());
        self
    }

    #[doc= "Set the field `most_disruptive_allowed_action`.\nThe most disruptive action to perform on the instance during an update.\nDefault is 'REPLACE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn set_most_disruptive_allowed_action(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().most_disruptive_allowed_action = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `remove_instance_on_destroy`.\nWhen true, deleting this config will immediately remove the underlying instance.\nWhen false, deleting this config will use the behavior as determined by remove_instance_on_destroy."]
    pub fn set_remove_instance_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_instance_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `remove_instance_state_on_destroy`.\nWhen true, deleting this config will immediately remove any specified state from the underlying instance.\nWhen false, deleting this config will *not* immediately remove any state from the underlying instance.\nState will be removed on the next instance recreation or update."]
    pub fn set_remove_instance_state_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remove_instance_state_on_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nZone where the containing instance group manager is located"]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }

    #[doc= "Set the field `preserved_state`.\n"]
    pub fn set_preserved_state(self, v: impl Into<BlockAssignable<ComputePerInstanceConfigPreservedStateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preserved_state = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preserved_state = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputePerInstanceConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe instance group manager this instance config is part of."]
    pub fn instance_group_manager(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimal_action` after provisioning.\nThe minimal action to perform on the instance during an update.\nDefault is 'NONE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn minimal_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimal_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\nThe most disruptive action to perform on the instance during an update.\nDefault is 'REPLACE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn most_disruptive_allowed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_disruptive_allowed_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name for this per-instance config and its corresponding instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_instance_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove the underlying instance.\nWhen false, deleting this config will use the behavior as determined by remove_instance_on_destroy."]
    pub fn remove_instance_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_instance_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_instance_state_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove any specified state from the underlying instance.\nWhen false, deleting this config will *not* immediately remove any state from the underlying instance.\nState will be removed on the next instance recreation or update."]
    pub fn remove_instance_state_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_instance_state_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nZone where the containing instance group manager is located"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preserved_state` after provisioning.\n"]
    pub fn preserved_state(&self) -> ListRef<ComputePerInstanceConfigPreservedStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.preserved_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePerInstanceConfigTimeoutsElRef {
        ComputePerInstanceConfigTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputePerInstanceConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputePerInstanceConfig { }

impl ToListMappable for ComputePerInstanceConfig {
    type O = ListRef<ComputePerInstanceConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputePerInstanceConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_per_instance_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputePerInstanceConfig {
    pub tf_id: String,
    #[doc= "The instance group manager this instance config is part of."]
    pub instance_group_manager: PrimField<String>,
    #[doc= "The name for this per-instance config and its corresponding instance."]
    pub name: PrimField<String>,
}

impl BuildComputePerInstanceConfig {
    pub fn build(self, stack: &mut Stack) -> ComputePerInstanceConfig {
        let out = ComputePerInstanceConfig(Rc::new(ComputePerInstanceConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputePerInstanceConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputePerInstanceConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputePerInstanceConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe instance group manager this instance config is part of."]
    pub fn instance_group_manager(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `minimal_action` after provisioning.\nThe minimal action to perform on the instance during an update.\nDefault is 'NONE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn minimal_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimal_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\nThe most disruptive action to perform on the instance during an update.\nDefault is 'REPLACE'. Possible values are:\n* REPLACE\n* RESTART\n* REFRESH\n* NONE"]
    pub fn most_disruptive_allowed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_disruptive_allowed_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name for this per-instance config and its corresponding instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_instance_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove the underlying instance.\nWhen false, deleting this config will use the behavior as determined by remove_instance_on_destroy."]
    pub fn remove_instance_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_instance_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remove_instance_state_on_destroy` after provisioning.\nWhen true, deleting this config will immediately remove any specified state from the underlying instance.\nWhen false, deleting this config will *not* immediately remove any state from the underlying instance.\nState will be removed on the next instance recreation or update."]
    pub fn remove_instance_state_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remove_instance_state_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nZone where the containing instance group manager is located"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preserved_state` after provisioning.\n"]
    pub fn preserved_state(&self) -> ListRef<ComputePerInstanceConfigPreservedStateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.preserved_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputePerInstanceConfigTimeoutsElRef {
        ComputePerInstanceConfigTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigPreservedStateElDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_rule: Option<PrimField<String>>,
    device_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    source: PrimField<String>,
}

impl ComputePerInstanceConfigPreservedStateElDiskEl {
    #[doc= "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted.\nThe available options are 'NEVER' and 'ON_PERMANENT_INSTANCE_DELETION'.\n'NEVER' - detach the disk when the VM is deleted, but do not delete the disk.\n'ON_PERMANENT_INSTANCE_DELETION' will delete the stateful disk when the VM is permanently\ndeleted from the instance group. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn set_delete_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_rule = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\nThe mode of the disk. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigPreservedStateElDiskEl {
    type O = BlockAssignable<ComputePerInstanceConfigPreservedStateElDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigPreservedStateElDiskEl {
    #[doc= "A unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance."]
    pub device_name: PrimField<String>,
    #[doc= "The URI of an existing persistent disk to attach under the specified device-name in the format\n'projects/project-id/zones/zone/disks/disk-name'."]
    pub source: PrimField<String>,
}

impl BuildComputePerInstanceConfigPreservedStateElDiskEl {
    pub fn build(self) -> ComputePerInstanceConfigPreservedStateElDiskEl {
        ComputePerInstanceConfigPreservedStateElDiskEl {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
            mode: core::default::Default::default(),
            source: self.source,
        }
    }
}

pub struct ComputePerInstanceConfigPreservedStateElDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigPreservedStateElDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigPreservedStateElDiskElRef {
        ComputePerInstanceConfigPreservedStateElDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigPreservedStateElDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted.\nThe available options are 'NEVER' and 'ON_PERMANENT_INSTANCE_DELETION'.\n'NEVER' - detach the disk when the VM is deleted, but do not delete the disk.\n'ON_PERMANENT_INSTANCE_DELETION' will delete the stateful disk when the VM is permanently\ndeleted from the instance group. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn delete_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\nA unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance."]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe mode of the disk. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nThe URI of an existing persistent disk to attach under the specified device-name in the format\n'projects/project-id/zones/zone/disks/disk-name'."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
}

impl ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {
    #[doc= "Set the field `address`.\nThe URL of the reservation for this IP address."]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {
    type O = BlockAssignable<ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {}

impl BuildComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {
    pub fn build(self) -> ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {
        ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl {
            address: core::default::Default::default(),
        }
    }
}

pub struct ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressElRef {
        ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nThe URL of the reservation for this IP address."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfigPreservedStateElExternalIpElDynamic {
    ip_address: Option<DynamicBlock<ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigPreservedStateElExternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<String>>,
    interface_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<Vec<ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl>>,
    dynamic: ComputePerInstanceConfigPreservedStateElExternalIpElDynamic,
}

impl ComputePerInstanceConfigPreservedStateElExternalIpEl {
    #[doc= "Set the field `auto_delete`.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address`.\n"]
    pub fn set_ip_address(
        mut self,
        v: impl Into<BlockAssignable<ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_address = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_address = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigPreservedStateElExternalIpEl {
    type O = BlockAssignable<ComputePerInstanceConfigPreservedStateElExternalIpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigPreservedStateElExternalIpEl {
    #[doc= ""]
    pub interface_name: PrimField<String>,
}

impl BuildComputePerInstanceConfigPreservedStateElExternalIpEl {
    pub fn build(self) -> ComputePerInstanceConfigPreservedStateElExternalIpEl {
        ComputePerInstanceConfigPreservedStateElExternalIpEl {
            auto_delete: core::default::Default::default(),
            interface_name: self.interface_name,
            ip_address: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputePerInstanceConfigPreservedStateElExternalIpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigPreservedStateElExternalIpElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigPreservedStateElExternalIpElRef {
        ComputePerInstanceConfigPreservedStateElExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigPreservedStateElExternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_delete` after provisioning.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn auto_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> ListRef<ComputePerInstanceConfigPreservedStateElExternalIpElIpAddressElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
}

impl ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {
    #[doc= "Set the field `address`.\nThe URL of the reservation for this IP address."]
    pub fn set_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address = Some(v.into());
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {
    type O = BlockAssignable<ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {}

impl BuildComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {
    pub fn build(self) -> ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {
        ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl {
            address: core::default::Default::default(),
        }
    }
}

pub struct ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressElRef {
        ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address` after provisioning.\nThe URL of the reservation for this IP address."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfigPreservedStateElInternalIpElDynamic {
    ip_address: Option<DynamicBlock<ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigPreservedStateElInternalIpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<String>>,
    interface_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<Vec<ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl>>,
    dynamic: ComputePerInstanceConfigPreservedStateElInternalIpElDynamic,
}

impl ComputePerInstanceConfigPreservedStateElInternalIpEl {
    #[doc= "Set the field `auto_delete`.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_address`.\n"]
    pub fn set_ip_address(
        mut self,
        v: impl Into<BlockAssignable<ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ip_address = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_address = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigPreservedStateElInternalIpEl {
    type O = BlockAssignable<ComputePerInstanceConfigPreservedStateElInternalIpEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigPreservedStateElInternalIpEl {
    #[doc= ""]
    pub interface_name: PrimField<String>,
}

impl BuildComputePerInstanceConfigPreservedStateElInternalIpEl {
    pub fn build(self) -> ComputePerInstanceConfigPreservedStateElInternalIpEl {
        ComputePerInstanceConfigPreservedStateElInternalIpEl {
            auto_delete: core::default::Default::default(),
            interface_name: self.interface_name,
            ip_address: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputePerInstanceConfigPreservedStateElInternalIpElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigPreservedStateElInternalIpElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigPreservedStateElInternalIpElRef {
        ComputePerInstanceConfigPreservedStateElInternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigPreservedStateElInternalIpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_delete` after provisioning.\nThese stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted. Default value: \"NEVER\" Possible values: [\"NEVER\", \"ON_PERMANENT_INSTANCE_DELETION\"]"]
    pub fn auto_delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> ListRef<ComputePerInstanceConfigPreservedStateElInternalIpElIpAddressElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfigPreservedStateElDynamic {
    disk: Option<DynamicBlock<ComputePerInstanceConfigPreservedStateElDiskEl>>,
    external_ip: Option<DynamicBlock<ComputePerInstanceConfigPreservedStateElExternalIpEl>>,
    internal_ip: Option<DynamicBlock<ComputePerInstanceConfigPreservedStateElInternalIpEl>>,
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigPreservedStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<Vec<ComputePerInstanceConfigPreservedStateElDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ip: Option<Vec<ComputePerInstanceConfigPreservedStateElExternalIpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip: Option<Vec<ComputePerInstanceConfigPreservedStateElInternalIpEl>>,
    dynamic: ComputePerInstanceConfigPreservedStateElDynamic,
}

impl ComputePerInstanceConfigPreservedStateEl {
    #[doc= "Set the field `metadata`.\nPreserved metadata defined for this instance. This is a list of key->value pairs."]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<BlockAssignable<ComputePerInstanceConfigPreservedStateElDiskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_ip`.\n"]
    pub fn set_external_ip(
        mut self,
        v: impl Into<BlockAssignable<ComputePerInstanceConfigPreservedStateElExternalIpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_ip = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_ip = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `internal_ip`.\n"]
    pub fn set_internal_ip(
        mut self,
        v: impl Into<BlockAssignable<ComputePerInstanceConfigPreservedStateElInternalIpEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internal_ip = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_ip = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigPreservedStateEl {
    type O = BlockAssignable<ComputePerInstanceConfigPreservedStateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigPreservedStateEl {}

impl BuildComputePerInstanceConfigPreservedStateEl {
    pub fn build(self) -> ComputePerInstanceConfigPreservedStateEl {
        ComputePerInstanceConfigPreservedStateEl {
            metadata: core::default::Default::default(),
            disk: core::default::Default::default(),
            external_ip: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputePerInstanceConfigPreservedStateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigPreservedStateElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigPreservedStateElRef {
        ComputePerInstanceConfigPreservedStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigPreservedStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nPreserved metadata defined for this instance. This is a list of key->value pairs."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputePerInstanceConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputePerInstanceConfigTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ComputePerInstanceConfigTimeoutsEl {
    type O = BlockAssignable<ComputePerInstanceConfigTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputePerInstanceConfigTimeoutsEl {}

impl BuildComputePerInstanceConfigTimeoutsEl {
    pub fn build(self) -> ComputePerInstanceConfigTimeoutsEl {
        ComputePerInstanceConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputePerInstanceConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputePerInstanceConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputePerInstanceConfigTimeoutsElRef {
        ComputePerInstanceConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputePerInstanceConfigTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputePerInstanceConfigDynamic {
    preserved_state: Option<DynamicBlock<ComputePerInstanceConfigPreservedStateEl>>,
}
