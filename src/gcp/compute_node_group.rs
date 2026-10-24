use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNodeGroupData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    node_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policy: std::option::Option<std::vec::Vec<ComputeNodeGroupAutoscalingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<ComputeNodeGroupMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_settings: std::option::Option<std::vec::Vec<ComputeNodeGroupShareSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeNodeGroupTimeoutsEl>,
    dynamic: ComputeNodeGroupDynamic,
}

struct ComputeNodeGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeNodeGroupData>,
}

#[derive(Clone)]
pub struct ComputeNodeGroup(std::rc::Rc<ComputeNodeGroup_>);

impl ComputeNodeGroup {
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

    #[doc = "Set the field `description`.\nAn optional textual description of the resource."]
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
        "Set the field `initial_size`.\nThe initial number of nodes in the node group. One of 'initial_size' or 'autoscaling_policy' must be configured on resource creation."]
    pub fn set_initial_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().initial_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maintenance_policy`.\nSpecifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT."]
    pub fn set_maintenance_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().maintenance_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the resource."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nZone where this node group is located"]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeGroupAutoscalingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeGroupMaintenanceWindowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `share_settings`.\n"]
    pub fn set_share_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeGroupShareSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().share_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.share_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeNodeGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_size` after provisioning.\nThe initial number of nodes in the node group. One of 'initial_size' or 'autoscaling_policy' must be configured on resource creation."]
    pub fn initial_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nSpecifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT."]
    pub fn maintenance_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_template` after provisioning.\nThe URL of the node template to which this node group belongs."]
    pub fn node_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `size` after provisioning.\nThe total number of nodes in the node group."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone where this node group is located"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::ListRef<ComputeNodeGroupAutoscalingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<ComputeNodeGroupMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> terrars::ListRef<ComputeNodeGroupShareSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeGroupTimeoutsElRef {
        ComputeNodeGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeNodeGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeNodeGroup { }

impl terrars::ToListMappable for ComputeNodeGroup {
    type O = terrars::ListRef<ComputeNodeGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeNodeGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_node_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeNodeGroup {
    pub tf_id: std::string::String,
    #[doc = "The URL of the node template to which this node group belongs."]
    pub node_template: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeNodeGroup {
        let out = ComputeNodeGroup(std::rc::Rc::new(ComputeNodeGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeNodeGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                initial_size: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                name: core::default::Default::default(),
                node_template: self.node_template,
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
                autoscaling_policy: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                share_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeNodeGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeNodeGroupRef {
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `initial_size` after provisioning.\nThe initial number of nodes in the node group. One of 'initial_size' or 'autoscaling_policy' must be configured on resource creation."]
    pub fn initial_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nSpecifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT."]
    pub fn maintenance_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_template` after provisioning.\nThe URL of the node template to which this node group belongs."]
    pub fn node_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `size` after provisioning.\nThe total number of nodes in the node group."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone where this node group is located"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::ListRef<ComputeNodeGroupAutoscalingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<ComputeNodeGroupMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_settings` after provisioning.\n"]
    pub fn share_settings(&self) -> terrars::ListRef<ComputeNodeGroupShareSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeGroupTimeoutsElRef {
        ComputeNodeGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeGroupAutoscalingPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeGroupAutoscalingPolicyEl {
    #[doc =
        "Set the field `max_nodes`.\nMaximum size of the node group. Set to a value less than or equal\nto 100 and greater than or equal to min-nodes."]
    pub fn set_max_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_nodes`.\nMinimum size of the node group. Must be less\nthan or equal to max-nodes. The default value is 0."]
    pub fn set_min_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nThe autoscaling mode. Set to one of the following:\n  - OFF: Disables the autoscaler.\n  - ON: Enables scaling in and scaling out.\n  - ONLY_SCALE_OUT: Enables only scaling out.\n  You must use this mode if your node groups are configured to\n  restart their hosted VMs on minimal servers. Possible values: [\"OFF\", \"ON\", \"ONLY_SCALE_OUT\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNodeGroupAutoscalingPolicyEl {
    type O = terrars::BlockAssignable<ComputeNodeGroupAutoscalingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeGroupAutoscalingPolicyEl {}

impl BuildComputeNodeGroupAutoscalingPolicyEl {
    pub fn build(self) -> ComputeNodeGroupAutoscalingPolicyEl {
        ComputeNodeGroupAutoscalingPolicyEl {
            max_nodes: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct ComputeNodeGroupAutoscalingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeGroupAutoscalingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeGroupAutoscalingPolicyElRef {
        ComputeNodeGroupAutoscalingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeGroupAutoscalingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_nodes` after provisioning.\nMaximum size of the node group. Set to a value less than or equal\nto 100 and greater than or equal to min-nodes."]
    pub fn max_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_nodes` after provisioning.\nMinimum size of the node group. Must be less\nthan or equal to max-nodes. The default value is 0."]
    pub fn min_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe autoscaling mode. Set to one of the following:\n  - OFF: Disables the autoscaler.\n  - ON: Enables scaling in and scaling out.\n  - ONLY_SCALE_OUT: Enables only scaling out.\n  You must use this mode if your node groups are configured to\n  restart their hosted VMs on minimal servers. Possible values: [\"OFF\", \"ON\", \"ONLY_SCALE_OUT\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeGroupMaintenanceWindowEl {
    start_time: terrars::PrimField<std::string::String>,
}

impl ComputeNodeGroupMaintenanceWindowEl { }

impl terrars::ToListMappable for ComputeNodeGroupMaintenanceWindowEl {
    type O = terrars::BlockAssignable<ComputeNodeGroupMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeGroupMaintenanceWindowEl {
    #[doc =
        "instances.start time of the window. This must be in UTC format that resolves to one of 00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example, both 13:00-5 and 08:00 are valid."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeGroupMaintenanceWindowEl {
    pub fn build(self) -> ComputeNodeGroupMaintenanceWindowEl {
        ComputeNodeGroupMaintenanceWindowEl { start_time: self.start_time }
    }
}

pub struct ComputeNodeGroupMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeGroupMaintenanceWindowElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeGroupMaintenanceWindowElRef {
        ComputeNodeGroupMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeGroupMaintenanceWindowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\ninstances.start time of the window. This must be in UTC format that resolves to one of 00:00, 04:00, 08:00, 12:00, 16:00, or 20:00. For example, both 13:00-5 and 08:00 are valid."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeGroupShareSettingsElProjectMapEl {
    id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl ComputeNodeGroupShareSettingsElProjectMapEl { }

impl terrars::ToListMappable for ComputeNodeGroupShareSettingsElProjectMapEl {
    type O = terrars::BlockAssignable<ComputeNodeGroupShareSettingsElProjectMapEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeGroupShareSettingsElProjectMapEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The project id/number should be the same as the key of this project config in the project map."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeGroupShareSettingsElProjectMapEl {
    pub fn build(self) -> ComputeNodeGroupShareSettingsElProjectMapEl {
        ComputeNodeGroupShareSettingsElProjectMapEl {
            id: self.id,
            project_id: self.project_id,
        }
    }
}

pub struct ComputeNodeGroupShareSettingsElProjectMapElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeGroupShareSettingsElProjectMapElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNodeGroupShareSettingsElProjectMapElRef {
        ComputeNodeGroupShareSettingsElProjectMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeGroupShareSettingsElProjectMapElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project id/number should be the same as the key of this project config in the project map."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeNodeGroupShareSettingsElDynamic {
    project_map: std::option::Option<terrars::DynamicBlock<ComputeNodeGroupShareSettingsElProjectMapEl>>,
}

#[derive(Serialize)]
pub struct ComputeNodeGroupShareSettingsEl {
    share_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_map: std::option::Option<std::vec::Vec<ComputeNodeGroupShareSettingsElProjectMapEl>>,
    dynamic: ComputeNodeGroupShareSettingsElDynamic,
}

impl ComputeNodeGroupShareSettingsEl {
    #[doc = "Set the field `project_map`.\n"]
    pub fn set_project_map(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeGroupShareSettingsElProjectMapEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.project_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.project_map = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeNodeGroupShareSettingsEl {
    type O = terrars::BlockAssignable<ComputeNodeGroupShareSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeGroupShareSettingsEl {
    #[doc = "Node group sharing type. Possible values: [\"ORGANIZATION\", \"SPECIFIC_PROJECTS\", \"LOCAL\"]"]
    pub share_type: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeGroupShareSettingsEl {
    pub fn build(self) -> ComputeNodeGroupShareSettingsEl {
        ComputeNodeGroupShareSettingsEl {
            share_type: self.share_type,
            project_map: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeNodeGroupShareSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeGroupShareSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeGroupShareSettingsElRef {
        ComputeNodeGroupShareSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeGroupShareSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `share_type` after provisioning.\nNode group sharing type. Possible values: [\"ORGANIZATION\", \"SPECIFIC_PROJECTS\", \"LOCAL\"]"]
    pub fn share_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeGroupTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeGroupTimeoutsEl {
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

impl terrars::ToListMappable for ComputeNodeGroupTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeNodeGroupTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeGroupTimeoutsEl {}

impl BuildComputeNodeGroupTimeoutsEl {
    pub fn build(self) -> ComputeNodeGroupTimeoutsEl {
        ComputeNodeGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeNodeGroupTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeGroupTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeGroupTimeoutsElRef {
        ComputeNodeGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeGroupTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ComputeNodeGroupDynamic {
    autoscaling_policy: std::option::Option<terrars::DynamicBlock<ComputeNodeGroupAutoscalingPolicyEl>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<ComputeNodeGroupMaintenanceWindowEl>>,
    share_settings: std::option::Option<terrars::DynamicBlock<ComputeNodeGroupShareSettingsEl>>,
}
