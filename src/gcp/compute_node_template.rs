use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNodeTemplateData {
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
    cpu_overcommit_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_flexibility: std::option::Option<std::vec::Vec<ComputeNodeTemplateNodeTypeFlexibilityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_binding: std::option::Option<std::vec::Vec<ComputeNodeTemplateServerBindingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeNodeTemplateTimeoutsEl>,
    dynamic: ComputeNodeTemplateDynamic,
}

struct ComputeNodeTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeNodeTemplateData>,
}

#[derive(Clone)]
pub struct ComputeNodeTemplate(std::rc::Rc<ComputeNodeTemplate_>);

impl ComputeNodeTemplate {
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
        "Set the field `cpu_overcommit_type`.\nCPU overcommit. Default value: \"NONE\" Possible values: [\"ENABLED\", \"NONE\"]"]
    pub fn set_cpu_overcommit_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpu_overcommit_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `name`.\nName of the resource."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_affinity_labels`.\nLabels to use for node affinity, which will be used in\ninstance scheduling."]
    pub fn set_node_affinity_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().node_affinity_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_type`.\nNode type to use for nodes group that are created from this template.\nOnly one of nodeTypeFlexibility and nodeType can be specified."]
    pub fn set_node_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where nodes using the node template will be created.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type_flexibility`.\n"]
    pub fn set_node_type_flexibility(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeTemplateNodeTypeFlexibilityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_type_flexibility = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_type_flexibility = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_binding`.\n"]
    pub fn set_server_binding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeTemplateServerBindingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeNodeTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cpu_overcommit_type` after provisioning.\nCPU overcommit. Default value: \"NONE\" Possible values: [\"ENABLED\", \"NONE\"]"]
    pub fn cpu_overcommit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_overcommit_type", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_affinity_labels` after provisioning.\nLabels to use for node affinity, which will be used in\ninstance scheduling."]
    pub fn node_affinity_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.node_affinity_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nNode type to use for nodes group that are created from this template.\nOnly one of nodeTypeFlexibility and nodeType can be specified."]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where nodes using the node template will be created.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type_flexibility` after provisioning.\n"]
    pub fn node_type_flexibility(&self) -> terrars::ListRef<ComputeNodeTemplateNodeTypeFlexibilityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_type_flexibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_binding` after provisioning.\n"]
    pub fn server_binding(&self) -> terrars::ListRef<ComputeNodeTemplateServerBindingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_binding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeTemplateTimeoutsElRef {
        ComputeNodeTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeNodeTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeNodeTemplate { }

impl terrars::ToListMappable for ComputeNodeTemplate {
    type O = terrars::ListRef<ComputeNodeTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeNodeTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_node_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeNodeTemplate {
    pub tf_id: std::string::String,
}

impl BuildComputeNodeTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeNodeTemplate {
        let out = ComputeNodeTemplate(std::rc::Rc::new(ComputeNodeTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeNodeTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cpu_overcommit_type: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                node_affinity_labels: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                node_type_flexibility: core::default::Default::default(),
                server_binding: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeNodeTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeNodeTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_overcommit_type` after provisioning.\nCPU overcommit. Default value: \"NONE\" Possible values: [\"ENABLED\", \"NONE\"]"]
    pub fn cpu_overcommit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_overcommit_type", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_affinity_labels` after provisioning.\nLabels to use for node affinity, which will be used in\ninstance scheduling."]
    pub fn node_affinity_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.node_affinity_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_type` after provisioning.\nNode type to use for nodes group that are created from this template.\nOnly one of nodeTypeFlexibility and nodeType can be specified."]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where nodes using the node template will be created.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type_flexibility` after provisioning.\n"]
    pub fn node_type_flexibility(&self) -> terrars::ListRef<ComputeNodeTemplateNodeTypeFlexibilityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_type_flexibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_binding` after provisioning.\n"]
    pub fn server_binding(&self) -> terrars::ListRef<ComputeNodeTemplateServerBindingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_binding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeTemplateTimeoutsElRef {
        ComputeNodeTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeTemplateNodeTypeFlexibilityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeTemplateNodeTypeFlexibilityEl {
    #[doc = "Set the field `cpus`.\nNumber of virtual CPUs to use."]
    pub fn set_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nPhysical memory available to the node, defined in MB."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNodeTemplateNodeTypeFlexibilityEl {
    type O = terrars::BlockAssignable<ComputeNodeTemplateNodeTypeFlexibilityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplateNodeTypeFlexibilityEl {}

impl BuildComputeNodeTemplateNodeTypeFlexibilityEl {
    pub fn build(self) -> ComputeNodeTemplateNodeTypeFlexibilityEl {
        ComputeNodeTemplateNodeTypeFlexibilityEl {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

pub struct ComputeNodeTemplateNodeTypeFlexibilityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplateNodeTypeFlexibilityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplateNodeTypeFlexibilityElRef {
        ComputeNodeTemplateNodeTypeFlexibilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplateNodeTypeFlexibilityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpus` after provisioning.\nNumber of virtual CPUs to use."]
    pub fn cpus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd` after provisioning.\nUse local SSD"]
    pub fn local_ssd(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_ssd", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nPhysical memory available to the node, defined in MB."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeTemplateServerBindingEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeNodeTemplateServerBindingEl { }

impl terrars::ToListMappable for ComputeNodeTemplateServerBindingEl {
    type O = terrars::BlockAssignable<ComputeNodeTemplateServerBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplateServerBindingEl {
    #[doc =
        "Type of server binding policy. If 'RESTART_NODE_ON_ANY_SERVER',\nnodes using this template will restart on any physical server\nfollowing a maintenance event.\n\nIf 'RESTART_NODE_ON_MINIMAL_SERVER', nodes using this template\nwill restart on the same physical server following a maintenance\nevent, instead of being live migrated to or restarted on a new\nphysical server. This option may be useful if you are using\nsoftware licenses tied to the underlying server characteristics\nsuch as physical sockets or cores, to avoid the need for\nadditional licenses when maintenance occurs. However, VMs on such\nnodes will experience outages while maintenance is applied. Possible values: [\"RESTART_NODE_ON_ANY_SERVER\", \"RESTART_NODE_ON_MINIMAL_SERVERS\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeTemplateServerBindingEl {
    pub fn build(self) -> ComputeNodeTemplateServerBindingEl {
        ComputeNodeTemplateServerBindingEl { type_: self.type_ }
    }
}

pub struct ComputeNodeTemplateServerBindingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplateServerBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplateServerBindingElRef {
        ComputeNodeTemplateServerBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplateServerBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of server binding policy. If 'RESTART_NODE_ON_ANY_SERVER',\nnodes using this template will restart on any physical server\nfollowing a maintenance event.\n\nIf 'RESTART_NODE_ON_MINIMAL_SERVER', nodes using this template\nwill restart on the same physical server following a maintenance\nevent, instead of being live migrated to or restarted on a new\nphysical server. This option may be useful if you are using\nsoftware licenses tied to the underlying server characteristics\nsuch as physical sockets or cores, to avoid the need for\nadditional licenses when maintenance occurs. However, VMs on such\nnodes will experience outages while maintenance is applied. Possible values: [\"RESTART_NODE_ON_ANY_SERVER\", \"RESTART_NODE_ON_MINIMAL_SERVERS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeTemplateTimeoutsEl {
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
}

impl terrars::ToListMappable for ComputeNodeTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeNodeTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplateTimeoutsEl {}

impl BuildComputeNodeTemplateTimeoutsEl {
    pub fn build(self) -> ComputeNodeTemplateTimeoutsEl {
        ComputeNodeTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeNodeTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplateTimeoutsElRef {
        ComputeNodeTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplateTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct ComputeNodeTemplateDynamic {
    node_type_flexibility: std::option::Option<terrars::DynamicBlock<ComputeNodeTemplateNodeTypeFlexibilityEl>>,
    server_binding: std::option::Option<terrars::DynamicBlock<ComputeNodeTemplateServerBindingEl>>,
}
