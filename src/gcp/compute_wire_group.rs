#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeWireGroupData_ {
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
    admin_enabled: std::option::Option<terrars::PrimField<bool>>,
    cross_site_network: terrars::PrimField<std::string::String>,
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
    endpoints: std::option::Option<std::vec::Vec<ComputeWireGroup_Endpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeWireGroup_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wire_properties: std::option::Option<std::vec::Vec<ComputeWireGroup_WireProperties>>,
    dynamic: ComputeWireGroupDynamic,
}

struct ComputeWireGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeWireGroupData_>,
}

#[derive(Clone)]
pub struct ComputeWireGroup(std::rc::Rc<ComputeWireGroup_>);

impl ComputeWireGroup {
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

    #[doc = "Set the field `admin_enabled`.\nIndicates whether the wire group is administratively enabled."]
    pub fn set_admin_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeWireGroup_Endpoints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeWireGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wire_properties`.\n"]
    pub fn set_wire_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeWireGroup_WireProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().wire_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.wire_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `admin_enabled` after provisioning.\nIndicates whether the wire group is administratively enabled."]
    pub fn admin_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cross_site_network` after provisioning.\nRequired cross site network to which wire group belongs."]
    pub fn cross_site_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_site_network", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `topology` after provisioning.\nTopology details for the wire group configuration."]
    pub fn topology(&self) -> terrars::ListRef<ComputeWireGroup_TopologyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topology", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wires` after provisioning.\nThe single/redundant wire(s) managed by the wire group."]
    pub fn wires(&self) -> terrars::ListRef<ComputeWireGroup_WiresRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wires", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeWireGroup_TimeoutsRef {
        ComputeWireGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wire_properties` after provisioning.\n"]
    pub fn wire_properties(&self) -> terrars::ListRef<ComputeWireGroup_WirePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wire_properties", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeWireGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeWireGroup { }

impl terrars::ToListMappable for ComputeWireGroup {
    type O = terrars::ListRef<ComputeWireGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeWireGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_wire_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeWireGroup {
    pub tf_id: std::string::String,
    #[doc = "Required cross site network to which wire group belongs."]
    pub cross_site_network: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeWireGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeWireGroup {
        let out = ComputeWireGroup(std::rc::Rc::new(ComputeWireGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeWireGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_enabled: core::default::Default::default(),
                cross_site_network: self.cross_site_network,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                endpoints: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                wire_properties: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeWireGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeWireGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_enabled` after provisioning.\nIndicates whether the wire group is administratively enabled."]
    pub fn admin_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cross_site_network` after provisioning.\nRequired cross site network to which wire group belongs."]
    pub fn cross_site_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_site_network", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `topology` after provisioning.\nTopology details for the wire group configuration."]
    pub fn topology(&self) -> terrars::ListRef<ComputeWireGroup_TopologyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topology", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wires` after provisioning.\nThe single/redundant wire(s) managed by the wire group."]
    pub fn wires(&self) -> terrars::ListRef<ComputeWireGroup_WiresRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wires", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeWireGroup_TimeoutsRef {
        ComputeWireGroup_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wire_properties` after provisioning.\n"]
    pub fn wire_properties(&self) -> terrars::ListRef<ComputeWireGroup_WirePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wire_properties", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeWireGroupDynamic {
    endpoints: std::option::Option<terrars::DynamicBlock<ComputeWireGroup_Endpoints>>,
    wire_properties: std::option::Option<terrars::DynamicBlock<ComputeWireGroup_WireProperties>>,
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Topology_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeWireGroup_Topology_Endpoints {
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.city = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Topology_Endpoints {
    type O = terrars::BlockAssignable<ComputeWireGroup_Topology_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Topology_Endpoints {}

impl BuildComputeWireGroup_Topology_Endpoints {
    pub fn build(self) -> ComputeWireGroup_Topology_Endpoints {
        ComputeWireGroup_Topology_Endpoints {
            city: core::default::Default::default(),
            label: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_Topology_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_Topology_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_Topology_EndpointsRef {
        ComputeWireGroup_Topology_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_Topology_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Topology {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<ComputeWireGroup_Topology_Endpoints>>,
}

impl ComputeWireGroup_Topology {
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeWireGroup_Topology_Endpoints>>,
    ) -> Self {
        self.endpoints = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Topology {
    type O = terrars::BlockAssignable<ComputeWireGroup_Topology>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Topology {}

impl BuildComputeWireGroup_Topology {
    pub fn build(self) -> ComputeWireGroup_Topology {
        ComputeWireGroup_Topology { endpoints: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_TopologyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_TopologyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_TopologyRef {
        ComputeWireGroup_TopologyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_TopologyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<ComputeWireGroup_Topology_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Wires_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan_tag: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeWireGroup_Wires_Endpoints {
    #[doc = "Set the field `interconnect`.\n"]
    pub fn set_interconnect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interconnect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vlan_tag`.\n"]
    pub fn set_vlan_tag(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vlan_tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Wires_Endpoints {
    type O = terrars::BlockAssignable<ComputeWireGroup_Wires_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Wires_Endpoints {}

impl BuildComputeWireGroup_Wires_Endpoints {
    pub fn build(self) -> ComputeWireGroup_Wires_Endpoints {
        ComputeWireGroup_Wires_Endpoints {
            interconnect: core::default::Default::default(),
            vlan_tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_Wires_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_Wires_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_Wires_EndpointsRef {
        ComputeWireGroup_Wires_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_Wires_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interconnect` after provisioning.\n"]
    pub fn interconnect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.base))
    }

    #[doc = "Get a reference to the value of field `vlan_tag` after provisioning.\n"]
    pub fn vlan_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan_tag", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Wires_WireProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bandwidth_unmetered: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeWireGroup_Wires_WireProperties {
    #[doc = "Set the field `bandwidth_unmetered`.\n"]
    pub fn set_bandwidth_unmetered(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bandwidth_unmetered = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fault_response`.\n"]
    pub fn set_fault_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fault_response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Wires_WireProperties {
    type O = terrars::BlockAssignable<ComputeWireGroup_Wires_WireProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Wires_WireProperties {}

impl BuildComputeWireGroup_Wires_WireProperties {
    pub fn build(self) -> ComputeWireGroup_Wires_WireProperties {
        ComputeWireGroup_Wires_WireProperties {
            bandwidth_unmetered: core::default::Default::default(),
            fault_response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_Wires_WirePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_Wires_WirePropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_Wires_WirePropertiesRef {
        ComputeWireGroup_Wires_WirePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_Wires_WirePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bandwidth_unmetered` after provisioning.\n"]
    pub fn bandwidth_unmetered(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_unmetered", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_response` after provisioning.\n"]
    pub fn fault_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fault_response", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Wires {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<ComputeWireGroup_Wires_Endpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wire_properties: std::option::Option<terrars::ListField<ComputeWireGroup_Wires_WireProperties>>,
}

impl ComputeWireGroup_Wires {
    #[doc = "Set the field `admin_enabled`.\n"]
    pub fn set_admin_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.admin_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeWireGroup_Wires_Endpoints>>,
    ) -> Self {
        self.endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wire_properties`.\n"]
    pub fn set_wire_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeWireGroup_Wires_WireProperties>>,
    ) -> Self {
        self.wire_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Wires {
    type O = terrars::BlockAssignable<ComputeWireGroup_Wires>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Wires {}

impl BuildComputeWireGroup_Wires {
    pub fn build(self) -> ComputeWireGroup_Wires {
        ComputeWireGroup_Wires {
            admin_enabled: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            label: core::default::Default::default(),
            wire_properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_WiresRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_WiresRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_WiresRef {
        ComputeWireGroup_WiresRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_WiresRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `admin_enabled` after provisioning.\n"]
    pub fn admin_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<ComputeWireGroup_Wires_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `wire_properties` after provisioning.\n"]
    pub fn wire_properties(&self) -> terrars::ListRef<ComputeWireGroup_Wires_WirePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.wire_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Endpoints_Interconnects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnect: std::option::Option<terrars::PrimField<std::string::String>>,
    interconnect_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan_tags: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl ComputeWireGroup_Endpoints_Interconnects {
    #[doc = "Set the field `interconnect`.\n"]
    pub fn set_interconnect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interconnect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vlan_tags`.\nVLAN tags for the interconnect."]
    pub fn set_vlan_tags(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.vlan_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Endpoints_Interconnects {
    type O = terrars::BlockAssignable<ComputeWireGroup_Endpoints_Interconnects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Endpoints_Interconnects {
    #[doc = ""]
    pub interconnect_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeWireGroup_Endpoints_Interconnects {
    pub fn build(self) -> ComputeWireGroup_Endpoints_Interconnects {
        ComputeWireGroup_Endpoints_Interconnects {
            interconnect: core::default::Default::default(),
            interconnect_name: self.interconnect_name,
            vlan_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_Endpoints_InterconnectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_Endpoints_InterconnectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_Endpoints_InterconnectsRef {
        ComputeWireGroup_Endpoints_InterconnectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_Endpoints_InterconnectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interconnect` after provisioning.\n"]
    pub fn interconnect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect", self.base))
    }

    #[doc = "Get a reference to the value of field `interconnect_name` after provisioning.\n"]
    pub fn interconnect_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vlan_tags` after provisioning.\nVLAN tags for the interconnect."]
    pub fn vlan_tags(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vlan_tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeWireGroup_EndpointsDynamic {
    interconnects: std::option::Option<terrars::DynamicBlock<ComputeWireGroup_Endpoints_Interconnects>>,
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Endpoints {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnects: std::option::Option<std::vec::Vec<ComputeWireGroup_Endpoints_Interconnects>>,
    dynamic: ComputeWireGroup_EndpointsDynamic,
}

impl ComputeWireGroup_Endpoints {
    #[doc = "Set the field `interconnects`.\n"]
    pub fn set_interconnects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeWireGroup_Endpoints_Interconnects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.interconnects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.interconnects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_Endpoints {
    type O = terrars::BlockAssignable<ComputeWireGroup_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Endpoints {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildComputeWireGroup_Endpoints {
    pub fn build(self) -> ComputeWireGroup_Endpoints {
        ComputeWireGroup_Endpoints {
            endpoint: self.endpoint,
            interconnects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_EndpointsRef {
        ComputeWireGroup_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeWireGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeWireGroup_Timeouts {
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

impl terrars::ToListMappable for ComputeWireGroup_Timeouts {
    type O = terrars::BlockAssignable<ComputeWireGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_Timeouts {}

impl BuildComputeWireGroup_Timeouts {
    pub fn build(self) -> ComputeWireGroup_Timeouts {
        ComputeWireGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_TimeoutsRef {
        ComputeWireGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_TimeoutsRef {
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
pub struct ComputeWireGroup_WireProperties {
    bandwidth_allocation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bandwidth_unmetered: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeWireGroup_WireProperties {
    #[doc = "Set the field `bandwidth_unmetered`.\nThe unmetered bandwidth setting."]
    pub fn set_bandwidth_unmetered(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bandwidth_unmetered = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fault_response`.\nResponse when a fault is detected in a pseudowire:\nNONE: default.\nDISABLE_PORT: set the port line protocol down when inline probes detect a fault. This setting is only permitted on port mode pseudowires."]
    pub fn set_fault_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fault_response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeWireGroup_WireProperties {
    type O = terrars::BlockAssignable<ComputeWireGroup_WireProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeWireGroup_WireProperties {
    #[doc =
        "The configuration of a wire's bandwidth allocation.\nALLOCATE_PER_WIRE: configures a separate unmetered bandwidth allocation (and associated charges) for each wire in the group.\nSHARED_WITH_WIRE_GROUP: this is the default behavior, which configures one unmetered bandwidth allocation for the wire group. The unmetered bandwidth is divided equally across each wire in the group, but dynamic\nthrottling reallocates unused unmetered bandwidth from unused or underused wires to other wires in the group."]
    pub bandwidth_allocation: terrars::PrimField<std::string::String>,
}

impl BuildComputeWireGroup_WireProperties {
    pub fn build(self) -> ComputeWireGroup_WireProperties {
        ComputeWireGroup_WireProperties {
            bandwidth_allocation: self.bandwidth_allocation,
            bandwidth_unmetered: core::default::Default::default(),
            fault_response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeWireGroup_WirePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeWireGroup_WirePropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeWireGroup_WirePropertiesRef {
        ComputeWireGroup_WirePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeWireGroup_WirePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bandwidth_allocation` after provisioning.\nThe configuration of a wire's bandwidth allocation.\nALLOCATE_PER_WIRE: configures a separate unmetered bandwidth allocation (and associated charges) for each wire in the group.\nSHARED_WITH_WIRE_GROUP: this is the default behavior, which configures one unmetered bandwidth allocation for the wire group. The unmetered bandwidth is divided equally across each wire in the group, but dynamic\nthrottling reallocates unused unmetered bandwidth from unused or underused wires to other wires in the group."]
    pub fn bandwidth_allocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_allocation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bandwidth_unmetered` after provisioning.\nThe unmetered bandwidth setting."]
    pub fn bandwidth_unmetered(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_unmetered", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fault_response` after provisioning.\nResponse when a fault is detected in a pseudowire:\nNONE: default.\nDISABLE_PORT: set the port line protocol down when inline probes detect a fault. This setting is only permitted on port mode pseudowires."]
    pub fn fault_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fault_response", self.base))
    }
}
