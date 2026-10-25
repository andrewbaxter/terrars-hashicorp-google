#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInterconnectAttachmentGroupData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interconnect_group: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachments: std::option::Option<std::vec::Vec<ComputeInterconnectAttachmentGroup_Attachments>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intent: std::option::Option<std::vec::Vec<ComputeInterconnectAttachmentGroup_Intent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInterconnectAttachmentGroup_Timeouts>,
    dynamic: ComputeInterconnectAttachmentGroupDynamic,
}

struct ComputeInterconnectAttachmentGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInterconnectAttachmentGroupData_>,
}

#[derive(Clone)]
pub struct ComputeInterconnectAttachmentGroup(std::rc::Rc<ComputeInterconnectAttachmentGroup_>);

impl ComputeInterconnectAttachmentGroup {
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

    #[doc =
        "Set the field `interconnect_group`.\nThe URL of an InterconnectGroup that groups these Attachments'\nInterconnects. Customers do not need to set this unless directed by\nGoogle Support."]
    pub fn set_interconnect_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().interconnect_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attachments`.\n"]
    pub fn set_attachments(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Attachments>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `intent`.\n"]
    pub fn set_intent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Intent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().intent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.intent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInterconnectAttachmentGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `configured` after provisioning.\nThe redundancy this group is configured to support. The way a\nuser queries what SLA their Attachment gets is by looking at this field of\nthe Attachment's AttachmentGroup."]
    pub fn configured(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_ConfiguredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configured", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interconnect_group` after provisioning.\nThe URL of an InterconnectGroup that groups these Attachments'\nInterconnects. Customers do not need to set this unless directed by\nGoogle Support."]
    pub fn interconnect_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logical_structure` after provisioning.\nAn analysis of the logical layout of Attachments in this\ngroup. Every Attachment in the group is shown once in this structure."]
    pub fn logical_structure(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_LogicalStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logical_structure", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachmentGroup_TimeoutsRef {
        ComputeInterconnectAttachmentGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeInterconnectAttachmentGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeInterconnectAttachmentGroup { }

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup {
    type O = terrars::ListRef<ComputeInterconnectAttachmentGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeInterconnectAttachmentGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_interconnect_attachment_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInterconnectAttachmentGroup {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInterconnectAttachmentGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeInterconnectAttachmentGroup {
        let out = ComputeInterconnectAttachmentGroup(std::rc::Rc::new(ComputeInterconnectAttachmentGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeInterconnectAttachmentGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                interconnect_group: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                attachments: core::default::Default::default(),
                intent: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeInterconnectAttachmentGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInterconnectAttachmentGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `configured` after provisioning.\nThe redundancy this group is configured to support. The way a\nuser queries what SLA their Attachment gets is by looking at this field of\nthe Attachment's AttachmentGroup."]
    pub fn configured(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_ConfiguredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configured", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interconnect_group` after provisioning.\nThe URL of an InterconnectGroup that groups these Attachments'\nInterconnects. Customers do not need to set this unless directed by\nGoogle Support."]
    pub fn interconnect_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interconnect_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logical_structure` after provisioning.\nAn analysis of the logical layout of Attachments in this\ngroup. Every Attachment in the group is shown once in this structure."]
    pub fn logical_structure(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_LogicalStructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logical_structure", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `intent` after provisioning.\n"]
    pub fn intent(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_IntentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectAttachmentGroup_TimeoutsRef {
        ComputeInterconnectAttachmentGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeInterconnectAttachmentGroupDynamic {
    attachments: std::option::Option<terrars::DynamicBlock<ComputeInterconnectAttachmentGroup_Attachments>>,
    intent: std::option::Option<terrars::DynamicBlock<ComputeInterconnectAttachmentGroup_Intent>>,
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocker_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documentation_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explanation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metros: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {
    #[doc = "Set the field `attachments`.\n"]
    pub fn set_attachments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attachments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `blocker_type`.\n"]
    pub fn set_blocker_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.blocker_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documentation_link`.\n"]
    pub fn set_documentation_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.documentation_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `explanation`.\n"]
    pub fn set_explanation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.explanation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metros = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.zones = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {
    type O =
        terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {}

impl BuildComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {
        ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers {
            attachments: core::default::Default::default(),
            blocker_type: core::default::Default::default(),
            documentation_link: core::default::Default::default(),
            explanation: core::default::Default::default(),
            metros: core::default::Default::default(),
            regions: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockersRef {
        ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachments` after provisioning.\n"]
    pub fn attachments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachments", self.base))
    }

    #[doc = "Get a reference to the value of field `blocker_type` after provisioning.\n"]
    pub fn blocker_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blocker_type", self.base))
    }

    #[doc = "Get a reference to the value of field `documentation_link` after provisioning.\n"]
    pub fn documentation_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation_link", self.base))
    }

    #[doc = "Get a reference to the value of field `explanation` after provisioning.\n"]
    pub fn explanation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explanation", self.base))
    }

    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_sla: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    intended_sla_blockers: std
    ::option
    ::Option<terrars::ListField<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers>>,
}

impl ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {
    #[doc = "Set the field `effective_sla`.\n"]
    pub fn set_effective_sla(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effective_sla = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `intended_sla_blockers`.\n"]
    pub fn set_intended_sla_blockers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockers>,
                    >,
    ) -> Self {
        self.intended_sla_blockers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {}

impl BuildComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {
        ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla {
            effective_sla: core::default::Default::default(),
            intended_sla_blockers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_Configured_AvailabilitySlaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_Configured_AvailabilitySlaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_Configured_AvailabilitySlaRef {
        ComputeInterconnectAttachmentGroup_Configured_AvailabilitySlaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_Configured_AvailabilitySlaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_sla` after provisioning.\n"]
    pub fn effective_sla(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_sla", self.base))
    }

    #[doc = "Get a reference to the value of field `intended_sla_blockers` after provisioning.\n"]
    pub fn intended_sla_blockers(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla_IntendedSlaBlockersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.intended_sla_blockers", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_Configured {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_sla: std
    ::option
    ::Option<terrars::ListField<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla>>,
}

impl ComputeInterconnectAttachmentGroup_Configured {
    #[doc = "Set the field `availability_sla`.\n"]
    pub fn set_availability_sla(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySla>>,
    ) -> Self {
        self.availability_sla = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_Configured {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Configured>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_Configured {}

impl BuildComputeInterconnectAttachmentGroup_Configured {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_Configured {
        ComputeInterconnectAttachmentGroup_Configured { availability_sla: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_ConfiguredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_ConfiguredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_ConfiguredRef {
        ComputeInterconnectAttachmentGroup_ConfiguredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_ConfiguredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_sla` after provisioning.\n"]
    pub fn availability_sla(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_Configured_AvailabilitySlaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_sla", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {
    #[doc = "Set the field `attachments`.\n"]
    pub fn set_attachments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attachments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {
    type O =
        terrars
        ::BlockAssignable<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {}

impl BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones {
            attachments: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_ZonesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_ZonesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_ZonesRef {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_ZonesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_ZonesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachments` after provisioning.\n"]
    pub fn attachments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachments", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std
    ::option
    ::Option<
        terrars::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones>,
    >,
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {
    #[doc = "Set the field `facility`.\n"]
    pub fn set_facility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.facility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zones`.\n"]
    pub fn set_zones(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_Zones,
                        >,
                    >,
    ) -> Self {
        self.zones = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {}

impl BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities {
            facility: core::default::Default::default(),
            zones: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_FacilitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_FacilitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_FacilitiesRef {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_FacilitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_FacilitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `facility` after provisioning.\n"]
    pub fn facility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.facility", self.base))
    }

    #[doc = "Get a reference to the value of field `zones` after provisioning.\n"]
    pub fn zones(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities_ZonesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facilities: std
    ::option
    ::Option<terrars::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metro: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {
    #[doc = "Set the field `facilities`.\n"]
    pub fn set_facilities(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_Facilities>,
                    >,
    ) -> Self {
        self.facilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metro`.\n"]
    pub fn set_metro(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metro = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {}

impl BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros {
            facilities: core::default::Default::default(),
            metro: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_MetrosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_MetrosRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_MetrosRef {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_MetrosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_MetrosRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `facilities` after provisioning.\n"]
    pub fn facilities(
        &self,
    ) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros_FacilitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.facilities", self.base))
    }

    #[doc = "Get a reference to the value of field `metro` after provisioning.\n"]
    pub fn metro(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metro", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_Regions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metros: std::option::Option<terrars::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_Regions {
    #[doc = "Set the field `metros`.\n"]
    pub fn set_metros(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_Metros>>,
    ) -> Self {
        self.metros = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_LogicalStructure_Regions {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions {}

impl BuildComputeInterconnectAttachmentGroup_LogicalStructure_Regions {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_LogicalStructure_Regions {
        ComputeInterconnectAttachmentGroup_LogicalStructure_Regions {
            metros: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure_RegionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_LogicalStructure_RegionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_LogicalStructure_RegionsRef {
        ComputeInterconnectAttachmentGroup_LogicalStructure_RegionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure_RegionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metros` after provisioning.\n"]
    pub fn metros(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions_MetrosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metros", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions>>,
}

impl ComputeInterconnectAttachmentGroup_LogicalStructure {
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInterconnectAttachmentGroup_LogicalStructure_Regions>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_LogicalStructure {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_LogicalStructure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_LogicalStructure {}

impl BuildComputeInterconnectAttachmentGroup_LogicalStructure {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_LogicalStructure {
        ComputeInterconnectAttachmentGroup_LogicalStructure { regions: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_LogicalStructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_LogicalStructureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_LogicalStructureRef {
        ComputeInterconnectAttachmentGroup_LogicalStructureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_LogicalStructureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<ComputeInterconnectAttachmentGroup_LogicalStructure_RegionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_Attachments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl ComputeInterconnectAttachmentGroup_Attachments {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_Attachments {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Attachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_Attachments {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInterconnectAttachmentGroup_Attachments {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_Attachments {
        ComputeInterconnectAttachmentGroup_Attachments {
            attachment: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_AttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_AttachmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_AttachmentsRef {
        ComputeInterconnectAttachmentGroup_AttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_AttachmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_Intent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_sla: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectAttachmentGroup_Intent {
    #[doc =
        "Set the field `availability_sla`.\nWhich SLA the user intends this group to support. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn set_availability_sla(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_sla = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_Intent {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Intent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_Intent {}

impl BuildComputeInterconnectAttachmentGroup_Intent {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_Intent {
        ComputeInterconnectAttachmentGroup_Intent { availability_sla: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_IntentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_IntentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInterconnectAttachmentGroup_IntentRef {
        ComputeInterconnectAttachmentGroup_IntentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_IntentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `availability_sla` after provisioning.\nWhich SLA the user intends this group to support. Possible values: [\"PRODUCTION_NON_CRITICAL\", \"PRODUCTION_CRITICAL\", \"NO_SLA\", \"AVAILABILITY_SLA_UNSPECIFIED\"]"]
    pub fn availability_sla(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_sla", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInterconnectAttachmentGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInterconnectAttachmentGroup_Timeouts {
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

impl terrars::ToListMappable for ComputeInterconnectAttachmentGroup_Timeouts {
    type O = terrars::BlockAssignable<ComputeInterconnectAttachmentGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInterconnectAttachmentGroup_Timeouts {}

impl BuildComputeInterconnectAttachmentGroup_Timeouts {
    pub fn build(self) -> ComputeInterconnectAttachmentGroup_Timeouts {
        ComputeInterconnectAttachmentGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInterconnectAttachmentGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInterconnectAttachmentGroup_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInterconnectAttachmentGroup_TimeoutsRef {
        ComputeInterconnectAttachmentGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInterconnectAttachmentGroup_TimeoutsRef {
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
