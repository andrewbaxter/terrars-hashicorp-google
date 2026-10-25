#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNodeTemplateData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinity_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<ComputeNodeTemplate_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disks: std::option::Option<std::vec::Vec<ComputeNodeTemplate_Disks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type_flexibility: std::option::Option<std::vec::Vec<ComputeNodeTemplate_NodeTypeFlexibility>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_binding: std::option::Option<std::vec::Vec<ComputeNodeTemplate_ServerBinding>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeNodeTemplate_Timeouts>,
    dynamic: ComputeNodeTemplateDynamic,
}

struct ComputeNodeTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeNodeTemplateData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeTemplate_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disks`.\n"]
    pub fn set_disks(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeTemplate_Disks>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_type_flexibility`.\n"]
    pub fn set_node_type_flexibility(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeTemplate_NodeTypeFlexibility>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNodeTemplate_ServerBinding>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeNodeTemplate_Timeouts>) -> Self {
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<ComputeNodeTemplate_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(&self) -> terrars::ListRef<ComputeNodeTemplate_DisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type_flexibility` after provisioning.\n"]
    pub fn node_type_flexibility(&self) -> terrars::ListRef<ComputeNodeTemplate_NodeTypeFlexibilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_type_flexibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_binding` after provisioning.\n"]
    pub fn server_binding(&self) -> terrars::ListRef<ComputeNodeTemplate_ServerBindingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_binding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeTemplate_TimeoutsRef {
        ComputeNodeTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
    #[doc = "Name of the resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeNodeTemplate {
        let out = ComputeNodeTemplate(std::rc::Rc::new(ComputeNodeTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeNodeTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cpu_overcommit_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                node_affinity_labels: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                accelerators: core::default::Default::default(),
                disks: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<ComputeNodeTemplate_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(&self) -> terrars::ListRef<ComputeNodeTemplate_DisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type_flexibility` after provisioning.\n"]
    pub fn node_type_flexibility(&self) -> terrars::ListRef<ComputeNodeTemplate_NodeTypeFlexibilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_type_flexibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_binding` after provisioning.\n"]
    pub fn server_binding(&self) -> terrars::ListRef<ComputeNodeTemplate_ServerBindingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_binding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeTemplate_TimeoutsRef {
        ComputeNodeTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeNodeTemplateDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<ComputeNodeTemplate_Accelerators>>,
    disks: std::option::Option<terrars::DynamicBlock<ComputeNodeTemplate_Disks>>,
    node_type_flexibility: std::option::Option<terrars::DynamicBlock<ComputeNodeTemplate_NodeTypeFlexibility>>,
    server_binding: std::option::Option<terrars::DynamicBlock<ComputeNodeTemplate_ServerBinding>>,
}

#[derive(Serialize)]
pub struct ComputeNodeTemplate_Accelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeTemplate_Accelerators {
    #[doc =
        "Set the field `accelerator_count`.\nThe number of the guest accelerator cards exposed to this\nnode template."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nFull or partial URL of the accelerator type resource to expose\nto this node template."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNodeTemplate_Accelerators {
    type O = terrars::BlockAssignable<ComputeNodeTemplate_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplate_Accelerators {}

impl BuildComputeNodeTemplate_Accelerators {
    pub fn build(self) -> ComputeNodeTemplate_Accelerators {
        ComputeNodeTemplate_Accelerators {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNodeTemplate_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplate_AcceleratorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplate_AcceleratorsRef {
        ComputeNodeTemplate_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplate_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the guest accelerator cards exposed to this\nnode template."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nFull or partial URL of the accelerator type resource to expose\nto this node template."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeTemplate_Disks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeTemplate_Disks {
    #[doc = "Set the field `disk_count`.\nSpecifies the number of such disks."]
    pub fn set_disk_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size_gb`.\nSpecifies the size of the disk in base-2 GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nSpecifies the desired disk type on the node. This disk type must be a local storage type (e.g.: local-ssd). Note that for nodeTemplates, this should be the name of the disk type and not its URL."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNodeTemplate_Disks {
    type O = terrars::BlockAssignable<ComputeNodeTemplate_Disks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplate_Disks {}

impl BuildComputeNodeTemplate_Disks {
    pub fn build(self) -> ComputeNodeTemplate_Disks {
        ComputeNodeTemplate_Disks {
            disk_count: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNodeTemplate_DisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplate_DisksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplate_DisksRef {
        ComputeNodeTemplate_DisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplate_DisksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_count` after provisioning.\nSpecifies the number of such disks."]
    pub fn disk_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSpecifies the size of the disk in base-2 GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nSpecifies the desired disk type on the node. This disk type must be a local storage type (e.g.: local-ssd). Note that for nodeTemplates, this should be the name of the disk type and not its URL."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNodeTemplate_NodeTypeFlexibility {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeTemplate_NodeTypeFlexibility {
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

impl terrars::ToListMappable for ComputeNodeTemplate_NodeTypeFlexibility {
    type O = terrars::BlockAssignable<ComputeNodeTemplate_NodeTypeFlexibility>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplate_NodeTypeFlexibility {}

impl BuildComputeNodeTemplate_NodeTypeFlexibility {
    pub fn build(self) -> ComputeNodeTemplate_NodeTypeFlexibility {
        ComputeNodeTemplate_NodeTypeFlexibility {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNodeTemplate_NodeTypeFlexibilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplate_NodeTypeFlexibilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplate_NodeTypeFlexibilityRef {
        ComputeNodeTemplate_NodeTypeFlexibilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplate_NodeTypeFlexibilityRef {
    #[allow(dead_code)]
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
pub struct ComputeNodeTemplate_ServerBinding {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeNodeTemplate_ServerBinding { }

impl terrars::ToListMappable for ComputeNodeTemplate_ServerBinding {
    type O = terrars::BlockAssignable<ComputeNodeTemplate_ServerBinding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplate_ServerBinding {
    #[doc =
        "Type of server binding policy. If 'RESTART_NODE_ON_ANY_SERVER',\nnodes using this template will restart on any physical server\nfollowing a maintenance event.\n\nIf 'RESTART_NODE_ON_MINIMAL_SERVER', nodes using this template\nwill restart on the same physical server following a maintenance\nevent, instead of being live migrated to or restarted on a new\nphysical server. This option may be useful if you are using\nsoftware licenses tied to the underlying server characteristics\nsuch as physical sockets or cores, to avoid the need for\nadditional licenses when maintenance occurs. However, VMs on such\nnodes will experience outages while maintenance is applied. Possible values: [\"RESTART_NODE_ON_ANY_SERVER\", \"RESTART_NODE_ON_MINIMAL_SERVERS\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeNodeTemplate_ServerBinding {
    pub fn build(self) -> ComputeNodeTemplate_ServerBinding {
        ComputeNodeTemplate_ServerBinding { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ComputeNodeTemplate_ServerBindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplate_ServerBindingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplate_ServerBindingRef {
        ComputeNodeTemplate_ServerBindingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplate_ServerBindingRef {
    #[allow(dead_code)]
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
pub struct ComputeNodeTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNodeTemplate_Timeouts {
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

impl terrars::ToListMappable for ComputeNodeTemplate_Timeouts {
    type O = terrars::BlockAssignable<ComputeNodeTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNodeTemplate_Timeouts {}

impl BuildComputeNodeTemplate_Timeouts {
    pub fn build(self) -> ComputeNodeTemplate_Timeouts {
        ComputeNodeTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNodeTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNodeTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNodeTemplate_TimeoutsRef {
        ComputeNodeTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNodeTemplate_TimeoutsRef {
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
}
